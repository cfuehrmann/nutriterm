use crate::data::loader;
use crate::error::AppResult;
use std::path::Path;

pub fn handle_kitchen_ref_command(data_dir: &Path) -> AppResult<()> {
    let recipes = loader::load_recipes(data_dir)?;

    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<head><title>Kitchen Reference</title></head>");
    println!("<body>");
    println!("<h1>Kitchen Reference</h1>");
    println!();

    for recipe in &recipes {
        println!("<h2>{}</h2>", recipe.name);
        println!("<ul>");

        for ingredient in &recipe.ingredients {
            println!(
                "<li>{:.1} g  {}</li>",
                ingredient.grams, ingredient.ingredient.name
            );
        }

        println!("</ul>");
        println!();
    }

    // Add bottom padding so users can scroll any recipe to the top of their screen
    // This is especially important for recipes near the end of the list
    println!("<div style=\"height: 50vh;\"></div>");
    println!();
    println!("</body>");
    println!("</html>");

    Ok(())
}
