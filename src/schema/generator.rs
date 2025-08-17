use serde_json::Value;
use std::path::Path;

const RECIPE_SCHEMA: &str = include_str!("recipes.schema.json");
const INGREDIENT_SCHEMA: &str = include_str!("ingredients.schema.json");

pub fn generate_recipe_schema() -> Value {
    serde_json::from_str(RECIPE_SCHEMA).expect("Embedded recipe schema should be valid JSON")
}

pub fn generate_ingredient_schema() -> Value {
    serde_json::from_str(INGREDIENT_SCHEMA)
        .expect("Embedded ingredient schema should be valid JSON")
}

use crate::error::AppResult;

pub fn generate_all_schemas(output_dir: &Path) -> AppResult<()> {
    std::fs::create_dir_all(output_dir)?;

    let recipe_schema = generate_recipe_schema();
    let recipe_schema_path = output_dir.join("recipes.schema.json");
    let recipe_json = serde_json::to_string_pretty(&recipe_schema).map_err(|e| {
        crate::error::AppError::Other(format!(
            "Failed to serialize recipe schema to JSON for file '{}': {}",
            recipe_schema_path.display(),
            e
        ))
    })?;
    std::fs::write(&recipe_schema_path, recipe_json)?;

    let ingredient_schema = generate_ingredient_schema();
    let ingredient_path = output_dir.join("ingredients.schema.json");
    let ingredient_json = serde_json::to_string_pretty(&ingredient_schema).map_err(|e| {
        crate::error::AppError::Other(format!(
            "Failed to serialize ingredient schema to JSON for file '{}': {}",
            ingredient_path.display(),
            e
        ))
    })?;
    std::fs::write(ingredient_path, ingredient_json)?;

    Ok(())
}
