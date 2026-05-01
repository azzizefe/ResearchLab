use crate::errors::app_error::AppError;
use std::path::{Path, PathBuf};

/// 13.2.3: Input validation: her public fonksiyon girisini dogrula
/// 13.2.4: Dosya yolu sanitization (path traversal onleme)
pub fn sanitize_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, AppError> {
    let path = path.as_ref();

    // Path traversal check
    if path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(AppError::ConfigError(
            "Path traversal detected (..) ".to_string(),
        ));
    }

    // Canonicalize if exists, otherwise just return as is but validated
    if path.exists() {
        Ok(dunce::canonicalize(path)
            .map_err(|e| AppError::ConfigError(format!("Failed to canonicalize path: {e}")))?)
    } else {
        Ok(path.to_path_buf())
    }
}

/// 13.2.5: Log dosyasi boyut kontrolu
pub fn check_file_size(path: &Path, max_mb: u64) -> Result<(), AppError> {
    let metadata = std::fs::metadata(path)?;
    if metadata.len() > max_mb * 1024 * 1024 {
        return Err(AppError::ParseError(format!(
            "File too large ({} MB), maximum allowed is {} MB. Use streaming mode.",
            metadata.len() / 1024 / 1024,
            max_mb
        )));
    }
    Ok(())
}
