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
