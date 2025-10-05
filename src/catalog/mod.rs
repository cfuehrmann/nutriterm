pub mod items;
mod storage;

use crate::error::{AppResult, LoadError};
use std::path::{Path, PathBuf};

// Domain layer functions (orchestrate storage layer)

/// Find the catalog directory
pub fn find_dir() -> AppResult<PathBuf> {
    storage::find_dir()
}

/// Initialize a complete catalog
pub fn initialize(path: &Path) -> AppResult<()> {
    storage::initialize(path)
}

/// Load recipes from catalog
pub fn load_recipes(path: &Path) -> Result<Vec<items::Recipe>, LoadError> {
    storage::load_recipes(path)
}
