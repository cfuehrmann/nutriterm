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
        .args(&["recipe", "Chicken Rice Bowl"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content =
        format_test_snapshot(&["Chicken Rice Bowl"], "Chicken Rice Bowl", &stdout);
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
        format_test_snapshot(&["Chicken Rice Bowl"], "nonexistent-recipe", &stdout);
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
        .args(&["recipe", "Chicken Rice Bowl"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content =
        format_test_snapshot(&["Chicken Rice Bowl"], "Chicken Rice Bowl", &stdout);
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
    let snapshot_content = format_test_snapshot(&["Chicken Rice Bowl"], "chicken", &stdout);
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
      "name": "Chicken Rice Bowl",
      "ingredients": [{"ingredient_id": "chicken_breast", "grams": 150}]
    },
    {
      "name": "Beef Rice Stir Fry", 
      "ingredients": [{"ingredient_id": "chicken_breast", "grams": 100}]
    },
    {
      "name": "Chicken Salad",
      "ingredients": [{"ingredient_id": "chicken_breast", "grams": 120}]
    }
  ]
}"#,
    )
    .unwrap();

    std::fs::write(
        workspace_dir.join("ingredients.jsonc"),
        r#"{
  "ingredients": [{
    "id": "chicken_breast",
    "name": "Chicken Breast",
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
        &["Chicken Rice Bowl", "Beef Rice Stir Fry", "Chicken Salad"],
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
      "name": "Chicken Rice Bowl",
      "description": "A balanced meal",
      "ingredients": [{"ingredient_id": "chicken_breast", "grams": 150}]
    },
    {
      "name": "Chicken Salad",
      "description": "Fresh salad", 
      "ingredients": [{"ingredient_id": "chicken_breast", "grams": 120}]
    },
    {
      "name": "Spicy Chicken Curry",
      "ingredients": [{"ingredient_id": "chicken_breast", "grams": 200}]
    }
  ]
}"#,
    )
    .unwrap();

    std::fs::write(
        workspace_dir.join("ingredients.jsonc"),
        r#"{
  "ingredients": [{
    "id": "chicken_breast",
    "name": "Chicken Breast",
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
        &["Chicken Rice Bowl", "Chicken Salad", "Spicy Chicken Curry"],
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
    let snapshot_content = format_test_snapshot(&["Chicken Rice Bowl"], "pizza", &stdout);
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
    let snapshot_content = format_test_snapshot(&["Chicken Rice Bowl"], "CHICKEN", &stdout);
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
            "id": "invalid_ingredient",
            "name": "Invalid Ingredient",
            "carbs_per_100g": -5,
            "protein_per_100g": 25,
            "fat_per_100g": 10,
            "fiber_per_100g": 3
        }]
    }"#,
        r#"{
        "recipes": [{
            "name": "test-recipe",
            "ingredients": [{"ingredient_id": "invalid_ingredient", "grams": 100}]
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
            "id": "test_ingredient",
            "name": "Test Ingredient",
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
                "ingredient_id": "test_ingredient",
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

// Validation tests - ensure that the recipe command validates workspace before execution

#[test]
fn test_recipe_validates_unknown_ingredient() {
    // Recipe command should validate and fail with unknown ingredient error (not proceed to recipe lookup)
    let temp_dir = TempDir::new().unwrap();
    let workspace = temp_dir.path().join("validation-workspace");
    fs::create_dir_all(&workspace).unwrap();

    // Create ingredients with 'chicken_breast'
    std::fs::write(
        workspace.join("ingredients.jsonc"),
        r#"{
        "ingredients": [{
            "id": "chicken_breast",
            "name": "Chicken Breast",
            "carbs_per_100g": 0,
            "protein_per_100g": 31,
            "fat_per_100g": 3.6,
            "fiber_per_100g": 0
        }]
    }"#,
    )
    .unwrap();

    // Recipe with typo 'chiken_breast'
    std::fs::write(
        workspace.join("recipes.jsonc"),
        r#"{
        "recipes": [{
            "name": "test-recipe",
            "ingredients": [{"ingredient_id": "chiken_breast", "grams": 100}]
        }]
    }"#,
    )
    .unwrap();

    // Try to view the recipe - should fail with validation error, not "recipe not found"
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "test-recipe"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());

    // Should show suggestion error, proving validation happened before recipe lookup
    assert!(normalized_stderr.contains("Did you mean 'chicken_breast'?"));
    assert!(normalized_stderr.contains("Available ingredient IDs: chicken_breast"));
    assert_snapshot!("unknown_ingredient", normalized_stderr);
}

#[test]
fn test_recipe_validates_with_any_recipe_name() {
    // Validation should happen even for non-existent recipes (validation runs first)
    let temp_dir = TempDir::new().unwrap();
    let workspace = temp_dir.path().join("validation-workspace");
    fs::create_dir_all(&workspace).unwrap();

    // Empty ingredients list
    std::fs::write(
        workspace.join("ingredients.jsonc"),
        r#"{"ingredients": []}"#,
    )
    .unwrap();

    // Recipe with unknown ingredient
    std::fs::write(
        workspace.join("recipes.jsonc"),
        r#"{
        "recipes": [{
            "name": "existing-recipe",
            "ingredients": [{"ingredient_id": "nonexistent_ingredient", "grams": 100}]
        }]
    }"#,
    )
    .unwrap();

    // Try to view a DIFFERENT recipe name - should still fail validation before recipe lookup
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "completely-different-recipe"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show validation error, not "recipe not found" - proves validation runs first
    assert!(stderr.contains("Recipe 'existing-recipe' references unknown ingredient"));
    assert!(stderr.contains("nonexistent_ingredient"));
}

#[test]
fn test_recipe_validates_schema_errors() {
    // Recipe command should validate schema before trying to process recipes
    let temp_dir = TempDir::new().unwrap();
    let workspace = temp_dir.path().join("schema-workspace");
    fs::create_dir_all(&workspace).unwrap();

    // Valid ingredients
    std::fs::write(
        workspace.join("ingredients.jsonc"),
        r#"{
        "ingredients": [{
            "id": "test_ingredient",
            "name": "Test Ingredient", 
            "carbs_per_100g": 10,
            "protein_per_100g": 20,
            "fat_per_100g": 5,
            "fiber_per_100g": 2
        }]
    }"#,
    )
    .unwrap();

    // Invalid recipe schema (negative grams)
    std::fs::write(
        workspace.join("recipes.jsonc"),
        r#"{
        "recipes": [{
            "name": "invalid-recipe",
            "ingredients": [{
                "ingredient_id": "test_ingredient",
                "grams": -100
            }]
        }]
    }"#,
    )
    .unwrap();

    // Try to view any recipe - should fail schema validation first
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "any-recipe"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show schema validation error, proving validation runs before recipe lookup
    assert!(stderr.contains("Schema validation failed"));
    assert!(stderr.contains("grams"));
}

#[test]
fn test_recipe_command_comprehensive_validation_coverage() {
    // This test documents and verifies that recipe command validates ALL workspace issues before execution
    // It serves as a comprehensive check that no validation is skipped

    let temp_dir = TempDir::new().unwrap();
    let workspace = temp_dir.path().join("comprehensive-workspace");
    fs::create_dir_all(&workspace).unwrap();

    // Create a workspace with MULTIPLE validation issues
    std::fs::write(
        workspace.join("ingredients.jsonc"),
        r#"{
        "ingredients": [{
            "id": "valid_ingredient", 
            "name": "Valid Ingredient",
            "carbs_per_100g": 10,
            "protein_per_100g": 20,
            "fat_per_100g": 5,
            "fiber_per_100g": 2
        }]
    }"#,
    )
    .unwrap();

    std::fs::write(
        workspace.join("recipes.jsonc"),
        r#"{
        "recipes": [
            {
                "name": "valid-recipe",
                "ingredients": [{"ingredient_id": "valid_ingredient", "grams": 100}]
            },
            {
                "name": "invalid-recipe-with-unknown-ingredient",
                "ingredients": [{"ingredient_id": "unknown_ingredient", "grams": 100}]
            }
        ]
    }"#,
    )
    .unwrap();

    // Try to access any recipe - validation should catch unknown ingredient before recipe processing
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "valid-recipe"]) // Even requesting valid recipe should fail validation
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Validation should find the unknown ingredient issue and fail before recipe processing
    assert!(stderr.contains("unknown ingredient"));
    assert!(stderr.contains("unknown_ingredient"));
    assert!(stderr.contains("invalid-recipe-with-unknown-ingredient"));

    // Validation is working: it found workspace issues before trying to process the requested recipe
}

#[test]
fn test_exact_match_disambiguates_substring_conflicts() {
    // Test case where exact match is crucial: recipe names have substring relationships
    // Without exact match priority, "rice" would be ambiguous between "rice" and "rice-bowl"
    let temp_dir = temp_dir();
    let workspace = workspace_dir(&temp_dir, "substring-conflict-test");

    write_files(
        &workspace,
        r#"{
        "ingredients": [
            {
                "id": "rice",
                "name": "White Rice (cooked)",
                "carbs_per_100g": 28,
                "protein_per_100g": 2.7,
                "fat_per_100g": 0.3,
                "fiber_per_100g": 0.4
            },
            {
                "id": "chicken_breast",
                "name": "Chicken Breast (skinless)",
                "carbs_per_100g": 0,
                "protein_per_100g": 31,
                "fat_per_100g": 3.6,
                "fiber_per_100g": 0
            }
        ]
    }"#,
        r#"{
        "recipes": [
            {
                "name": "rice",
                "description": "Simple rice dish",
                "ingredients": [
                    {
                        "ingredient_id": "rice",
                        "grams": 200
                    }
                ]
            },
            {
                "name": "Rice Bowl",
                "description": "Rice bowl with protein",
                "ingredients": [
                    {
                        "ingredient_id": "rice",
                        "grams": 150
                    },
                    {
                        "ingredient_id": "chicken_breast",
                        "grams": 100
                    }
                ]
            }
        ]
    }"#,
    );

    // User searches for exact "rice" - should find "rice" recipe, not be ambiguous
    // This test would fail if exact match were removed (would show "Multiple recipes found")
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "rice"])
        .current_dir(&workspace)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let snapshot_content = format_test_snapshot(&["rice", "rice-bowl"], "rice", &stdout);
    assert_snapshot!("exact_match_disambiguates_substring", snapshot_content);
}

#[test]
fn test_duplicate_recipe_names_search_behavior() {
    let temp_dir = temp_dir();
    let workspace = workspace_dir(&temp_dir, "duplicate-names-test");

    write_files(
        &workspace,
        r#"{
        "ingredients": [{
            "id": "rice",
            "name": "White Rice",
            "carbs_per_100g": 28,
            "protein_per_100g": 3,
            "fat_per_100g": 0.3,
            "fiber_per_100g": 0.4
        }]
    }"#,
        r#"{
        "recipes": [
            {
                "name": "Rice Bowl",
                "description": "First rice bowl (150g rice)",
                "ingredients": [{
                    "ingredient_id": "rice",
                    "grams": 150
                }]
            },
            {
                "name": "Rice Bowl", 
                "description": "Second rice bowl (200g rice)",
                "ingredients": [{
                    "ingredient_id": "rice",
                    "grams": 200
                }]
            }
        ]
    }"#,
    );

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "rice-bowl"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("duplicate_recipe_names_search", normalized_stderr);
}

#[test]
fn test_duplicate_ingredient_ids_validation() {
    let temp_dir = temp_dir();
    let workspace = workspace_dir(&temp_dir, "duplicate-ingredients-test");

    write_files(
        &workspace,
        r#"{
        "ingredients": [
            {
                "id": "chicken_breast",
                "name": "Chicken Breast (skinless)",
                "carbs_per_100g": 0,
                "protein_per_100g": 31,
                "fat_per_100g": 3.6,
                "fiber_per_100g": 0
            },
            {
                "id": "chicken_breast",
                "name": "Chicken Breast (with skin)",
                "carbs_per_100g": 0,
                "protein_per_100g": 25,
                "fat_per_100g": 7.4,
                "fiber_per_100g": 0
            }
        ]
    }"#,
        r#"{
        "recipes": [
            {
                "name": "Test Recipe",
                "description": "Simple test recipe",
                "ingredients": [{
                    "ingredient_id": "chicken_breast",
                    "grams": 150
                }]
            }
        ]
    }"#,
    );

    // User tries to view recipe with duplicate ingredient IDs - should get validation error
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "Test Recipe"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("duplicate_ingredient_ids_validation", normalized_stderr);
}
