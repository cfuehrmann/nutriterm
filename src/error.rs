use std::path::PathBuf;

#[derive(Debug)]
pub struct DuplicateGroup {
    pub key: String,
    pub items: Vec<String>,
}

#[derive(Debug)]
pub enum LoadError {
    FileError {
        path: PathBuf,
        source: std::io::Error,
    },
    ParseError {
        filename: String,
        message: String,
    },
    ValidationError {
        filename: String,
        message: String,
    },
    SchemaValidationError {
        filename: String,
        errors: Vec<String>,
    },
    DuplicateKeyError {
        filename: String,
        key_type: String,
        duplicates: Vec<DuplicateGroup>,
    },
    UnknownIngredientError {
        recipe: String,
        ingredient: String,
        suggestion: Option<String>,
        available_ids: Vec<String>,
    },
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::FileError { path, source } => {
                write!(
                    f,
                    "Cannot read file {}: {}\n\nTip: Make sure the file exists and you have read permissions. Run 'nutriterm init' to create missing workspace files.",
                    path.display(),
                    source
                )
            }
            LoadError::ParseError { filename, message } => {
                write!(
                    f,
                    "Invalid JSONC syntax in {}: {}\n\nTip: Check for missing commas, brackets, or quotes. Most editors highlight syntax errors when you save the file with a .jsonc extension.",
                    filename, message
                )
            }
            LoadError::ValidationError { filename, message } => {
                write!(f, "Invalid {} structure: {}", filename, message)
            }
            LoadError::SchemaValidationError { filename, errors } => {
                write!(
                    f,
                    "Schema validation failed for {}:\n{}\n\nTip: Check the values against the expected data types and ranges. Use 'nutriterm init' to see example file formats.",
                    filename,
                    errors.join("\n")
                )
            }
            LoadError::DuplicateKeyError {
                filename,
                key_type,
                duplicates,
            } => {
                let duplicate_descriptions: Vec<String> = duplicates
                    .iter()
                    .map(|group| {
                        let item_list = group
                            .items
                            .iter()
                            .map(|item| format!("  - {}", item))
                            .collect::<Vec<_>>()
                            .join("\n");
                        format!(
                            "Duplicate {} '{}' found:\n{}",
                            key_type, group.key, item_list
                        )
                    })
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
            LoadError::UnknownIngredientError {
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
        }
    }
}

impl std::error::Error for LoadError {}

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

pub type AppResult<T> = Result<T, AppError>;
