#[derive(Debug, Clone)]
pub struct Ingredient {
    pub name: String,
    /// Net carbohydrates per 100 grams (total carbs minus fiber)
    pub carbs_per_100g: f64,
    pub protein_per_100g: f64,
    pub fat_per_100g: f64,
    /// Dietary fiber per 100 grams (in grams)
    pub fiber_per_100g: f64,
}
