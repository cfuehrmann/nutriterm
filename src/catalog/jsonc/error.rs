/// JSONC format-specific errors (internal to catalog/jsonc module)
#[derive(Debug)]
pub(super) enum JsoncError {
    Parsing {
        filename: String,
        message: String,
    },
    Deserializing {
        filename: String,
        message: String,
    },
    SchemaValidation {
        filename: String,
        errors: Vec<String>,
    },
}

impl std::fmt::Display for JsoncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSONC format error")
    }
}

impl std::error::Error for JsoncError {}
