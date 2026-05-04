use crate::errors::app_error::AppError;
use flate2::read::GzDecoder;
use tar::Archive;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use colored::Colorize;

pub async fn update_geoip_db(
    url: &str,
    license_key: &str,
    target_path: &str,
) -> Result<(), AppError> {
    println!("{} Checking for GeoIP database updates...", "[GEOIP]".cyan());
    
    let download_url = format!("{}&license_key={}", url, license_key);
    
    let response = reqwest::get(&download_url).await
        .map_err(|e| AppError::InternalError(format!("Failed to download GeoIP DB: {e}")))?;
    
    if !response.status().is_success() {
        return Err(AppError::InternalError(format!("Failed to download GeoIP DB: HTTP {}", response.status())));
    }

    let bytes = response.bytes().await
        .map_err(|e| AppError::InternalError(format!("Failed to read GeoIP DB bytes: {e}")))?;

    // MaxMind returns a tar.gz. We need to extract it and find the .mmdb file.
    let tar_gz = GzDecoder::new(&bytes[..]);
    let mut archive = Archive::new(tar_gz);
    
    let mut found = false;
    for entry in archive.entries().map_err(|e| AppError::InternalError(format!("Failed to read tar entries: {e}")))? {
        let mut entry = entry.map_err(|e| AppError::InternalError(format!("Failed to read tar entry: {e}")))?;
        let path = entry.path().map_err(|e| AppError::InternalError(format!("Failed to get entry path: {e}")))?;
        
        if path.extension().is_some_and(|ext| ext == "mmdb") {
            let parent = Path::new(target_path).parent().unwrap_or_else(|| Path::new("."));
            std::fs::create_dir_all(parent).ok();
            
            let mut outfile = File::create(target_path)
                .map_err(|e| AppError::IoError(e))?;
            
            copy(&mut entry, &mut outfile)
                .map_err(|e| AppError::IoError(e))?;
            
            println!("{} GeoIP database updated successfully: {}", "[GEOIP]".green().bold(), target_path);
            found = true;
            break;
        }
    }

    if !found {
        return Err(AppError::InternalError("No .mmdb file found in the MaxMind archive".to_string()));
    }

    Ok(())
}
