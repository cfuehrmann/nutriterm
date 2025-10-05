use crate::error::{AppError, AppResult};
use std::path::Path;

pub fn run() -> AppResult<()> {
    let current_dir = std::env::current_dir()?;

    if !is_empty_or_safe_to_initialize(&current_dir)? {
        let message = format!(
            "Directory '{}' is not empty. Please run init in an empty directory.",
            current_dir.display()
        );
        return Err(AppError::DirectoryNotEmpty {
            path: current_dir,
            message,
        });
    }

    std::fs::create_dir_all(&current_dir)?;
    crate::catalog::initialize(&current_dir)?;

    println!("✅ Initialized recipe catalog in {}", current_dir.display());
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
