use crate::config::AppConfig;
use crate::models::alert::{Alert, AlertSeverity};
use crate::models::network_event::NetworkEvent;
use crate::models::process_event::ProcessEvent;
use chrono::{Timelike, Utc};
use serde_json::json;

pub struct RuleEngine {
    config: AppConfig,
}

impl RuleEngine {
    #[must_use] 
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// Rule: Connection detection outside working hours
    #[must_use] 
    pub fn check_working_hours(&self, timestamp: chrono::DateTime<Utc>) -> Option<Alert> {
        let hour = timestamp.hour();
        if hour < self.config.monitor.working_hour_start
            || hour >= self.config.monitor.working_hour_end
        {
            return Some(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::Medium,
                title: "Outside Working Hours Connection".to_string(),
                description: format!(
                    "AnyDesk activity detected outside working hours ({}:00 - {}:00)",
                    self.config.monitor.working_hour_start, self.config.monitor.working_hour_end
                ),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "event_timestamp": timestamp, "hour": hour }),
            });
        }
        None
    }

    /// Rule: Connection from IDs outside known ACL
    #[must_use] 
    pub fn check_acl(&self, anydesk_id: &str) -> Option<Alert> {
        if self.config.network.allowed_anydesk_ids.is_empty() {
            return None; // No ACL defined, assume all allowed or not checked
        }

        if !self
            .config
            .network
            .allowed_anydesk_ids
            .contains(&anydesk_id.to_string())
        {
            return Some(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::High,
                title: "Unauthorized AnyDesk ID".to_string(),
                description: format!("Connection from unauthorized AnyDesk ID: {anydesk_id}"),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "anydesk_id": anydesk_id }),
            });
        }
        None
    }

    /// Rule: Shell/CLI started via `AnyDesk`
    #[must_use] 
    pub fn check_suspicious_process(&self, event: &ProcessEvent) -> Option<Alert> {
        let name_lower = event.name.to_lowercase();
        let is_suspicious = self
            .config
            .monitor
            .suspicious_processes
            .iter()
            .any(|s| name_lower.contains(&s.to_lowercase()));

        if is_suspicious {
            let severity = if event.parent_pid.is_some() {
                // If we could determine parent is AnyDesk (checked in monitor)
                AlertSeverity::Critical
            } else {
                AlertSeverity::Medium
            };

            return Some(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity,
                title: "Suspicious Process Execution".to_string(),
                description: format!(
                    "Suspicious process '{}' (PID: {}) detected.",
                    event.name, event.pid
                ),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "process": event }),
            });
        }
        None
    }

    /// Rule: Network scanning tool execution (already covered by suspicious processes,
    /// but we can add more specific logic if needed)
    #[must_use] 
    pub fn check_network_scanning(&self, event: &NetworkEvent) -> Option<Alert> {
        // 9.3.1: Forbidden ports traffic
        if self.config.network.blocked_ports.contains(&event.remote_port) {
            return Some(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::High,
                title: "Forbidden Port Traffic".to_string(),
                description: format!("Traffic detected on forbidden port: {}", event.remote_port),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "network_event": event }),
            });
        }

        // 9.3.2: Malicious IP detection (IOC)
        if self.config.network.malicious_ips.contains(&event.remote_address) {
            return Some(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::Critical,
                title: "Malicious IP Connection".to_string(),
                description: format!("Connection to known malicious IP detected: {}", event.remote_address),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "network_event": event }),
            });
        }

        // 9.3.3: Data Exfiltration Detection (High data volume)
        let total_bytes = event.data_sent_bytes.unwrap_or(0) + event.data_received_bytes.unwrap_or(0);
        if total_bytes > self.config.network.high_data_threshold_bytes {
            return Some(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::High,
                title: "High Data Volume Detected".to_string(),
                description: format!("Suspicious data volume detected: {} bytes", total_bytes),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "network_event": event, "total_bytes": total_bytes }),
            });
        }

        None
    }
    /// Rule: Dosya transferi + hassas dosya yolu eslesme
    #[must_use] 
    pub fn check_sensitive_file_access(&self, path: &str) -> Option<Alert> {
        let sensitive_paths = ["C:\\Windows\\System32\\config",
            "C:\\Users\\Administrator",
            "wallet.dat",
            "config.php",
            ".env"];
        if sensitive_paths.iter().any(|s| path.contains(s)) {
            return Some(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::Critical,
                title: "Sensitive File Access".to_string(),
                description: format!("AnyDesk accessed sensitive path: {path}"),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "path": path }),
            });
        }
        None
    }

    /// Rule: Unattended access + statik sifre kullanimi
    #[must_use] 
    pub fn check_unattended_access(&self, log_line: &str) -> Option<Alert> {
        if log_line.contains("Password accepted") || log_line.contains("Unattended access") {
            return Some(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::Medium,
                title: "Unattended Access Detected".to_string(),
                description: "Unattended access or static password used for connection."
                    .to_string(),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "line": log_line }),
            });
        }
        None
    }

    /// Rule: Portable `AnyDesk` calistirilmasi
    #[must_use] 
    pub fn check_portable_anydesk(&self, process_path: &str) -> Option<Alert> {
        let path_lower = process_path.to_lowercase();
        let is_portable = !path_lower.contains("program files");

        if is_portable && path_lower.contains("anydesk") {
            return Some(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::Low,
                title: "Portable AnyDesk Execution".to_string(),
                description: format!(
                    "AnyDesk running from non-standard location: {process_path}"
                ),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "path": process_path }),
            });
        }
        None
    }
}
