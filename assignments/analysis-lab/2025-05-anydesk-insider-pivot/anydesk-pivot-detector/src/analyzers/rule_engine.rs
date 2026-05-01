use crate::models::alert::{Alert, AlertSeverity};
use crate::models::process_event::ProcessEvent;
use crate::models::network_event::NetworkEvent;
use crate::config::AppConfig;
use chrono::{Utc, Timelike};
use serde_json::json;

pub struct RuleEngine {
    config: AppConfig,
}

impl RuleEngine {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    /// Rule: Connection detection outside working hours
    pub fn check_working_hours(&self, timestamp: chrono::DateTime<Utc>) -> Option<Alert> {
        let hour = timestamp.hour();
        if hour < self.config.monitor.working_hour_start || hour >= self.config.monitor.working_hour_end {
            return Some(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::Medium,
                title: "Outside Working Hours Connection".to_string(),
                description: format!("AnyDesk activity detected outside working hours ({}:00 - {}:00)", 
                    self.config.monitor.working_hour_start, self.config.monitor.working_hour_end),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "event_timestamp": timestamp, "hour": hour }),
            });
        }
        None
    }

    /// Rule: Connection from IDs outside known ACL
    pub fn check_acl(&self, anydesk_id: &str) -> Option<Alert> {
        if self.config.network.allowed_anydesk_ids.is_empty() {
            return None; // No ACL defined, assume all allowed or not checked
        }

        if !self.config.network.allowed_anydesk_ids.contains(&anydesk_id.to_string()) {
            return Some(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                severity: AlertSeverity::High,
                title: "Unauthorized AnyDesk ID".to_string(),
                description: format!("Connection from unauthorized AnyDesk ID: {}", anydesk_id),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "anydesk_id": anydesk_id }),
            });
        }
        None
    }

    /// Rule: Shell/CLI started via AnyDesk
    pub fn check_suspicious_process(&self, event: &ProcessEvent) -> Option<Alert> {
        let name_lower = event.name.to_lowercase();
        let is_suspicious = self.config.monitor.suspicious_processes.iter()
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
                description: format!("Suspicious process '{}' (PID: {}) detected.", event.name, event.pid),
                source_module: "RuleEngine".to_string(),
                evidence: json!({ "process": event }),
            });
        }
        None
    }

    /// Rule: Network scanning tool execution (already covered by suspicious processes, 
    /// but we can add more specific logic if needed)
    pub fn check_network_scanning(&self, event: &NetworkEvent) -> Option<Alert> {
        // Example: High number of different remote addresses in a short time
        // For now, check for forbidden ports traffic
        let forbidden_ports = vec![22, 23, 445, 3389];
        if forbidden_ports.contains(&event.remote_port) {
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
        None
    }
}
