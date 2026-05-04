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
    geoip_db_path: Option<String>,
}

impl NetworkMonitor {
    #[must_use] 
    pub fn new(tx: mpsc::Sender<NetworkEvent>, geoip_db_path: Option<String>) -> Self {
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
            geoip_db_path,
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
                let local_addr = conn["LocalAddress"].as_str().unwrap_or("").to_string();

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
                            local_address: local_addr.clone(),
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
                    let _ = self.tx.blocking_send(NetworkEvent {
                        timestamp: chrono::Utc::now(),
                        local_address: local_addr.clone(),
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

        #[cfg(target_os = "linux")]
        {
            let output = tokio::process::Command::new("ss")
                .args(["-ntup"])
                .output()
                .await;

            if let Ok(output) = output {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    for line in stdout.lines().skip(1) {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 6 {
                            let local = parts[4];
                            let remote = parts[5];
                            let user_info = if parts.len() > 6 { parts[6] } else { "" };

                            let remote_parts: Vec<&str> = remote.rsplitn(2, ':').collect();
                            if remote_parts.len() == 2 {
                                let remote_addr = remote_parts[1].to_string();
                                let remote_port = remote_parts[0].parse::<u16>().unwrap_or(0);
                                
                                let local_parts: Vec<&str> = local.rsplitn(2, ':').collect();
                                let local_addr = if local_parts.len() == 2 { local_parts[1] } else { "" }.to_string();

                                let process_id = if user_info.contains("pid=") {
                                    user_info.split("pid=").nth(1)
                                        .and_then(|s| s.split(',').next())
                                        .and_then(|s| s.parse::<u32>().ok())
                                        .unwrap_or(0)
                                } else {
                                    0
                                };

                                if remote_addr == "0.0.0.0" || remote_addr == "127.0.0.1" || remote_addr == "::" {
                                    continue;
                                }

                                if remote_port == 6568 || remote_port == 443 {
                                    let is_anydesk = self.sys.process(sysinfo::Pid::from(process_id as usize))
                                        .is_some_and(|p| p.name().to_string_lossy().to_lowercase().contains("anydesk"));

                                    if is_anydesk {
                                        let _ = self.tx.blocking_send(NetworkEvent {
                                            timestamp: chrono::Utc::now(),
                                            local_address: local_addr.clone(),
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
                                    let _ = self.tx.blocking_send(NetworkEvent {
                                        timestamp: chrono::Utc::now(),
                                        local_address: local_addr,
                                        remote_address: remote_addr,
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
                            }
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let output = tokio::process::Command::new("lsof")
                .args(["-nP", "-iTCP", "-sTCP:ESTABLISHED"])
                .output()
                .await;

            if let Ok(output) = output {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    for line in stdout.lines().skip(1) {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 9 {
                            let process_name = parts[0];
                            let process_id = parts[1].parse::<u32>().unwrap_or(0);
                            let connection = parts[8];

                            if let Some((local, remote)) = connection.split_once("->") {
                                let remote_parts: Vec<&str> = remote.rsplitn(2, ':').collect();
                                if remote_parts.len() == 2 {
                                    let remote_addr = remote_parts[1].to_string();
                                    let remote_port = remote_parts[0].parse::<u16>().unwrap_or(0);
                                    
                                    let local_parts: Vec<&str> = local.rsplitn(2, ':').collect();
                                    let local_addr = if local_parts.len() == 2 { local_parts[1] } else { "" }.to_string();

                                    if remote_addr == "127.0.0.1" || remote_addr == "::1" {
                                        continue;
                                    }

                                    if remote_port == 6568 || remote_port == 443 {
                                        if process_name.to_lowercase().contains("anydesk") {
                                            let _ = self.tx.blocking_send(NetworkEvent {
                                                timestamp: chrono::Utc::now(),
                                                local_address: local_addr.clone(),
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
                                        let _ = self.tx.blocking_send(NetworkEvent {
                                            timestamp: chrono::Utc::now(),
                                            local_address: local_addr,
                                            remote_address: remote_addr,
                                            remote_port,
                                            protocol: "TCP".to_string(),
                                            process_id,
                                            process_name: Some(process_name.to_string()),
                                            geolocation: None,
                                            domain_name: None,
                                            data_sent_bytes: None,
                                            data_received_bytes: None,
                                            event_type: NetworkEventType::ForbiddenPortTraffic,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
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
        #[cfg(target_os = "linux")]
        {
            // Try to use resolvectl which is common on systemd systems
            let output = tokio::process::Command::new("resolvectl")
                .args(["query", "anydesk.com"]) // This is more of a query than a cache dump, but Linux doesn't have a unified cache dump
                .output()
                .await;
            
            if let Ok(output) = output {
                if output.status.success() {
                    let _ = self.tx.blocking_send(NetworkEvent {
                        timestamp: chrono::Utc::now(),
                        local_address: String::new(),
                        remote_address: String::new(),
                        remote_port: 0,
                        protocol: "DNS".to_string(),
                        process_id: 0,
                        process_name: None,
                        geolocation: None,
                        domain_name: Some("anydesk.com (Linux Query)".to_string()),
                        data_sent_bytes: None,
                        data_received_bytes: None,
                        event_type: NetworkEventType::SuspiciousDNS,
                    });
                }
            }
        }

        Ok(())
    }

    pub async fn geolocate_ip(&mut self, ip: &str) -> Option<GeoLocation> {
        if self.geoloc_cache.contains_key(ip) {
            return self.geoloc_cache.get(ip).cloned();
        }

        // 1. Try local MaxMind DB
        if let Some(path) = &self.geoip_db_path {
            if let Ok(reader) = maxminddb::Reader::open_readfile(path) {
                let ip_addr: std::net::IpAddr = ip.parse().ok()?;
                if let Ok(city) = reader.lookup::<maxminddb::geoip2::City>(ip_addr) {
                    let loc = GeoLocation {
                        country: city.country.and_then(|c| c.names).and_then(|n| n.get("en")).unwrap_or(&"Unknown").to_string(),
                        city: city.city.and_then(|c| c.names).and_then(|n| n.get("en")).unwrap_or(&"Unknown").to_string(),
                        isp: "MaxMind DB".to_string(),
                    };
                    self.geoloc_cache.insert(ip.to_string(), loc.clone());
                    return Some(loc);
                }
            }
        }

        // 2. Fallback to API
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
