use crate::errors::app_error::AppError;
use crate::models::alert::Alert;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub struct JsonReporter {
    output_dir: PathBuf,
}

impl JsonReporter {
    #[must_use] 
    pub fn new(output_dir: String) -> Self {
        let path = PathBuf::from(output_dir);
        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }
        Self { output_dir: path }
    }

    /// 9.1.1 & 9.1.4: Uyari olaylarini JSON formatinda dosyaya yaz ve kaydet
    /// 9.1.2: Her rapor icin UUID olustur
    /// 9.1.3: Zaman damgasi, kaynak, skor, detay alanlari
    /// Generates a JSON report for the given alerts.
    ///
    /// # Errors
    ///
    /// Returns `AppError::ParseError` if serialization fails.
    /// Returns `AppError::IoError` if writing to the file fails.
    pub fn report(&self, alerts: &[Alert]) -> Result<String, AppError> {
        let report_id = Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now();

        let report = json!({
            "report_id": report_id,
            "timestamp": timestamp,
            "metadata": {
                "system": "AnyDesk Pivot Detector",
                "version": "0.1.0",
            },
            "summary": {
                "alert_count": alerts.len(),
            },
            "alerts": alerts,
        });

        let filename = format!("report_{report_id}.json");
        let file_path = self.output_dir.join(filename);

        let json_data = serde_json::to_string_pretty(&report)
            .map_err(|e| AppError::ParseError(format!("Failed to serialize report: {e}")))?;

        fs::write(&file_path, &json_data).map_err(AppError::IoError)?;

        Ok(file_path.to_string_lossy().into_owned())
    }
}
