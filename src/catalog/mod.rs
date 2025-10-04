mod discovery;
pub mod items;
mod loader;
pub mod schema;

pub use discovery::{find_catalog_dir, is_catalog_dir};
pub use loader::load_recipes;
