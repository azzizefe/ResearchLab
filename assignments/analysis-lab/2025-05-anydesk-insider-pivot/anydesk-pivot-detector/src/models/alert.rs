use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alert {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub severity: AlertSeverity,
    pub title: String,
    pub description: String,
    pub source_module: String,
    pub evidence: serde_json::Value,
}
