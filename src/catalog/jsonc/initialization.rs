use crate::error::AppResult;
use serde_json::Value;
use std::path::Path;

const RECIPE_SCHEMA: &str = include_str!("recipes.schema.json");
const INGREDIENT_SCHEMA: &str = include_str!("ingredients.schema.json");
const RECIPE_TEMPLATE: &str = include_str!("recipes.template.jsonc");
const INGREDIENT_TEMPLATE: &str = include_str!("ingredients.template.jsonc");

/// Initialize a complete catalog with all required files and editor support
pub fn initialize(output_dir: &Path) -> AppResult<()> {
    create_required_files(output_dir)?;
    create_all_schemas(output_dir)?;
    Ok(())
}

/// Schema generators for loader validation (exported to storage module)
pub(super) fn create_recipe_schema() -> Value {
    serde_json::from_str(RECIPE_SCHEMA).expect("Embedded recipe schema should be valid JSON")
}

pub(super) fn create_ingredient_schema() -> Value {
    serde_json::from_str(INGREDIENT_SCHEMA)
        .expect("Embedded ingredient schema should be valid JSON")
}

/// Create the required data files with starter content
fn create_required_files(output_dir: &Path) -> AppResult<()> {
    let recipes_path = output_dir.join("recipes.jsonc");
    std::fs::write(recipes_path, get_recipe_template())?;

    let ingredients_path = output_dir.join("ingredients.jsonc");
    std::fs::write(ingredients_path, get_ingredient_template())?;

    Ok(())
}

/// Create editor support files (JSON Schema files)
fn create_all_schemas(output_dir: &Path) -> AppResult<()> {
    std::fs::create_dir_all(output_dir)?;

    let recipe_schema = create_recipe_schema();
    let recipe_schema_path = output_dir.join("recipes.schema.json");
    let recipe_json = serde_json::to_string_pretty(&recipe_schema).map_err(|e| {
        crate::error::AppError::Other(format!(
            "Failed to serialize recipe schema to JSON for file '{}': {}",
            recipe_schema_path.display(),
            e
        ))
    })?;
    std::fs::write(&recipe_schema_path, recipe_json)?;

    let ingredient_schema = create_ingredient_schema();
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

/// Template accessors (private - only used internally)
fn get_recipe_template() -> &'static str {
    RECIPE_TEMPLATE
}

fn get_ingredient_template() -> &'static str {
    INGREDIENT_TEMPLATE
}
