use crate::error::AppResult;
use serde_json::Value;
use std::path::Path;

const RECIPE_SCHEMA: &str = include_str!("recipes.schema.json");
const INGREDIENT_SCHEMA: &str = include_str!("ingredients.schema.json");
const RECIPE_TEMPLATE: &str = include_str!("recipes.template.jsonc");
const INGREDIENT_TEMPLATE: &str = include_str!("ingredients.template.jsonc");

/// Initialize a complete catalog with all required files and editor support
pub fn initialize(output_dir: &Path) -> AppResult<()> {
    create_data_files(output_dir)?;
    create_schemas(output_dir)?;
    Ok(())
}

/// Schema generators for loader validation (exported to storage module)
pub(super) fn create_recipe_schema() -> Result<Value, crate::error::AppError> {
    serde_json::from_str(RECIPE_SCHEMA).map_err(|e| crate::error::AppError::InvalidSchema {
        message: format!("Failed to parse embedded recipe schema: {}", e),
    })
}

pub(super) fn create_ingredient_schema() -> Result<Value, crate::error::AppError> {
    serde_json::from_str(INGREDIENT_SCHEMA).map_err(|e| crate::error::AppError::InvalidSchema {
        message: format!("Failed to parse embedded ingredient schema: {}", e),
    })
}

/// Create the required data files with starter content
fn create_data_files(output_dir: &Path) -> AppResult<()> {
    let recipes_path = output_dir.join("recipes.jsonc");
    std::fs::write(recipes_path, get_recipe_template())?;

    let ingredients_path = output_dir.join("ingredients.jsonc");
    std::fs::write(ingredients_path, get_ingredient_template())?;

    Ok(())
}

/// Create editor support files (JSON Schema files)
fn create_schemas(output_dir: &Path) -> AppResult<()> {
    let recipe_schema_path = output_dir.join("recipes.schema.json");
    std::fs::write(&recipe_schema_path, RECIPE_SCHEMA)?;

    let ingredient_schema_path = output_dir.join("ingredients.schema.json");
    std::fs::write(&ingredient_schema_path, INGREDIENT_SCHEMA)?;

    Ok(())
}

/// Template accessors (private - only used internally)
fn get_recipe_template() -> &'static str {
    RECIPE_TEMPLATE
}

fn get_ingredient_template() -> &'static str {
    INGREDIENT_TEMPLATE
}
