// Integration tests focused on general CLI behavior and error handling
// Each test represents one complete user action (vertically sliced)
use assert_cmd::Command;
use insta::assert_snapshot;
use std::fs;

use tempfile::TempDir;

mod common;
use common::normalize_temp_paths;

#[test]
fn test_user_requests_help() {
    // User runs --help to understand available commands
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .arg("--help")
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_snapshot!("help_output", stdout);
}

#[test]
fn test_user_runs_without_command() {
    // User runs command without arguments and gets usage guidance
    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_snapshot!("no_subcommand_error", stderr);
}

#[test]
fn test_improved_file_error_unreadable_recipes_file() {
    let temp_dir = TempDir::new().unwrap();

    // Test error handling when workspace files are corrupted by external tools
    let workspace = temp_dir.path().join("test-workspace");
    fs::create_dir_all(&workspace).unwrap();
    std::fs::write(
        workspace.join("ingredients.jsonc"),
        r#"{"ingredients": []}"#,
    )
    .unwrap();
    fs::create_dir_all(workspace.join("recipes.jsonc")).unwrap();

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("improved_file_error_unreadable_recipes", normalized_stderr);
}

#[test]
fn test_improved_file_error_unreadable_ingredients_file() {
    let temp_dir = TempDir::new().unwrap();

    // Test error handling when workspace files are corrupted by external tools
    let workspace = temp_dir.path().join("test-workspace");
    fs::create_dir_all(&workspace).unwrap();
    std::fs::write(workspace.join("recipes.jsonc"), r#"{"recipes": []}"#).unwrap();
    fs::create_dir_all(workspace.join("ingredients.jsonc")).unwrap();

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["list-recipes"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!(
        "improved_file_error_unreadable_ingredients",
        normalized_stderr
    );
}

#[test]
fn test_early_return_error_propagation_consistency() {
    let temp_dir = TempDir::new().unwrap();

    // Ensures error message consistency across commands for better user experience
    let workspace = temp_dir.path().join("test-workspace");
    fs::create_dir_all(&workspace).unwrap();
    std::fs::write(
        workspace.join("ingredients.jsonc"),
        r#"{"ingredients": []}"#,
    )
    .unwrap();
    std::fs::write(workspace.join("recipes.jsonc"), "{ invalid json").unwrap();

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["recipe", "test"])
        .current_dir(&workspace)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("early_return_recipe_error_format", normalized_stderr);
}
