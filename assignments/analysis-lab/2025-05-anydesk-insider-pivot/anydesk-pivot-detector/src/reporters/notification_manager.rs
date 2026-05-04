use crate::config::NotificationSettings;
use crate::models::alert::Alert;
use crate::errors::app_error::AppError;
use reqwest::Client;
use serde_json::json;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct NotificationManager {
    settings: NotificationSettings,
    client: Client,
}

impl NotificationManager {
    pub fn new(settings: NotificationSettings) -> Self {
        Self {
            settings,
            client: Client::new(),
        }
    }

    pub async fn notify(&self, alert: &Alert) -> Result<(), AppError> {
        if self.settings.enable_slack {
            self.send_slack(alert).await?;
        }

        if self.settings.enable_email {
            self.send_email(alert).await?;
        }

        if self.settings.enable_webhook {
            self.send_webhook(alert).await?;
        }

        Ok(())
    }

    async fn send_slack(&self, alert: &Alert) -> Result<(), AppError> {
        let payload = json!({
            "text": format!("🚨 *CRITICAL ALERT DETECTED*\n*Title:* {}\n*Severity:* {:?}\n*Module:* {}\n*Description:* {}", 
                alert.title, alert.severity, alert.source_module, alert.description)
        });

        self.client.post(&self.settings.slack_webhook_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(format!("Slack notification failed: {}", e)))?;

        Ok(())
    }

    async fn send_email(&self, alert: &Alert) -> Result<(), AppError> {
        let email = Message::builder()
            .from("AnyDesk Pivot Detector <noreply@researchlab.com>".parse().unwrap())
            .to(self.settings.email_to.parse().unwrap())
            .subject(format!("AnyDesk Alert: {}", alert.title))
            .body(format!(
                "Security Alert Details:\n\nTitle: {}\nSeverity: {:?}\nSource: {}\nDescription: {}\nTimestamp: {}",
                alert.title, alert.severity, alert.source_module, alert.description, alert.timestamp
            ))
            .map_err(|e| AppError::InternalError(format!("Email construction failed: {}", e)))?;

        // Note: In a real app, you'd want to load credentials from env or config
        // For now, we assume no auth or simple setup for demonstration
        let mailer = SmtpTransport::relay(&self.settings.smtp_server)
            .map_err(|e| AppError::InternalError(format!("SMTP relay setup failed: {}", e)))?
            .port(self.settings.smtp_port)
            .build();

        mailer.send(&email)
            .map_err(|e| AppError::InternalError(format!("Email sending failed: {}", e)))?;

        Ok(())
    }

    async fn send_webhook(&self, alert: &Alert) -> Result<(), AppError> {
        self.client.post(&self.settings.webhook_url)
            .json(alert)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(format!("Webhook notification failed: {}", e)))?;

        Ok(())
    }
}
