use super::initialization::{create_ingredient_schema, create_recipe_schema};
use crate::catalog::items::{Ingredient, Recipe, WeightedIngredient};
use crate::error::{AppError, DuplicateGroup};
use crate::utils::suggestions::find_best_suggestion;
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
    ingredients: Vec<JsonRecipeIngredient>,
}

#[derive(Deserialize)]
struct JsonRecipeIngredient {
    id: String,
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

pub fn load_recipes(data_dir: &Path) -> Result<Vec<Recipe>, AppError> {
    let json_recipes: JsonRecipes =
        load_jsonc_file(data_dir, "recipes.jsonc", create_recipe_schema)?;

    check_recipe_uniqueness(&json_recipes.recipes)?;

    let json_ingredients = load_json_ingredients(data_dir)?;
    check_ingredient_uniqueness(&json_ingredients.ingredients)?;
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
            let ingredient = ingredient_map.get(&json_ingredient.id).ok_or_else(|| {
                let available_ids: Vec<String> = ingredient_map.keys().cloned().collect();
                let suggestion = find_best_suggestion(&json_ingredient.id, &available_ids);

                AppError::UnknownIngredientError {
                    recipe: json_recipe.name.clone(),
                    ingredient: json_ingredient.id.clone(),
                    suggestion,
                    available_ids,
                }
            })?;

            recipe_ingredients.push(WeightedIngredient {
                ingredient: ingredient.clone(),
                grams: json_ingredient.grams,
            });
        }

        recipes.push(Recipe {
            name: json_recipe.name,
            ingredients: recipe_ingredients,
        });
    }

    Ok(recipes)
}

fn load_json_ingredients(data_dir: &Path) -> Result<JsonIngredients, AppError> {
    let ingredients: JsonIngredients =
        load_jsonc_file(data_dir, "ingredients.jsonc", create_ingredient_schema)?;
    // Note: duplicates will be handled at domain level, not here
    Ok(ingredients)
}

fn load_jsonc_file<T: DeserializeOwned>(
    data_dir: &Path,
    filename: &str,
    schema_generator: fn() -> Value,
) -> Result<T, AppError> {
    let file_path = data_dir.join(filename);
    let content = std::fs::read_to_string(&file_path).map_err(|e| AppError::FileUnreadable {
        message: format!("Cannot read file {}: {}", file_path.display(), e),
    })?;

    let json_value = jsonc_parser::parse_to_serde_value(&content, &Default::default())
        .map_err(|e| AppError::ParsingError {
            message: format!(
                "Invalid JSONC syntax in {}: {}\n\nTip: Check for missing commas, brackets, or quotes. Most editors highlight syntax errors when you save the file with a .jsonc extension.",
                filename, e
            ),
        })?
        .ok_or_else(|| AppError::ParsingError {
            message: format!("Empty file: {}", filename),
        })?;

    let schema_json = schema_generator();
    let schema = Validator::new(&schema_json).map_err(|e| AppError::Other {
        message: format!("Invalid {} structure: Failed to compile schema: {}", filename, e),
    })?;

    check_with_schema(&json_value, &schema, filename)?;

    serde_json::from_value(json_value).map_err(|e| AppError::Other {
        message: format!("Invalid {} structure: {}", filename, e),
    })
}

// Validation

fn check_with_schema(
    json_value: &Value,
    schema: &Validator,
    filename: &str,
) -> Result<(), AppError> {
    let validation_errors: Vec<_> = schema.iter_errors(json_value).collect();

    if !validation_errors.is_empty() {
        let error_messages: Vec<String> = validation_errors
            .into_iter()
            .map(|error| format!("- {}: {}", error.instance_path, error))
            .collect();

        return Err(AppError::Other {
            message: format!(
                "Schema validation failed for {}:\n{}\n\nTip: Check the values against the expected data types and ranges. Use 'nutriterm init' to see example file formats.",
                filename,
                error_messages.join("\n")
            ),
        });
    }

    Ok(())
}

fn check_recipe_uniqueness(recipes: &[JsonRecipe]) -> Result<(), AppError> {
    check_uniqueness(recipes, "recipes.jsonc", "recipe name", |recipe| {
        (&recipe.name, format!("recipe '{}'", recipe.name))
    })
}

fn check_ingredient_uniqueness(ingredients: &[JsonIngredient]) -> Result<(), AppError> {
    check_uniqueness(
        ingredients,
        "ingredients.jsonc",
        "ingredient ID",
        |ingredient| (&ingredient.id, ingredient.name.clone()),
    )
}

fn check_uniqueness<T, K, F>(
    items: &[T],
    filename: &str,
    key_type: &str,
    key_extractor: F,
) -> Result<(), AppError>
where
    F: Fn(&T) -> (&K, String),
    K: Eq + std::hash::Hash + Clone + std::fmt::Display,
{
    let mut key_groups: HashMap<&K, Vec<String>> = HashMap::new();

    for item in items {
        let (key, display_name) = key_extractor(item);
        key_groups.entry(key).or_default().push(display_name);
    }

    let mut duplicates = Vec::new();
    for (key, items) in key_groups {
        if items.len() > 1 {
            duplicates.push(DuplicateGroup {
                key: key.to_string(),
                items,
            });
        }
    }

    duplicates.sort_by(|a, b| a.key.cmp(&b.key));

    if !duplicates.is_empty() {
        return Err(AppError::DuplicateKeyError {
            filename: filename.to_string(),
            key_type: key_type.to_string(),
            duplicates,
        });
    }

    Ok(())
}


