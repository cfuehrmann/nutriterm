mod discovery;
pub mod items;
mod jsonc;

use crate::error::AppResult;
use std::path::Path;

// Domain layer functions (orchestrate discovery and JSONC implementation)

/// Initialize a complete catalog
pub fn initialize(path: &Path) -> AppResult<()> {
    jsonc::initialize(path)
}

/// Load recipes from catalog
pub fn load_recipes() -> AppResult<Vec<items::Recipe>> {
    let catalog_dir = discovery::find_dir()?;
    jsonc::load_recipes(&catalog_dir)
}
