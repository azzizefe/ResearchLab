use crate::errors::app_error::AppError;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct SystemConfig {
    pub anydesk_id: Option<String>,
    pub unattended_access: bool,
    pub interactive_access: String,
    pub license_type: String,
    pub raw_settings: HashMap<String, String>,
}

/// Parses AnyDesk's system configuration file.
///
/// # Errors
///
/// Returns `AppError::IoError` if the file cannot be opened.
/// Returns `std::io::Error` via `AppError::IoError` if a line cannot be read.
pub fn parse_system_conf<P: AsRef<Path>>(path: P) -> Result<SystemConfig, AppError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut settings = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            settings.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    let anydesk_id = settings.get("ad.anynet.id").cloned();
    let unattended_access = settings
        .get("ad.security.unattended_access")
        .is_some_and(|v| v == "true" || v == "1");
    let interactive_access = settings
        .get("ad.security.interactive_access")
        .cloned()
        .unwrap_or_default();
    let license_type = settings
        .get("ad.license")
        .cloned()
        .unwrap_or_else(|| "free".to_string());

    Ok(SystemConfig {
        anydesk_id,
        unattended_access,
        interactive_access,
        license_type,
        raw_settings: settings,
    })
}
