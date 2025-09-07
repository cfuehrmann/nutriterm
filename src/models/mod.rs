mod ingredient;
mod weighted_ingredient;

pub use ingredient::Ingredient;
pub use weighted_ingredient::WeightedIngredient;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<WeightedIngredient>,
}
