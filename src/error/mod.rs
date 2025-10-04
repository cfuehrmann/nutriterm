mod app_error;
mod load_error;

pub use app_error::AppError;
pub use load_error::{DuplicateGroup, LoadError};

pub type AppResult<T> = Result<T, AppError>;