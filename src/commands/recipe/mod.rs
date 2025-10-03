mod display;
mod search;

use crate::data::loader;
use crate::error::AppResult;
use display::render_nutrition_table;
use search::{find_exact_match, find_substring_matches, parse_search_terms};
use std::io;
use std::path::Path;

pub fn handle_recipe_command(data_dir: &Path, recipe_name: &str) -> AppResult<()> {
    let recipes = loader::load_recipes(data_dir)?;

    if let Some(recipe) = find_exact_match(&recipes, recipe_name) {
        println!("Recipe: {}", recipe.name);
        println!();
        let mut stdout = io::stdout();
        render_nutrition_table(&recipe.ingredients, &mut stdout)?;
        return Ok(());
    }

    let search_terms = parse_search_terms(recipe_name);
    let matches = find_substring_matches(&recipes, &search_terms);

    match matches.len() {
        0 => {
            let available: Vec<String> = recipes.iter().map(|r| r.name.clone()).collect();
            if available.is_empty() {
                println!("No matches for '{}'", recipe_name);
            } else {
                println!(
                    "No matches for '{}'. Available recipes: {}",
                    recipe_name,
                    available.join(", ")
                );
            }
            Ok(())
        }
        1 => {
            let recipe = matches[0];
            println!("Recipe: {}", recipe.name);
            println!();
            let mut stdout = io::stdout();
            render_nutrition_table(&recipe.ingredients, &mut stdout)?;
            Ok(())
        }
        _ => {
            let available: Vec<String> = matches.iter().map(|r| r.name.clone()).collect();
            let display_count = std::cmp::min(3, matches.len());
            let example_recipes: Vec<String> =
                available.iter().take(display_count).cloned().collect();

            if matches.len() <= 3 {
                println!(
                    "Multiple recipes found for '{}' ({} matches):\n{}\n\nPlease be more specific with your search term.",
                    recipe_name,
                    matches.len(),
                    example_recipes
                        .iter()
                        .map(|name| format!("- {}", name))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            } else {
                println!(
                    "Multiple recipes found for '{}' ({} matches):\n{}\n... and {} more\n\nPlease be more specific with your search term.",
                    recipe_name,
                    matches.len(),
                    example_recipes
                        .iter()
                        .map(|name| format!("- {}", name))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    matches.len() - display_count
                );
            }
            Ok(())
        }
    }
}
