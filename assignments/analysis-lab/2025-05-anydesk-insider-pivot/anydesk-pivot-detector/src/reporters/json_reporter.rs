use crate::models::alert::Alert;
use crate::errors::app_error::AppError;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use serde_json::json;

pub struct JsonReporter {
    output_dir: PathBuf,
}

impl JsonReporter {
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

        let filename = format!("report_{}.json", report_id);
        let file_path = self.output_dir.join(filename);
        
        let json_data = serde_json::to_string_pretty(&report)
            .map_err(|e| AppError::ParseError(format!("Failed to serialize report: {}", e)))?;

        fs::write(&file_path, &json_data)
            .map_err(|e| AppError::IoError(e))?;

        Ok(file_path.to_string_lossy().into_owned())
    }
}
