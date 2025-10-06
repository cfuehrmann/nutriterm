use std::path::PathBuf;

#[derive(Debug)]
pub struct DuplicateGroup {
    pub key: String,
    pub items: Vec<String>,
}

// Errors not specific to the storage format
#[derive(Debug)]
pub enum AppError {
    CatalogNotFound {
        searched: Vec<PathBuf>,
        message: String,
    },
    DirectoryNotEmpty {
        path: PathBuf,
        message: String,
    },
    UnknownIngredient {
        recipe: String,
        ingredient: String,
        suggestion: Option<String>,
        available_ids: Vec<String>,
    },
    DuplicateKey {
        filename: String,
        key_type: String,
        duplicates: Vec<DuplicateGroup>,
    },
    FileUnreadable {
        path: PathBuf,
        io_error: String,
    },
    ParsingError {
        message: String,
    },
    SchemaComplianceError {
        message: String,
    },
    InvalidSchema {
        message: String,
    },
    TypeMappingError {
        message: String,
    },

    Io(std::io::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::CatalogNotFound { message, .. }
            | AppError::DirectoryNotEmpty { message, .. }
            | AppError::ParsingError { message, .. }
            | AppError::SchemaComplianceError { message, .. }
            | AppError::InvalidSchema { message, .. }
            | AppError::TypeMappingError { message, .. } => write!(f, "{}", message),

            AppError::FileUnreadable { path, io_error } => {
                write!(f, "Cannot read file {}: {}", path.display(), io_error)
            }

            AppError::UnknownIngredient {
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

            AppError::DuplicateKey {
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

            // Legacy variants
            AppError::Io(error) => write!(f, "{}", error),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl std::error::Error for AppError {}
