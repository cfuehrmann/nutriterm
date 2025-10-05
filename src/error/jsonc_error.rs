/// JSONC format-specific errors
#[derive(Debug)]
pub enum JsoncError {
    ParseError {
        filename: String,
        message: String,
    },
    ProcessingError {
        filename: String,
        message: String,
    },
    SchemaViolationError {
        filename: String,
        errors: Vec<String>,
    },
}

impl std::fmt::Display for JsoncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsoncError::ParseError { filename, message } => {
                write!(
                    f,
                    "Invalid JSONC syntax in {}: {}\n\nTip: Check for missing commas, brackets, or quotes. Most editors highlight syntax errors when you save the file with a .jsonc extension.",
                    filename, message
                )
            }
            JsoncError::ProcessingError { filename, message } => {
                write!(f, "Invalid {} structure: {}", filename, message)
            }
            JsoncError::SchemaViolationError { filename, errors } => {
                write!(
                    f,
                    "Schema validation failed for {}:\n{}\n\nTip: Check the values against the expected data types and ranges. Use 'nutriterm init' to see example file formats.",
                    filename,
                    errors.join("\n")
                )
            }
        }
    }
}

impl std::error::Error for JsoncError {}
