use crate::errors::app_error::AppError;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct ServiceConfig {
    pub allowed_ids: Vec<String>,
    pub enable_2fa: bool,
    pub file_transfer_enabled: bool,
    pub clipboard_enabled: bool,
    pub raw_settings: HashMap<String, String>,
}

pub fn parse_service_conf<P: AsRef<Path>>(path: P) -> Result<ServiceConfig, AppError> {
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

    let allowed_ids_str = settings
        .get("ad.security.allow_ids")
        .cloned()
        .unwrap_or_default();
    let allowed_ids = allowed_ids_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let enable_2fa = settings
        .get("ad.security.2fa")
        .is_some_and(|v| v == "true" || v == "1");

    let file_transfer_enabled = settings
        .get("ad.features.file_transfer")
        .is_none_or(|v| v != "false" && v != "0");

    let clipboard_enabled = settings
        .get("ad.features.clipboard")
        .is_none_or(|v| v != "false" && v != "0");

    Ok(ServiceConfig {
        allowed_ids,
        enable_2fa,
        file_transfer_enabled,
        clipboard_enabled,
        raw_settings: settings,
    })
}
