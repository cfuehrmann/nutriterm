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

    // Add bottom padding so users can scroll any recipe to the top of their screen
    // This is especially important for recipes near the end of the list
    // Using <br/> tags for reliable spacing across all markdown processors
    println!("{}", "<br/>".repeat(40));

    Ok(())
}
