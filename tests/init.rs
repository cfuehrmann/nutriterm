use assert_cmd::Command;
use insta::assert_snapshot;
use std::fs;
use tempfile::TempDir;

mod common;
use common::normalize_temp_paths;

#[test]
fn test_init_in_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    // User successfully initializes workspace in empty directory
    let workspace_dir = temp_dir.path().join("recipes");
    fs::create_dir_all(&workspace_dir).unwrap();

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["init"])
        .current_dir(&workspace_dir)
        .assert()
        .success();

    let output = assert.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let normalized_stdout = normalize_temp_paths(&stdout, temp_dir.path());
    assert_snapshot!("success", normalized_stdout);

    // Verify all expected files were created
    assert!(workspace_dir.join("recipes.schema.json").exists());
    assert!(workspace_dir.join("ingredients.schema.json").exists());
    assert!(workspace_dir.join("recipes.jsonc").exists());
    assert!(workspace_dir.join("ingredients.jsonc").exists());

    // Snapshot the content of created files to ensure they're properly formatted
    let ingredients_content =
        std::fs::read_to_string(workspace_dir.join("ingredients.jsonc")).unwrap();
    assert_snapshot!("ingredients_content", ingredients_content);

    let recipes_content = std::fs::read_to_string(workspace_dir.join("recipes.jsonc")).unwrap();
    assert_snapshot!("recipes_content", recipes_content);

    let ingredients_schema =
        std::fs::read_to_string(workspace_dir.join("ingredients.schema.json")).unwrap();
    assert_snapshot!("ingredients_schema", ingredients_schema);

    let recipes_schema =
        std::fs::read_to_string(workspace_dir.join("recipes.schema.json")).unwrap();
    assert_snapshot!("recipes_schema", recipes_schema);
}

#[test]
fn test_init_in_non_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    // User tries to initialize in non-empty directory and gets helpful error
    let non_empty_dir = temp_dir.path().join("non-empty");
    fs::create_dir_all(&non_empty_dir).unwrap();
    fs::write(non_empty_dir.join("existing_file.txt"), "content").unwrap();

    let assert = Command::cargo_bin("nutriterm")
        .unwrap()
        .args(&["init"])
        .current_dir(&non_empty_dir)
        .assert()
        .failure();

    let output = assert.get_output();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let normalized_stderr = normalize_temp_paths(&stderr, temp_dir.path());
    assert_snapshot!("directory_not_empty", normalized_stderr);

    // Verify no files were created in the non-empty directory
    assert!(!non_empty_dir.join("recipes.jsonc").exists());
    assert!(!non_empty_dir.join("ingredients.jsonc").exists());
}
