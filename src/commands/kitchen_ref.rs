use crate::data::loader;
use crate::error::AppResult;
use std::path::Path;

pub fn handle_kitchen_ref_command(data_dir: &Path) -> AppResult<()> {
    let recipes = loader::load_recipes(data_dir)?;

    println!("# Kitchen Reference");
    println!();

    for recipe in &recipes {
        println!("## {}", recipe.name);
        println!();

        for ingredient in &recipe.ingredients {
            println!(
                "- {:.1} g  {}",
                ingredient.grams, ingredient.ingredient.name
            );
        }

        println!();
    }

    Ok(())
}
