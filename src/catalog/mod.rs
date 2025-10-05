mod discovery;
pub mod items;
mod loader;
mod schema;

pub use discovery::{find_catalog_dir, is_catalog_dir};
pub use loader::load_recipes;
pub use schema::{
    create_example_files, generate_all_schemas, generate_ingredient_schema, generate_recipe_schema,
    get_ingredient_template, get_recipe_template,
};
