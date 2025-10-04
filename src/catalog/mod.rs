mod discovery;
pub mod items;
mod loader;
mod schema;

pub use discovery::{find_catalog_dir, is_catalog_dir};
pub use loader::load_recipes;
pub use schema::{generate_all_schemas, generate_ingredient_schema, generate_recipe_schema};
