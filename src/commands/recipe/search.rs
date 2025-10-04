use crate::catalog::items::Recipe;

pub(super) fn find_exact_match<'a>(recipes: &'a [Recipe], name: &str) -> Option<&'a Recipe> {
    recipes.iter().find(|r| r.name == name)
}

pub(super) fn find_substring_matches<'a>(recipes: &'a [Recipe], search_terms: &[&str]) -> Vec<&'a Recipe> {
    recipes
        .iter()
        .filter(|recipe| {
            let recipe_name_lower = recipe.name.to_lowercase();
            search_terms
                .iter()
                .all(|term| recipe_name_lower.contains(&term.to_lowercase()))
        })
        .collect()
}

pub(super) fn parse_search_terms(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
