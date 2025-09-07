use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;
use tempfile::TempDir;

/// Helper function to strip ANSI color codes from test output for clean snapshots
pub fn strip_ansi_codes(output: &str) -> String {
    let mut result = String::new();
    let mut chars = output.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            // Skip the escape sequence
            chars.next(); // consume '['
            while let Some(seq_char) = chars.next() {
                if seq_char.is_ascii_alphabetic() {
                    break; // End of escape sequence
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Helper function to normalize temp paths in test output for stable snapshots
#[allow(dead_code)]
pub fn normalize_temp_paths(output: &str, temp_dir: &Path) -> String {
    let temp_path = temp_dir.to_str().unwrap();
    output
        .replace(temp_path, "[TEMP_DIR]")
        .replace(&format!("{}/", temp_path), "[TEMP_DIR]/")
}

/// Helper function to format test snapshots with database context and command
#[allow(dead_code)] // Used in recipe.rs tests, but Rust can't track cross-module test usage
pub fn format_test_snapshot(recipes: &[&str], command: &str, output: &str) -> String {
    format!(
        "Available recipes: {}\n$ nutriterm recipe {}\n{}",
        recipes.join(", "),
        command,
        strip_ansi_codes(output).trim_end()
    )
}



/// Create a temp directory for testing
#[allow(dead_code)]
pub fn temp_dir() -> TempDir {
    TempDir::new().unwrap()
}

/// Create a workspace directory within a temp directory
#[allow(dead_code)]
pub fn workspace_dir(temp_dir: &TempDir, name: &str) -> PathBuf {
    let workspace_dir = temp_dir.path().join(name);
    fs::create_dir_all(&workspace_dir).unwrap();
    workspace_dir
}

/// Run nutriterm command with args in a directory
#[allow(dead_code)]
pub fn run_cmd(args: &[&str], working_dir: &Path) -> Output {
    Command::cargo_bin("nutriterm")
        .unwrap()
        .args(args)
        .current_dir(working_dir)
        .output()
        .unwrap()
}

/// Write test files for schema validation tests
#[allow(dead_code)]
pub fn write_files(workspace: &Path, ingredients_content: &str, recipes_content: &str) {
    fs::write(workspace.join("ingredients.jsonc"), ingredients_content).unwrap();
    fs::write(workspace.join("recipes.jsonc"), recipes_content).unwrap();
}

/// Helper function to manually create workspace files (without running init command)
#[allow(dead_code)] // Used across multiple test modules, but Rust can't track cross-module test usage
pub fn create_workspace_files(workspace_dir: &std::path::Path) {
    // Avoids running init command in test helpers to prevent coupling test setup to command implementation
    std::fs::write(
        workspace_dir.join("recipes.jsonc"),
        r#"{
  "$schema": "./recipes.schema.json",
  
  "recipes": [
    {
      "name": "Chicken Rice Bowl",

      "ingredients": [
        {
          "id": "chicken_breast",
          "grams": 150
        },
        {
          "id": "brown_rice",
          "grams": 100
        },
        {
          "id": "broccoli",
          "grams": 80
        }
      ]
    }
  ]
}"#,
    )
    .unwrap();

    std::fs::write(
        workspace_dir.join("ingredients.jsonc"),
        r#"{
  "$schema": "./ingredients.schema.json",
  
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
      "id": "brown_rice",
      "name": "Brown Rice (cooked)",
      "carbs_per_100g": 23,
      "protein_per_100g": 2.6,
      "fat_per_100g": 0.9,
      "fiber_per_100g": 1.8
    },
    {
      "id": "broccoli",
      "name": "Broccoli (steamed)",
      "carbs_per_100g": 7,
      "protein_per_100g": 3,
      "fat_per_100g": 0.4,
      "fiber_per_100g": 2.6
    }
  ]
}"#,
    )
    .unwrap();
}
