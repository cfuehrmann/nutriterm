use crate::error::{AppError, AppResult};
use std::path::{Path, PathBuf};

/// Find the catalog directory by searching current directory and parent directories
pub fn find_catalog_dir() -> AppResult<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let mut searched = vec![current_dir.clone()];

    if is_catalog_dir(&current_dir) {
        return Ok(current_dir);
    }

    let mut dir = current_dir.as_path();
    while let Some(parent) = dir.parent() {
        searched.push(parent.to_path_buf());
        if is_catalog_dir(parent) {
            return Ok(parent.to_path_buf());
        }
        dir = parent;
    }

    let message = format!(
        "Not in a nutrition calculator catalog (or any parent directory)\n\
         Searched: {}\n\
         Run 'nutriterm init' to create a catalog.",
        searched
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    Err(AppError::CatalogNotFound { searched, message })
}

/// Check if a directory is a valid catalog (contains both required files)
pub fn is_catalog_dir(path: &Path) -> bool {
    path.join("ingredients.jsonc").exists() && path.join("recipes.jsonc").exists()
}
