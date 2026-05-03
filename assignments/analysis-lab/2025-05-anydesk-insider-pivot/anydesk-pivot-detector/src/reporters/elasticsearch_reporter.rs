use crate::errors::app_error::AppError;
use crate::models::alert::Alert;
use serde_json::json;
use reqwest::Client;

pub struct ElasticsearchReporter {
    client: Client,
    url: String,
    index: String,
}

impl ElasticsearchReporter {
    #[must_use]
    pub fn new(url: String, index: String) -> Self {
        Self {
            client: Client::new(),
            url,
            index,
        }
    }

    pub async fn report(&self, alerts: &[Alert]) -> Result<(), AppError> {
        for alert in alerts {
            let body = json!({
                "@timestamp": alert.timestamp,
                "severity": format!("{:?}", alert.severity),
                "title": alert.title,
                "description": alert.description,
                "source_module": alert.source_module,
                "evidence": alert.evidence,
                "report_id": alert.id,
            });

            let endpoint = format!("{}/{}/_doc", self.url, self.index);
            
            self.client
                .post(&endpoint)
                .json(&body)
                .send()
                .await
                .map_err(|e| AppError::MonitorError(format!("Failed to send to Elasticsearch: {e}")))?;
        }
        Ok(())
    }
}
