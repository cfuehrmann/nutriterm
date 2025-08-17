use assert_cmd::Command;
use insta::assert_snapshot;
use std::fs;
use tempfile::TempDir;

mod common;
use common::{
    create_workspace_files, format_list_test_snapshot, normalize_temp_paths, temp_dir,
    workspace_dir, write_files,
};

// Helper for schema validation tests - avoids repetitive workspace setup
fn schema_workspace(
    ingredients_content: &str,
    recipes_content: Option<&str>,
) -> (TempDir, std::path::PathBuf) {
    let temp = temp_dir();
    let workspace = workspace_dir(&temp, "schema-test");

    let recipes_json = recipes_content.unwrap_or(r#"{"recipes": []}"#);
    write_files(&workspace, ingredients_content, recipes_json);

    (temp, workspace)
}

#[test]
fn test_in_valid_workspace() {
    let temp = temp_dir();
    let workspace_dir = workspace_dir(&temp, "workspace");
    create_workspace_files(&workspace_dir);

    // User lists available recipes in valid workspace
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content = format_list_test_snapshot(
        &["chicken-rice-bowl"],
        &[], // No additional args for basic list-recipes
        &stdout,
    );
    assert_snapshot!("success", snapshot_content);
}

#[test]
fn test_outside_workspace() {
    // User tries to list recipes outside workspace
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir("/tmp")
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_snapshot!("no_workspace", stderr);
}

#[test]
fn test_with_missing_files() {
    let temp_dir = TempDir::new().unwrap();

    // User tries to list recipes with missing recipes file
    let broken_workspace = temp_dir.path().join("broken");
    fs::create_dir_all(&broken_workspace).unwrap();
    std::fs::write(
        broken_workspace.join("ingredients.jsonc"),
        r#"{"ingredients": []}"#,
    )
    .unwrap();

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&broken_workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("missing_file", normalized_stderr);
}

#[test]
fn test_with_invalid_json() {
    let temp_dir = TempDir::new().unwrap();

    // User tries to list recipes with invalid JSON syntax
    let corrupted_workspace = temp_dir.path().join("corrupted");
    fs::create_dir_all(&corrupted_workspace).unwrap();
    std::fs::write(corrupted_workspace.join("recipes.jsonc"), "{ invalid json").unwrap();
    std::fs::write(
        corrupted_workspace.join("ingredients.jsonc"),
        r#"{"ingredients": []}"#,
    )
    .unwrap();

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&corrupted_workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("invalid_json", normalized_stderr);
}

#[test]
fn test_with_unknown_ingredient_reference() {
    let temp_dir = TempDir::new().unwrap();

    // User tries to list recipes with unknown ingredient reference
    let bad_refs_workspace = temp_dir.path().join("bad-refs");
    fs::create_dir_all(&bad_refs_workspace).unwrap();
    std::fs::write(
        bad_refs_workspace.join("recipes.jsonc"),
        r#"{
        "recipes": [{
            "name": "test-recipe",
            "ingredients": [{"name": "nonexistent_ingredient", "grams": 100}]
        }]
    }"#,
    )
    .unwrap();
    std::fs::write(
        bad_refs_workspace.join("ingredients.jsonc"),
        r#"{"ingredients": []}"#,
    )
    .unwrap();

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&bad_refs_workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("unknown_ingredient", normalized_stderr);
}

// Schema validation tests ensure users get clear feedback when their data files have validation errors
// This prevents silent failures and helps users fix data quality issues quickly

#[test]
fn test_with_invalid_ingredient_negative_values() {
    // Negative nutrition values are nonsensical - users need clear validation feedback
    let (temp_dir, workspace) = schema_workspace(
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
        None,
    );

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("schema_ingredients_negative_values", normalized_stderr);
}

#[test]
fn test_with_invalid_ingredient_excessive_values() {
    // Values over 100g per 100g are impossible - helps catch data entry errors
    let (temp_dir, workspace) = schema_workspace(
        r#"{
        "ingredients": [{
            "name": "impossible_ingredient",
            "display_name": "Impossible Ingredient",
            "carbs_per_100g": 50,
            "protein_per_100g": 150,
            "fat_per_100g": 10,
            "fiber_per_100g": 3
        }]
    }"#,
        None,
    );

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("schema_ingredients_excessive_values", normalized_stderr);
}

#[test]
fn test_with_zero_grams_recipe() {
    // Zero-gram ingredients in recipes are meaningless - catches likely data errors
    let (temp_dir, workspace) = schema_workspace(
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
        Some(
            r#"{
        "recipes": [{
            "name": "invalid-recipe",
            "ingredients": [{
                "name": "test_ingredient",
                "grams": 0
            }]
        }]
    }"#,
        ),
    );

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("schema_recipes_zero_grams", normalized_stderr);
}

#[test]
fn test_with_negative_grams_recipe() {
    // Negative ingredient amounts are impossible - prevents nonsensical calculations
    let (temp_dir, workspace) = schema_workspace(
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
        Some(
            r#"{
        "recipes": [{
            "name": "invalid-recipe",
            "ingredients": [{
                "name": "test_ingredient",
                "grams": -100
            }]
        }]
    }"#,
        ),
    );

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("schema_recipes_negative_grams", normalized_stderr);
}

#[test]
fn test_with_multiple_schema_errors() {
    // Multiple validation errors should be reported together for efficient fixing
    let (temp_dir, workspace) = schema_workspace(
        r#"{
        "ingredients": [{
            "name": "multi_error_ingredient",
            "display_name": "Multi Error Ingredient",
            "carbs_per_100g": -10,
            "protein_per_100g": 200,
            "fat_per_100g": -5,
            "fiber_per_100g": 150
        }]
    }"#,
        None,
    );

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("schema_multiple_errors", normalized_stderr);
}

#[test]
fn test_with_valid_boundary_values() {
    // Edge case: boundary values (0 and 100) should be valid to avoid false rejections
    let (_temp_dir, workspace) = schema_workspace(
        r#"{
        "ingredients": [{
            "name": "boundary_ingredient",
            "display_name": "Boundary Test Ingredient",
            "carbs_per_100g": 0,
            "protein_per_100g": 100,
            "fat_per_100g": 0,
            "fiber_per_100g": 100
        }]
    }"#,
        Some(
            r#"{
        "recipes": [{
            "name": "boundary-recipe",
            "ingredients": [{
                "name": "boundary_ingredient",
                "grams": 0.1
            }]
        }]
    }"#,
        ),
    );

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&workspace)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content = format_list_test_snapshot(&["boundary-recipe"], &[], &stdout);
    assert_snapshot!("schema_valid_boundary_values", snapshot_content);
}
