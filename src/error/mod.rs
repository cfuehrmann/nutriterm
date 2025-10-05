mod app_error;

pub use app_error::{AppError, DuplicateGroup};

pub type AppResult<T> = Result<T, AppError>;
