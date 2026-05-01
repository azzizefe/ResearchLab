use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProcessEventType {
    Started,
    Stopped,
    SuspiciousActivity,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessEvent {
    pub timestamp: DateTime<Utc>,
    pub pid: u32,
    pub name: String,
    pub path: String,
    pub command_line: Option<String>,
    pub event_type: ProcessEventType,
    pub parent_pid: Option<u32>,
}
