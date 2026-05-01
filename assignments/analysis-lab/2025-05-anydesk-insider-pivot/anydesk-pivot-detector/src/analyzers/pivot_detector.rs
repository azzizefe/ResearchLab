use crate::models::alert::{Alert, AlertSeverity};
// Removed unused connection imports
use regex::Regex;
use chrono::Utc;

pub struct PivotDetector {
    re_incoming: Regex,
    re_file_transfer: Regex,
    re_startup: Regex,
}

impl PivotDetector {
    pub fn new() -> Self {
        Self {
            re_incoming: Regex::new(r"Incoming connection from (?P<id>\d{3} \d{3} \d{3})")
                .expect("PivotDetector: Incoming regex invalid"),
            re_file_transfer: Regex::new(r"File transfer")
                .expect("PivotDetector: File transfer regex invalid"),
            re_startup: Regex::new(r"AnyDesk is starting|Starting service")
                .expect("PivotDetector: Startup regex invalid"),
        }
    }

    pub fn analyze_line(&self, line: &str) -> Vec<Alert> {
        let mut alerts = Vec::new();

        if let Some(caps) = self.re_incoming.captures(line) {
            alerts.push(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::Medium,
                title: "New Incoming Connection".to_string(),
                description: format!("New connection from AnyDesk ID: {}", &caps["id"]),
                source_module: "LogAnalyzer".to_string(),
                evidence: serde_json::json!({ "anydesk_id": &caps["id"], "line": line }),
            });
        }

        if self.re_file_transfer.is_match(line) {
            alerts.push(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::High,
                title: "File Transfer Activity".to_string(),
                description: "AnyDesk file transfer activity detected.".to_string(),
                source_module: "LogAnalyzer".to_string(),
                evidence: serde_json::json!({ "line": line }),
            });
        }

        if self.re_startup.is_match(line) {
            alerts.push(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::Low,
                title: "AnyDesk Started".to_string(),
                description: "AnyDesk service or application has started.".to_string(),
                source_module: "LogAnalyzer".to_string(),
                evidence: serde_json::json!({ "line": line }),
            });
        }

        alerts
    }
}
