use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConnectionDirection {
    Inbound,
    Outbound,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    pub id: String,
    pub timestamp_start: DateTime<Utc>,
    pub timestamp_end: Option<DateTime<Utc>>,
    pub remote_id: String,
    pub remote_ip: Option<String>,
    pub direction: ConnectionDirection,
    pub status: ConnectionStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConnectionStatus {
    Connecting,
    Established,
    Closed,
    Failed,
    Unknown,
}

impl Connection {
    #[must_use] 
    pub fn duration(&self) -> Option<chrono::Duration> {
        match (self.timestamp_start, self.timestamp_end) {
            (start, Some(end)) => Some(end - start),
            _ => None,
        }
    }
}
