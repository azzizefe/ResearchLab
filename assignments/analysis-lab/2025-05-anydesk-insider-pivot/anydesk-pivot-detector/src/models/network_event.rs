use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkEvent {
    pub timestamp: DateTime<Utc>,
    pub local_address: String,
    pub remote_address: String,
    pub remote_port: u16,
    pub protocol: String,
    pub process_id: u32,
    pub process_name: Option<String>,
    pub geolocation: Option<GeoLocation>,
    pub domain_name: Option<String>,
    pub data_sent_bytes: Option<u64>,
    pub data_received_bytes: Option<u64>,
    pub event_type: NetworkEventType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeoLocation {
    pub country: String,
    pub city: String,
    pub isp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NetworkEventType {
    AnyDeskConnection,
    SuspiciousDNS,
    ForbiddenPortTraffic,
    HighDataVolume,
}
