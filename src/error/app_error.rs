use super::load_error::LoadError;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppError {
    Data(LoadError),
    DirectoryNotEmpty {
        path: PathBuf,
        message: String,
    },
    WorkspaceNotFound {
        searched: Vec<PathBuf>,
        message: String,
    },
    RecipeNotFound {
        name: String,
        available: Vec<String>,
        message: String,
    },

    Other(String),
    Io(std::io::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Data(error) => write!(f, "{}", error),
            AppError::Io(error) => write!(f, "{}", error),
            AppError::DirectoryNotEmpty { message, .. }
            | AppError::WorkspaceNotFound { message, .. }
            | AppError::RecipeNotFound { message, .. }
            | AppError::Other(message) => write!(f, "{}", message),
        }
    }
}

impl From<LoadError> for AppError {
    fn from(err: LoadError) -> Self {
        AppError::Data(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl std::error::Error for AppError {}