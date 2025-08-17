use assert_cmd::Command;
use insta::assert_snapshot;
use std::fs;
use tempfile::TempDir;

mod common;
use common::{
    create_workspace_files, format_test_snapshot, normalize_temp_paths, temp_dir, workspace_dir,
    write_files,
};

// Helper for recipe tests that need the standard workspace with chicken-rice-bowl
fn recipe_workspace() -> (TempDir, std::path::PathBuf) {
    let temp = temp_dir();
    let workspace = workspace_dir(&temp, "workspace");
    create_workspace_files(&workspace);
    (temp, workspace)
}

#[test]
fn test_view_valid_recipe() {
    let (_temp_dir, workspace_dir) = recipe_workspace();

    // User views nutrition for valid recipe
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "chicken-rice-bowl"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content =
        format_test_snapshot(&["chicken-rice-bowl"], "chicken-rice-bowl", &stdout);
    assert_snapshot!("nutrition_display", snapshot_content);
}

#[test]
fn test_view_invalid_recipe() {
    let (_temp_dir, workspace_dir) = recipe_workspace();

    // User tries to view nutrition for invalid recipe and gets helpful suggestions
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "nonexistent-recipe"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content =
        format_test_snapshot(&["chicken-rice-bowl"], "nonexistent-recipe", &stdout);
    assert_snapshot!("not_found_with_suggestions", snapshot_content);
}

#[test]
fn test_view_outside_workspace() {
    // User tries to view recipe outside workspace
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "anything"])
        .current_dir("/tmp")
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_snapshot!("no_workspace", stderr);
}

#[test]
fn test_view_with_broken_workspace() {
    let temp_dir = TempDir::new().unwrap();

    // User tries to view recipe with missing ingredients file
    let broken_workspace = temp_dir.path().join("broken-recipe");
    fs::create_dir_all(&broken_workspace).unwrap();
    std::fs::write(broken_workspace.join("recipes.jsonc"), r#"{"recipes": []}"#).unwrap();

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "anything"])
        .current_dir(&broken_workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("missing_ingredients", normalized_stderr);
}

#[test]
fn test_search_exact_match() {
    let (_temp_dir, workspace_dir) = recipe_workspace();

    // User searches with exact recipe name - should work as before
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "chicken-rice-bowl"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content =
        format_test_snapshot(&["chicken-rice-bowl"], "chicken-rice-bowl", &stdout);
    assert_snapshot!("search_exact_match", snapshot_content);
}

#[test]
fn test_search_substring_match() {
    let (_temp_dir, workspace_dir) = recipe_workspace();

    // User searches with partial term - should find unique match
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "chicken"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content = format_test_snapshot(&["chicken-rice-bowl"], "chicken", &stdout);
    assert_snapshot!("search_single_substring_match", snapshot_content);
}

#[test]
fn test_search_multiple_terms() {
    let temp_dir = temp_dir();
    let workspace_dir = workspace_dir(&temp_dir, "workspace");

    // Tests AND logic: multi-word searches should only match recipes containing ALL terms
    std::fs::write(
        workspace_dir.join("recipes.jsonc"),
        r#"{
  "recipes": [
    {
      "name": "chicken-rice-bowl",
      "ingredients": [{"name": "chicken_breast", "grams": 150}]
    },
    {
      "name": "beef-rice-stir-fry", 
      "ingredients": [{"name": "chicken_breast", "grams": 100}]
    },
    {
      "name": "chicken-salad",
      "ingredients": [{"name": "chicken_breast", "grams": 120}]
    }
  ]
}"#,
    )
    .unwrap();

    std::fs::write(
        workspace_dir.join("ingredients.jsonc"),
        r#"{
  "ingredients": [{
    "name": "chicken_breast",
    "display_name": "Chicken Breast",
    "carbs_per_100g": 0,
    "protein_per_100g": 31,
    "fat_per_100g": 3.6,
    "fiber_per_100g": 0
  }]
}"#,
    )
    .unwrap();

    // User searches with multiple terms - should find only recipes with ALL terms
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "chicken rice"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content = format_test_snapshot(
        &["chicken-rice-bowl", "beef-rice-stir-fry", "chicken-salad"],
        "\"chicken rice\"",
        &stdout,
    );
    assert_snapshot!("search_multiple_terms_unique_match", snapshot_content);
}

#[test]
fn test_search_multiple_matches() {
    let temp_dir = temp_dir();
    let workspace_dir = workspace_dir(&temp_dir, "workspace");

    // Tests user guidance when search is ambiguous - should show examples and suggest refinement
    std::fs::write(
        workspace_dir.join("recipes.jsonc"),
        r#"{
  "recipes": [
    {
      "name": "chicken-rice-bowl",
      "description": "A balanced meal",
      "ingredients": [{"name": "chicken_breast", "grams": 150}]
    },
    {
      "name": "chicken-salad",
      "description": "Fresh salad", 
      "ingredients": [{"name": "chicken_breast", "grams": 120}]
    },
    {
      "name": "spicy-chicken-curry",
      "ingredients": [{"name": "chicken_breast", "grams": 200}]
    }
  ]
}"#,
    )
    .unwrap();

    std::fs::write(
        workspace_dir.join("ingredients.jsonc"),
        r#"{
  "ingredients": [{
    "name": "chicken_breast",
    "display_name": "Chicken Breast",
    "carbs_per_100g": 0,
    "protein_per_100g": 31,
    "fat_per_100g": 3.6,
    "fiber_per_100g": 0
  }]
}"#,
    )
    .unwrap();

    // User searches with term that matches multiple recipes
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "chicken"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content = format_test_snapshot(
        &["chicken-rice-bowl", "chicken-salad", "spicy-chicken-curry"],
        "chicken",
        &stdout,
    );
    assert_snapshot!("search_multiple_matches", snapshot_content);
}

#[test]
fn test_search_no_matches() {
    let (_temp_dir, workspace_dir) = recipe_workspace();

    // User searches for non-existent recipe
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "pizza"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content = format_test_snapshot(&["chicken-rice-bowl"], "pizza", &stdout);
    assert_snapshot!("search_no_matches", snapshot_content);
}

#[test]
fn test_search_case_insensitive() {
    let (_temp_dir, workspace_dir) = recipe_workspace();

    // User searches with different case - should still match
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "CHICKEN"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content = format_test_snapshot(&["chicken-rice-bowl"], "CHICKEN", &stdout);
    assert_snapshot!("search_case_insensitive", snapshot_content);
}

// Schema validation tests - testing user workflows when data doesn't meet schema requirements

#[test]
fn test_view_with_invalid_ingredient_data() {
    // User tries to view recipe when ingredients file has schema violations
    let temp_dir = temp_dir();
    let workspace = workspace_dir(&temp_dir, "schema-test");
    write_files(
        &workspace,
        r#"{
        "ingredients": [{
            "name": "invalid_ingredient",
            "display_name": "Invalid Ingredient",
            "carbs_per_100g": -5,
            "protein_per_100g": 25,
            "fat_per_100g": 10,
            "fiber_per_100g": 3
        }]
    }"#,
        r#"{
        "recipes": [{
            "name": "test-recipe",
            "ingredients": [{"name": "invalid_ingredient", "grams": 100}]
        }]
    }"#,
    );

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "test-recipe"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("view_schema_ingredient_validation_error", normalized_stderr);
}

#[test]
fn test_view_with_invalid_recipe_data() {
    // User tries to view recipe when recipe itself has schema violations
    let temp_dir = temp_dir();
    let workspace = workspace_dir(&temp_dir, "schema-test");
    write_files(
        &workspace,
        r#"{
        "ingredients": [{
            "name": "test_ingredient",
            "display_name": "Test Ingredient",
            "carbs_per_100g": 50,
            "protein_per_100g": 25,
            "fat_per_100g": 10,
            "fiber_per_100g": 3
        }]
    }"#,
        r#"{
        "recipes": [{
            "name": "invalid-recipe",
            "ingredients": [{
                "name": "test_ingredient",
                "grams": -100
            }]
        }]
    }"#,
    );

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "invalid-recipe"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("view_schema_recipe_validation_error", normalized_stderr);
}
