mod display;
mod search;

use crate::catalog;
use crate::error::AppResult;
use display::render_nutrition_table;
use search::{find_exact_match, find_substring_matches, parse_search_terms};
use std::io;

pub fn run(recipe_name: &str) -> AppResult<()> {
    let catalog_dir = catalog::find_dir()?;
    let recipes = catalog::load_recipes(&catalog_dir)?;

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
            const MAX_DISPLAYED: usize = 3;

            let displayed_count = std::cmp::min(MAX_DISPLAYED, matches.len());
            let recipe_list = available
                .iter()
                .take(displayed_count)
                .map(|name| format!("- {}", name))
                .collect::<Vec<_>>()
                .join("\n");

            let more_text = if matches.len() > MAX_DISPLAYED {
                format!("\n... and {} more", matches.len() - MAX_DISPLAYED)
            } else {
                String::new()
            };

            println!(
                "Multiple recipes found for '{}' ({} matches):\n{}{}\n\nPlease be more specific with your search term.",
                recipe_name,
                matches.len(),
                recipe_list,
                more_text
            );
            Ok(())
        }
    }
}
