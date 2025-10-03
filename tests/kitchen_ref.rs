mod common;

use assert_cmd::Command;
use common::{normalize_temp_paths, temp_dir, workspace_dir};
use insta::assert_snapshot;
use std::fs;

#[test]
fn test_kitchen_ref_outside_workspace() {
    let temp = temp_dir();
    let outside_dir = workspace_dir(&temp, "outside");

    let mut cmd = Command::cargo_bin("nutriterm").unwrap();
    let assert = cmd
        .current_dir(&outside_dir)
        .arg("kitchen-ref")
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp.path());
    assert_snapshot!("no_workspace", normalized_stderr);
}

fn create_kitchen_ref_workspace(workspace_dir: &std::path::Path) {
    fs::write(
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
    },
    {
      "name": "Greek Salad",
      "ingredients": [
        {
          "id": "mixed_greens",
          "grams": 100
        },
        {
          "id": "feta_cheese",
          "grams": 50
        },
        {
          "id": "cherry_tomatoes",
          "grams": 75
        },
        {
          "id": "cucumber",
          "grams": 60
        }
      ]
    }
  ]
}"#,
    )
    .unwrap();

    fs::write(
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
    },
    {
      "id": "mixed_greens",
      "name": "Mixed Greens",
      "carbs_per_100g": 2,
      "protein_per_100g": 1.5,
      "fat_per_100g": 0.2,
      "fiber_per_100g": 1.2
    },
    {
      "id": "feta_cheese",
      "name": "Feta Cheese",
      "carbs_per_100g": 4,
      "protein_per_100g": 14,
      "fat_per_100g": 21,
      "fiber_per_100g": 0
    },
    {
      "id": "cherry_tomatoes",
      "name": "Cherry Tomatoes",
      "carbs_per_100g": 4,
      "protein_per_100g": 0.9,
      "fat_per_100g": 0.2,
      "fiber_per_100g": 1.2
    },
    {
      "id": "cucumber",
      "name": "Cucumber",
      "carbs_per_100g": 4,
      "protein_per_100g": 0.7,
      "fat_per_100g": 0.1,
      "fiber_per_100g": 0.5
    }
  ]
}"#,
    )
    .unwrap();
}

#[test]
fn test_kitchen_ref_success() {
    let temp = temp_dir();
    let workspace = workspace_dir(&temp, "workspace");
    create_kitchen_ref_workspace(&workspace);

    let mut cmd = Command::cargo_bin("nutriterm").unwrap();
    let assert = cmd
        .current_dir(&workspace)
        .arg("kitchen-ref")
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_snapshot!("output", stdout);
}
