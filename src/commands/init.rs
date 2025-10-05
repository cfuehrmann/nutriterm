use crate::error::{AppError, AppResult};
use std::path::Path;

pub fn run(path: &Path) -> AppResult<()> {
    if !is_empty_or_safe_to_initialize(path)? {
        let message = format!(
            "Directory '{}' is not empty. Please run init in an empty directory.",
            path.display()
        );
        return Err(AppError::DirectoryNotEmpty {
            path: path.to_path_buf(),
            message,
        });
    }

    std::fs::create_dir_all(path)?;
    crate::catalog::initialize(path)?;

    println!("✅ Initialized recipe catalog in {}", path.display());
    println!("📄 Created schemas, recipes, and ingredients files");
    println!("🍽️  Ready to use!");
    Ok(())
}

fn is_empty_or_safe_to_initialize(path: &Path) -> AppResult<bool> {
    if !path.exists() {
        return Ok(true);
    }

    let entries = std::fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name();
        let filename_str = filename.to_string_lossy();

        if filename_str.starts_with('.') {
            continue;
        }

        return Ok(false);
    }

    Ok(true)
}
