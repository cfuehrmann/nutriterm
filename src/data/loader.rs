use crate::error::LoadError;
use crate::models::{Ingredient, Recipe, WeightedIngredient};
use crate::schema::generator::{generate_ingredient_schema, generate_recipe_schema};
use jsonschema::Validator;
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

#[derive(Deserialize)]
struct JsonRecipes {
    recipes: Vec<JsonRecipe>,
}

#[derive(Deserialize)]
struct JsonRecipe {
    name: String,
    description: Option<String>,
    ingredients: Vec<JsonRecipeIngredient>,
}

#[derive(Deserialize)]
struct JsonRecipeIngredient {
    ingredient_id: String,
    grams: f64,
}

#[derive(Deserialize)]
struct JsonIngredients {
    ingredients: Vec<JsonIngredient>,
}

#[derive(Deserialize)]
struct JsonIngredient {
    id: String,
    name: String,
    carbs_per_100g: f64,
    protein_per_100g: f64,
    fat_per_100g: f64,
    fiber_per_100g: f64,
}

fn validate_with_schema(
    json_value: &Value,
    schema: &Validator,
    filename: &str,
) -> Result<(), LoadError> {
    // Use iter_errors to get all validation errors at once
    let validation_errors: Vec<_> = schema.iter_errors(json_value).collect();

    if !validation_errors.is_empty() {
        let error_messages: Vec<String> = validation_errors
            .into_iter()
            .map(|error| format!("- {}: {}", error.instance_path, error))
            .collect();

        return Err(LoadError::SchemaValidationError {
            filename: filename.to_string(),
            errors: error_messages,
        });
    }

    Ok(())
}

fn load_jsonc_file<T: DeserializeOwned>(
    data_dir: &Path,
    filename: &str,
    schema_generator: fn() -> Value,
) -> Result<T, LoadError> {
    let file_path = data_dir.join(filename);
    let content = std::fs::read_to_string(&file_path).map_err(|e| LoadError::FileError {
        path: file_path.clone(),
        source: e,
    })?;

    let json_value = jsonc_parser::parse_to_serde_value(&content, &Default::default())
        .map_err(|e| LoadError::ParseError {
            filename: filename.to_string(),
            message: format!("{}", e),
        })?
        .ok_or_else(|| LoadError::ParseError {
            filename: filename.to_string(),
            message: "Empty file".to_string(),
        })?;

    let schema_json = schema_generator();
    let schema = Validator::new(&schema_json).map_err(|e| LoadError::ValidationError {
        filename: filename.to_string(),
        message: format!("Failed to compile schema: {}", e),
    })?;

    validate_with_schema(&json_value, &schema, filename)?;

    serde_json::from_value(json_value).map_err(|e| LoadError::ValidationError {
        filename: filename.to_string(),
        message: format!("{}", e),
    })
}

pub fn load_recipes(data_dir: &Path) -> Result<Vec<Recipe>, LoadError> {
    let json_recipes: JsonRecipes =
        load_jsonc_file(data_dir, "recipes.jsonc", generate_recipe_schema)?;

    let json_ingredients = load_json_ingredients(data_dir)?;
    let ingredient_map: HashMap<String, Ingredient> = json_ingredients
        .ingredients
        .into_iter()
        .map(|json_ing| {
            (
                json_ing.id,
                Ingredient {
                    name: json_ing.name,
                    carbs_per_100g: json_ing.carbs_per_100g,
                    protein_per_100g: json_ing.protein_per_100g,
                    fat_per_100g: json_ing.fat_per_100g,
                    fiber_per_100g: json_ing.fiber_per_100g,
                },
            )
        })
        .collect();

    let mut recipes = Vec::new();

    for json_recipe in json_recipes.recipes {
        let mut recipe_ingredients = Vec::new();

        for json_ingredient in json_recipe.ingredients {
            let ingredient = ingredient_map.get(&json_ingredient.ingredient_id).ok_or_else(|| {
                LoadError::UnknownIngredientError {
                    recipe: json_recipe.name.clone(),
                    ingredient: json_ingredient.ingredient_id.clone(),
                }
            })?;

            recipe_ingredients.push(WeightedIngredient {
                ingredient: ingredient.clone(),
                grams: json_ingredient.grams,
            });
        }

        recipes.push(Recipe {
            name: json_recipe.name,
            description: json_recipe.description,
            ingredients: recipe_ingredients,
        });
    }

    Ok(recipes)
}

fn load_json_ingredients(data_dir: &Path) -> Result<JsonIngredients, LoadError> {
    load_jsonc_file(data_dir, "ingredients.jsonc", generate_ingredient_schema)
}
