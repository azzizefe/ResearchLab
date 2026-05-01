use crate::models::connection::{Connection, ConnectionDirection, ConnectionStatus};
use crate::errors::app_error::AppError;
use regex::Regex;
use chrono::{DateTime, Utc, NaiveDateTime, TimeZone};
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_trace_file<P: AsRef<Path>>(path: P) -> Result<Vec<Connection>, AppError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut connections = Vec::new();

    let re_incoming = Regex::new(r"(?P<ts>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3}).*Incoming connection from (?P<id>\d{3} \d{3} \d{3})")
        .expect("Static Regex for incoming connections is invalid");
    let re_outgoing = Regex::new(r"(?P<ts>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3}).*Connecting to (?P<id>\d{3} \d{3} \d{3})")
        .expect("Static Regex for outgoing connections is invalid");

    for line in reader.lines() {
        let line = line?;
        
        if let Some(caps) = re_incoming.captures(&line) {
            let ts_str = &caps["ts"];
            let anydesk_id = &caps["id"];
            let timestamp = parse_anydesk_timestamp(ts_str)?;

            connections.push(Connection {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp_start: timestamp,
                timestamp_end: None,
                remote_id: anydesk_id.to_string(),
                remote_ip: None,
                direction: ConnectionDirection::Inbound,
                status: ConnectionStatus::Connecting,
            });
        } else if let Some(caps) = re_outgoing.captures(&line) {
            let ts_str = &caps["ts"];
            let anydesk_id = &caps["id"];
            let timestamp = parse_anydesk_timestamp(ts_str)?;

            connections.push(Connection {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp_start: timestamp,
                timestamp_end: None,
                remote_id: anydesk_id.to_string(),
                remote_ip: None,
                direction: ConnectionDirection::Outbound,
                status: ConnectionStatus::Connecting,
            });
        }
    }

    Ok(connections)
}

fn parse_anydesk_timestamp(ts_str: &str) -> Result<DateTime<Utc>, AppError> {
    let naive = NaiveDateTime::parse_from_str(ts_str, "%Y-%m-%d %H:%M:%S%.3f")
        .map_err(|e| AppError::ParseError(format!("Failed to parse timestamp: {}", e)))?;
    
    // Assuming Utc for simplicity, AnyDesk logs are usually local time, but we convert to Utc
    Ok(Utc.from_utc_datetime(&naive))
}
