use crate::models::WeightedIngredient;
use std::io::Write;
use tabled::{
    Table, Tabled,
    settings::{Alignment, Color, Format, Modify, Padding, Style, object::Rows},
};

#[derive(Tabled)]
struct NutritionRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Weight")]
    weight: String,
    #[tabled(rename = "Net carbs")]
    carbs: String,
    #[tabled(rename = "Protein")]
    protein: String,
    #[tabled(rename = "Fat")]
    fat: String,
    #[tabled(rename = "Fiber")]
    fiber: String,
    #[tabled(rename = "Calories")]
    calories: String,
}

pub fn render_nutrition_table<W: Write>(
    recipe: &[WeightedIngredient],
    writer: &mut W,
) -> std::io::Result<()> {
    let mut rows = Vec::new();
    let mut totals = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    for ingredient in recipe {
        let carbs = ingredient.carbs_grams();
        let protein = ingredient.protein_grams();
        let fat = ingredient.fat_grams();
        let fiber = ingredient.fiber_grams();
        let calories = ingredient.calories();

        totals.0 += ingredient.grams;
        totals.1 += carbs;
        totals.2 += protein;
        totals.3 += fat;
        totals.4 += fiber;
        totals.5 += calories;

        let display_name = if ingredient.ingredient.name.len() > 25 {
            format!(
                "{}…",
                ingredient
                    .ingredient
                    .name
                    .chars()
                    .take(24)
                    .collect::<String>()
            )
        } else {
            ingredient.ingredient.name.clone()
        };

        rows.push(NutritionRow {
            name: display_name,
            weight: format_number_with_unit(ingredient.grams, "g"),
            carbs: format_number_with_unit(carbs, "g"),
            protein: format_number_with_unit(protein, "g"),
            fat: format_number_with_unit(fat, "g"),
            fiber: format_number_with_unit(fiber, "g"),
            calories: format_calories(calories),
        });
    }

    rows.push(NutritionRow {
        name: "Total".to_string(),
        weight: format_number_with_unit(totals.0, "g"),
        carbs: format_number_with_unit(totals.1, "g"),
        protein: format_number_with_unit(totals.2, "g"),
        fat: format_number_with_unit(totals.3, "g"),
        fiber: format_number_with_unit(totals.4, "g"),
        calories: format_calories(totals.5),
    });

    let last_row = rows.len();
    let mut table = Table::new(&rows);
    table
        .with(Style::rounded())
        .with(
            Modify::new(Rows::new(0..=0))
                .with(Color::FG_CYAN)
                .with(Format::content(|s| format!(" {} ", s))),
        )
        .with(Modify::new(Rows::new(1..last_row)).with(Alignment::right()))
        .with(
            Modify::new(Rows::new(last_row..=last_row))
                .with(Color::FG_BRIGHT_WHITE)
                .with(Alignment::right()),
        )
        .with(Padding::new(1, 1, 0, 0));

    writeln!(writer, "{}", table)?;

    Ok(())
}

// Low-level formatting utilities (bottom of call chain)
fn add_thousand_separators(s: &str) -> String {
    let parts: Vec<&str> = s.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = if parts.len() > 1 { parts[1] } else { "" };

    let mut result = String::new();
    for (i, c) in integer_part.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }

    let formatted_integer: String = result.chars().rev().collect();
    if decimal_part.is_empty() || decimal_part == "0" {
        formatted_integer
    } else {
        format!("{}.{}", formatted_integer, decimal_part)
    }
}

fn format_large_number(value: f64) -> String {
    if value >= 1000.0 {
        let formatted = format!("{:.1}", value);
        add_thousand_separators(&formatted)
    } else {
        format!("{:.1}", value)
    }
}

// Mid-level formatting functions (using above utilities)
fn format_number_with_unit(value: f64, unit: &str) -> String {
    if value <= 0.01 {
        format!("0 {}", unit)
    } else if value >= 1000.0 {
        format!("{} {}", format_large_number(value), unit)
    } else {
        format!("{:.1} {}", value, unit)
    }
}

fn format_calories(calories: f64) -> String {
    if calories <= 0.01 {
        "0 kcal".to_string()
    } else if calories >= 1000.0 {
        format!(
            "{} kcal",
            add_thousand_separators(&format!("{:.0}", calories))
        )
    } else {
        format!("{:.0} kcal", calories)
    }
}
