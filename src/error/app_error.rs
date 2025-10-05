use super::{DuplicateGroup, JsoncError, LoadError};
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppError {
    // Domain business rules
    CatalogNotFound {
        searched: Vec<PathBuf>,
        message: String,
    },
    RecipeNotFound {
        name: String,
        available: Vec<String>,
        message: String,
    },
    DirectoryNotEmpty {
        path: PathBuf,
        message: String,
    },
    UnknownIngredientError {
        recipe: String,
        ingredient: String,
        suggestion: Option<String>,
        available_ids: Vec<String>,
    },
    DuplicateKeyError {
        filename: String,
        key_type: String,
        duplicates: Vec<DuplicateGroup>,
    },

    // Infrastructure errors
    FileError {
        path: PathBuf,
        source: std::io::Error,
    },

    // Format-specific wrapper
    FormatError(JsoncError),

    // Legacy/other
    Data(LoadError),
    Other(String),
    Io(std::io::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::CatalogNotFound { message, .. }
            | AppError::RecipeNotFound { message, .. }
            | AppError::DirectoryNotEmpty { message, .. } => write!(f, "{}", message),

            AppError::UnknownIngredientError {
                recipe,
                ingredient,
                suggestion,
                available_ids,
            } => {
                write!(
                    f,
                    "Recipe '{}' references unknown ingredient '{}'",
                    recipe, ingredient
                )?;

                if let Some(suggested) = suggestion {
                    write!(f, ".\n\nDid you mean '{}'?", suggested)?;
                }

                if !available_ids.is_empty() {
                    write!(
                        f,
                        "\n\nAvailable ingredient IDs: {}",
                        available_ids.join(", ")
                    )?;
                }

                write!(
                    f,
                    "\n\nTip: Fix ingredient references in recipes.jsonc before running commands."
                )
            }

            AppError::DuplicateKeyError {
                filename,
                key_type,
                duplicates,
            } => {
                let duplicate_descriptions: Vec<String> = duplicates
                    .iter()
                    .map(|group| format!("Duplicate {} '{}' found!", key_type, group.key))
                    .collect();

                write!(
                    f,
                    "Duplicate {} found in {}:\n{}\n\nTip: Each {} must be unique. Rename the duplicates to use different values.",
                    key_type,
                    filename,
                    duplicate_descriptions.join("\n\n"),
                    key_type
                )
            }

            AppError::FileError { path, source } => {
                write!(
                    f,
                    "Cannot read file {}: {}\n\nTip: Make sure the file exists and you have read permissions. Run 'nutriterm init' to create missing catalog files.",
                    path.display(),
                    source
                )
            }

            AppError::FormatError(jsonc_error) => write!(f, "{}", jsonc_error),

            // Legacy variants
            AppError::Data(error) => write!(f, "{}", error),
            AppError::Io(error) => write!(f, "{}", error),
            AppError::Other(message) => write!(f, "{}", message),
        }
    }
}

impl From<LoadError> for AppError {
    fn from(err: LoadError) -> Self {
        AppError::Data(err)
    }
}

impl From<JsoncError> for AppError {
    fn from(err: JsoncError) -> Self {
        AppError::FormatError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl std::error::Error for AppError {}
