use super::Ingredient;

/// Ingredient along with its weight in grams.
///
/// "Carbs" fields refer to net carbohydrates (excluding fiber).
const PER_100G_FACTOR: f64 = 0.01;

#[derive(Debug, Clone)]
pub struct WeightedIngredient {
    pub grams: f64,
    pub ingredient: Ingredient,
}

impl WeightedIngredient {
    /// Net carbohydrates for the given weight (grams)
    ///
    /// Fiber is not included in this value.
    pub fn carbs_grams(&self) -> f64 {
        self.grams * self.ingredient.carbs_per_100g * PER_100G_FACTOR
    }

    pub fn protein_grams(&self) -> f64 {
        self.grams * self.ingredient.protein_per_100g * PER_100G_FACTOR
    }

    pub fn fat_grams(&self) -> f64 {
        self.grams * self.ingredient.fat_per_100g * PER_100G_FACTOR
    }
    /// Dietary fiber in grams for the weighted ingredient
    pub fn fiber_grams(&self) -> f64 {
        self.grams * self.ingredient.fiber_per_100g * PER_100G_FACTOR
    }

    pub fn calories(&self) -> f64 {
        self.protein_grams() * 4.0 + self.fat_grams() * 9.0 + self.carbs_grams() * 4.0
    }
}
