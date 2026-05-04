use crate::errors::app_error::AppError;
use crate::models::network_event::{GeoLocation, NetworkEvent, NetworkEventType};
use colored::Colorize;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use sysinfo::{ProcessRefreshKind, RefreshKind, System, UpdateKind};
use tokio::time::sleep;
use tokio::sync::mpsc;

pub struct NetworkMonitor {
    sys: System,
    geoloc_cache: HashMap<String, GeoLocation>,
    forbidden_ports: Vec<u16>,
    _high_traffic_threshold: u64, // bytes
    tx: mpsc::Sender<NetworkEvent>,
}

impl NetworkMonitor {
    #[must_use] 
    pub fn new(tx: mpsc::Sender<NetworkEvent>) -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_processes(ProcessRefreshKind::nothing().with_user(UpdateKind::Always)),
        );
        sys.refresh_all();

        Self {
            sys,
            geoloc_cache: HashMap::new(),
            forbidden_ports: vec![22, 23, 445, 3389], // SSH, Telnet, SMB, RDP
            _high_traffic_threshold: 10 * 1024 * 1024, // 10MB threshold for demo
            tx,
        }
    }

    pub async fn run(&mut self) -> Result<(), AppError> {
        println!("{}", "Starting Network Monitor...".green().bold());
        loop {
            self.sys.refresh_all();

            // 1. Check active connections (AnyDesk ports + Forbidden ports)
            if let Err(e) = self.check_connections().await {
                eprintln!("[ERROR] Connection check failed: {e}");
            }

            // 2. Check DNS cache for AnyDesk domains
            if let Err(e) = self.check_dns_cache().await {
                eprintln!("[ERROR] DNS cache check failed: {e}");
            }

            sleep(Duration::from_secs(10)).await;
        }
    }

    async fn check_connections(&mut self) -> Result<(), AppError> {
        #[cfg(windows)]
        {
            let output = tokio::process::Command::new("powershell")
                .args(["-Command", "Get-NetTCPConnection | Select-Object LocalAddress, LocalPort, RemoteAddress, RemotePort, State, OwningProcess | ConvertTo-Json"])
                .output()
                .await
                .map_err(|e| AppError::MonitorError(format!("Failed to run powershell: {e}")))?;

            if !output.status.success() {
                return Ok(());
            }

            let json_str = String::from_utf8_lossy(&output.stdout);
            if json_str.trim().is_empty() {
                return Ok(());
            }

            let connections: Value = serde_json::from_str(&json_str).unwrap_or(Value::Null);

            let process_conns = |conn: &Value| {
                let remote_port = conn["RemotePort"].as_u64().unwrap_or(0) as u16;
                let remote_addr = conn["RemoteAddress"].as_str().unwrap_or("").to_string();
                let process_id = conn["OwningProcess"].as_u64().unwrap_or(0) as u32;
                let _local_addr = conn["LocalAddress"].as_str().unwrap_or("").to_string();

                if remote_addr == "0.0.0.0" || remote_addr == "127.0.0.1" || remote_addr == "::" {
                    return;
                }

                if remote_port == 6568 || remote_port == 443 {
                    let is_anydesk = self
                        .sys
                        .process(sysinfo::Pid::from(process_id as usize))
                        .is_some_and(|p| {
                            p.name()
                                .to_string_lossy()
                                .to_lowercase()
                                .contains("anydesk")
                        });

                    if is_anydesk {
                        println!(
                            "{} AnyDesk connection: {}:{} (PID: {})",
                            "[NETWORK]".cyan(),
                            remote_addr,
                            remote_port,
                            process_id
                        );
                        let _ = self.tx.blocking_send(NetworkEvent {
                            timestamp: chrono::Utc::now(),
                            local_address: _local_addr.clone(),
                            remote_address: remote_addr.clone(),
                            remote_port,
                            protocol: "TCP".to_string(),
                            process_id,
                            process_name: Some("AnyDesk".to_string()),
                            geolocation: None,
                            domain_name: None,
                            data_sent_bytes: None,
                            data_received_bytes: None,
                            event_type: NetworkEventType::AnyDeskConnection,
                        });
                    }
                }

                if self.forbidden_ports.contains(&remote_port) {
                    println!(
                        "{} Traffic on forbidden port {} to {} (PID: {})",
                        "[ALERT]".red().bold(),
                        remote_port,
                        remote_addr,
                        process_id
                    );
                    let _ = self.tx.blocking_send(NetworkEvent {
                        timestamp: chrono::Utc::now(),
                        local_address: _local_addr.clone(),
                        remote_address: remote_addr.clone(),
                        remote_port,
                        protocol: "TCP".to_string(),
                        process_id,
                        process_name: None,
                        geolocation: None,
                        domain_name: None,
                        data_sent_bytes: None,
                        data_received_bytes: None,
                        event_type: NetworkEventType::ForbiddenPortTraffic,
                    });
                }
            };

            match connections {
                Value::Array(list) => {
                    for conn in list {
                        process_conns(&conn);
                    }
                }
                Value::Object(conn) => {
                    process_conns(&Value::Object(conn));
                }
                _ => {}
            }
        }
        #[cfg(not(windows))]
        {
            // TODO: Implement Linux network monitoring (e.g. using /proc/net/tcp or similar)
            // tracing::debug!("Network monitoring not implemented for this platform");
        }

        Ok(())
    }

    async fn check_dns_cache(&mut self) -> Result<(), AppError> {
        #[cfg(windows)]
        {
            let output = tokio::process::Command::new("powershell")
                .args(["-Command", "Get-DnsClientCache | Where-Object Name -like '*.net.anydesk.com' | Select-Object Name, Data, Type | ConvertTo-Json"])
                .output()
                .await
                .map_err(|e| AppError::MonitorError(format!("Failed to run powershell: {e}")))?;

            let json_str = String::from_utf8_lossy(&output.stdout);
            if json_str.trim().is_empty() || json_str.trim() == "null" {
                return Ok(());
            }

            let dns_entries: Value = serde_json::from_str(&json_str).unwrap_or(Value::Null);

            let process_dns = |entry: &Value| {
                let name = entry["Name"].as_str().unwrap_or("unknown");
                println!("{} AnyDesk DNS query detected: {}", "[DNS]".yellow(), name);
                // We use blocking_send here because we are inside a closure
                let _ = self.tx.blocking_send(NetworkEvent {
                    timestamp: chrono::Utc::now(),
                    local_address: String::new(),
                    remote_address: String::new(),
                    remote_port: 0,
                    protocol: "DNS".to_string(),
                    process_id: 0,
                    process_name: None,
                    geolocation: None,
                    domain_name: Some(name.to_string()),
                    data_sent_bytes: None,
                    data_received_bytes: None,
                    event_type: NetworkEventType::SuspiciousDNS,
                });
            };

            match dns_entries {
                Value::Array(entries) => {
                    for entry in entries {
                        process_dns(&entry);
                    }
                }
                Value::Object(entry) => {
                    process_dns(&Value::Object(entry));
                }
                _ => {}
            }
        }
        #[cfg(not(windows))]
        {
            // TODO: Implement DNS cache check for Linux (systemd-resolve or similar)
        }

        Ok(())
    }

    pub async fn geolocate_ip(&mut self, ip: &str) -> Option<GeoLocation> {
        if self.geoloc_cache.contains_key(ip) {
            return self.geoloc_cache.get(ip).cloned();
        }

        // Simulating Geolocation since we added reqwest
        let url = format!(
            "http://ip-api.com/json/{ip}?fields=status,country,city,isp"
        );
        if let Ok(resp) = reqwest::get(&url).await {
            if let Ok(json) = resp.json::<Value>().await {
                if json["status"] == "success" {
                    let loc = GeoLocation {
                        country: json["country"].as_str().unwrap_or("Unknown").to_string(),
                        city: json["city"].as_str().unwrap_or("Unknown").to_string(),
                        isp: json["isp"].as_str().unwrap_or("Unknown").to_string(),
                    };
                    self.geoloc_cache.insert(ip.to_string(), loc.clone());
                    return Some(loc);
                }
            }
        }
        None
    }
}
