mod discovery;
mod initialization;
mod loader;

// Storage operations (implementation layer)
pub use discovery::find_dir;
pub use initialization::initialize;
pub use loader::load_recipes;
