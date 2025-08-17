use crate::data::loader;
use crate::error::AppResult;
use std::path::Path;

pub fn handle_list_recipes_command(data_dir: &Path) -> AppResult<()> {
    let recipes = loader::load_recipes(data_dir)?;

    println!("Available recipes:");
    for recipe in recipes {
        if let Some(description) = &recipe.description {
            println!("  {} - {}", recipe.name, description);
        } else {
            println!("  {}", recipe.name);
        }
    }
    Ok(())
}
