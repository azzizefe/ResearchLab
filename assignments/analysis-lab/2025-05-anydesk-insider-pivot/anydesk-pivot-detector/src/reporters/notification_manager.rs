use crate::config::NotificationSettings;
use crate::models::alert::Alert;
use crate::errors::app_error::AppError;
use reqwest::Client;
use serde_json::json;
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
            self.send_email(alert)?;
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
            .map_err(|e| AppError::NetworkError(format!("Slack notification failed: {e}")))?;

        Ok(())
    }

    fn send_email(&self, alert: &Alert) -> Result<(), AppError> {
        let from_addr = "AnyDesk Pivot Detector <noreply@researchlab.com>";
        let to_addr = &self.settings.email_to;

        // Basic validation
        if !to_addr.contains('@') {
            return Err(AppError::InternalError(format!("Invalid recipient email address: {to_addr}")));
        }

        let email = Message::builder()
            .from(from_addr.parse().map_err(|e| AppError::InternalError(format!("Invalid from address: {e}")))? )
            .to(to_addr.parse().map_err(|e| AppError::InternalError(format!("Invalid recipient address: {e}")))? )
            .subject(format!("AnyDesk Alert: {}", alert.title))
            .body(format!(
                "Security Alert Details:\n\nTitle: {}\nSeverity: {:?}\nSource: {}\nDescription: {}\nTimestamp: {}",
                alert.title, alert.severity, alert.source_module, alert.description, alert.timestamp
            ))
            .map_err(|e| AppError::InternalError(format!("Email construction failed: {e}")))?;

        let mut mailer_builder = if self.settings.use_tls {
            SmtpTransport::starttls_relay(&self.settings.smtp_server)
        } else {
            SmtpTransport::relay(&self.settings.smtp_server)
        }.map_err(|e| AppError::InternalError(format!("SMTP relay setup failed: {e}")))?;

        mailer_builder = mailer_builder.port(self.settings.smtp_port);

        if let (Some(user), Some(pass)) = (&self.settings.smtp_username, &self.settings.smtp_password) {
            let credentials = lettre::transport::smtp::authentication::Credentials::new(user.to_string(), pass.to_string());
            mailer_builder = mailer_builder.credentials(credentials);
        }

        let mailer = mailer_builder.build();

        mailer.send(&email)
            .map_err(|e| AppError::InternalError(format!("Email sending failed: {e}")))?;

        Ok(())
    }

    async fn send_webhook(&self, alert: &Alert) -> Result<(), AppError> {
        self.client.post(&self.settings.webhook_url)
            .json(alert)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(format!("Webhook notification failed: {e}")))?;

        Ok(())
    }
}
