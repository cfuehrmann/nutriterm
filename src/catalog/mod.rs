pub mod items;
mod storage;

use crate::error::AppResult;
use std::path::Path;

// Domain layer functions (orchestrate storage layer)

/// Initialize a complete catalog
pub fn initialize(path: &Path) -> AppResult<()> {
    storage::initialize(path)
}

/// Load recipes from catalog
pub fn load_recipes() -> AppResult<Vec<items::Recipe>> {
    storage::load_recipes().map_err(Into::into)
}
