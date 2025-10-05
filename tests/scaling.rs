use insta::assert_snapshot;

mod common;
use common::{run_cmd, strip_ansi_codes, temp_dir, write_files};

fn create_test_catalog_dir(ingredients_content: &str, recipes_content: &str) -> tempfile::TempDir {
    let temp = temp_dir();
    write_files(temp.path(), ingredients_content, recipes_content);
    temp
}

fn format_scaling_snapshot(recipe_name: &str, output: &str) -> String {
    format!(
        "Available recipes: {}\n$ nutriterm recipe {}\n{}",
        recipe_name,
        recipe_name,
        strip_ansi_codes(output).trim_end()
    )
}

#[test]
fn test_long_ingredient_names() {
    // Create ingredients with various name lengths
    let ingredients_content = r#"{
  "ingredients": [
    {
      "id": "short",
      "name": "Short Name",
      "carbs_per_100g": 10.0,
      "protein_per_100g": 5.0,
      "fat_per_100g": 2.0,
      "fiber_per_100g": 1.0
    },
    {
      "id": "exactly_twenty_five_chars",
      "name": "Exactly Twenty Five Chars",
      "carbs_per_100g": 15.0,
      "protein_per_100g": 8.0,
      "fat_per_100g": 3.0,
      "fiber_per_100g": 2.0
    },
    {
      "id": "very_long_ingredient_name_exceeding_twenty_five_characters",
      "name": "Very Long Ingredient Name That Definitely Exceeds Twenty Five Characters And Should Be Truncated With Ellipsis",
      "carbs_per_100g": 20.0,
      "protein_per_100g": 12.0,
      "fat_per_100g": 4.0,
      "fiber_per_100g": 3.0
    }
  ]
}"#;

    let recipes_content = r#"{
  "recipes": [
    {
      "name": "long-names-test",
      "ingredients": [
        {
          "id": "short",
          "grams": 100.0
        },
        {
          "id": "exactly_twenty_five_chars",
          "grams": 150.0
        },
        {
          "id": "very_long_ingredient_name_exceeding_twenty_five_characters",
          "grams": 200.0
        }
      ]
    }
  ]
}"#;

    let temp_dir = create_test_catalog_dir(ingredients_content, recipes_content);

    let output = run_cmd(&["recipe", "long-names-test"], temp_dir.path());

    let content =
        format_scaling_snapshot("long-names-test", &String::from_utf8_lossy(&output.stdout));

    assert_snapshot!("long_ingredient_names", content);
}

#[test]
fn test_extreme_numerical_values() {
    // Create ingredients with various numerical ranges
    let ingredients_content = r#"{
  "ingredients": [
    {
      "id": "tiny_values",
      "name": "Tiny Values",
      "carbs_per_100g": 0.001,
      "protein_per_100g": 0.005,
      "fat_per_100g": 0.0001,
      "fiber_per_100g": 0.0005
    },
    {
      "id": "small_values",
      "name": "Small Values", 
      "carbs_per_100g": 0.15,
      "protein_per_100g": 0.25,
      "fat_per_100g": 0.08,
      "fiber_per_100g": 0.12
    },
    {
      "id": "large_values",
      "name": "Large Values",
      "carbs_per_100g": 95.5,
      "protein_per_100g": 88.8,
      "fat_per_100g": 77.7,
      "fiber_per_100g": 66.6
    }
  ]
}"#;

    let recipes_content = r#"{
  "recipes": [
    {
      "name": "extreme-values-test",
      "ingredients": [
        {
          "id": "tiny_values",
          "grams": 0.01
        },
        {
          "id": "small_values",
          "grams": 5.5
        },
        {
          "id": "large_values",
          "grams": 2500.0
        }
      ]
    }
  ]
}"#;

    let temp_dir = create_test_catalog_dir(ingredients_content, recipes_content);

    let output = run_cmd(&["recipe", "extreme-values-test"], temp_dir.path());

    let content = format_scaling_snapshot(
        "extreme-values-test",
        &String::from_utf8_lossy(&output.stdout),
    );

    assert_snapshot!("extreme_numerical_values", content);
}

#[test]
fn test_comma_formatting_boundary() {
    // Test values around the 1000 boundary for comma formatting
    let ingredients_content = r#"{
  "ingredients": [
    {
      "id": "below_thousand",
      "name": "Below Thousand",
      "carbs_per_100g": 50.0,
      "protein_per_100g": 30.0,
      "fat_per_100g": 20.0,
      "fiber_per_100g": 10.0
    },
    {
      "id": "at_thousand",
      "name": "At Thousand",
      "carbs_per_100g": 80.0,
      "protein_per_100g": 60.0,
      "fat_per_100g": 40.0,
      "fiber_per_100g": 30.0
    }
  ]
}"#;

    let recipes_content = r#"{
  "recipes": [
    {
      "name": "comma-boundary-test",
      "ingredients": [
        {
          "id": "below_thousand",
          "grams": 999.0
        },
        {
          "id": "at_thousand",
          "grams": 1000.0
        }
      ]
    }
  ]
}"#;

    let temp_dir = create_test_catalog_dir(ingredients_content, recipes_content);

    let output = run_cmd(&["recipe", "comma-boundary-test"], temp_dir.path());

    let content = format_scaling_snapshot(
        "comma-boundary-test",
        &String::from_utf8_lossy(&output.stdout),
    );

    assert_snapshot!("comma_formatting_boundary", content);
}

#[test]
fn test_precision_levels() {
    // Test different precision levels based on value ranges
    let ingredients_content = r#"{
  "ingredients": [
    {
      "id": "precision_test",
      "name": "Precision Test",
      "carbs_per_100g": 1.2345,
      "protein_per_100g": 0.6789,
      "fat_per_100g": 0.0987,
      "fiber_per_100g": 0.0054
    }
  ]
}"#;

    let recipes_content = r#"{
  "recipes": [
    {
      "name": "precision-test",
      "ingredients": [
        {
          "id": "precision_test",
          "grams": 123.456
        }
      ]
    }
  ]
}"#;

    let temp_dir = create_test_catalog_dir(ingredients_content, recipes_content);

    let output = run_cmd(&["recipe", "precision-test"], temp_dir.path());

    let content =
        format_scaling_snapshot("precision-test", &String::from_utf8_lossy(&output.stdout));

    assert_snapshot!("precision_levels", content);
}

#[test]
fn test_zero_values_formatting() {
    // Test how zero and near-zero values are formatted
    let ingredients_content = r#"{
  "ingredients": [
    {
      "id": "zero_carbs",
      "name": "Zero Carbs",
      "carbs_per_100g": 0.0,
      "protein_per_100g": 25.0,
      "fat_per_100g": 0.0,
      "fiber_per_100g": 0.0
    },
    {
      "id": "near_zero",
      "name": "Near Zero",
      "carbs_per_100g": 0.0001,
      "protein_per_100g": 0.0002,
      "fat_per_100g": 0.0000,
      "fiber_per_100g": 0.00001
    }
  ]
}"#;

    let recipes_content = r#"{
  "recipes": [
    {
      "name": "zero-values-test",
      "ingredients": [
        {
          "id": "zero_carbs",
          "grams": 100.0
        },
        {
          "id": "near_zero",
          "grams": 50.0
        }
      ]
    }
  ]
}"#;

    let temp_dir = create_test_catalog_dir(ingredients_content, recipes_content);

    let output = run_cmd(&["recipe", "zero-values-test"], temp_dir.path());

    let content =
        format_scaling_snapshot("zero-values-test", &String::from_utf8_lossy(&output.stdout));

    assert_snapshot!("zero_values_formatting", content);
}

#[test]
fn test_mixed_extreme_scenarios() {
    // Combine long names with extreme values
    let ingredients_content = r#"{
  "ingredients": [
    {
      "id": "extremely_long_ingredient_name_with_many_words_and_descriptive_text_exceeding_limits",
      "name": "Extremely Long Ingredient Name With Many Words And Descriptive Text That Definitely Exceeds Character Limits For Display",
      "carbs_per_100g": 99.999,
      "protein_per_100g": 0.001,
      "fat_per_100g": 50.5555,
      "fiber_per_100g": 0.0001
    },
    {
      "id": "x",
      "name": "X",
      "carbs_per_100g": 0.0,
      "protein_per_100g": 0.0,
      "fat_per_100g": 0.0,
      "fiber_per_100g": 0.0
    }
  ]
}"#;

    let recipes_content = r#"{
  "recipes": [
    {
      "name": "mixed-extreme-test",
      "ingredients": [
        {
          "id": "extremely_long_ingredient_name_with_many_words_and_descriptive_text_exceeding_limits",
          "grams": 1234.567
        },
        {
          "id": "x",
          "grams": 0.001
        }
      ]
    }
  ]
}"#;

    let temp_dir = create_test_catalog_dir(ingredients_content, recipes_content);

    let output = run_cmd(&["recipe", "mixed-extreme-test"], temp_dir.path());

    let content = format_scaling_snapshot(
        "mixed-extreme-test",
        &String::from_utf8_lossy(&output.stdout),
    );

    assert_snapshot!("mixed_extreme_scenarios", content);
}
