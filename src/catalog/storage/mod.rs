mod initialization;
mod loader;

// Storage operations (implementation layer)
pub use initialization::initialize;
pub use loader::load_recipes;
