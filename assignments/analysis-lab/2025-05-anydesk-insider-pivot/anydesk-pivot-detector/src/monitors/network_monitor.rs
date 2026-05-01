use std::time::Duration;
use tokio::time::sleep;
use crate::models::network_event::{NetworkEvent, NetworkEventType, GeoLocation};
use crate::errors::app_error::AppError;
use sysinfo::{System, ProcessRefreshKind, RefreshKind, UpdateKind};
use std::collections::HashMap;
use serde_json::Value;
use chrono::Utc;
use colored::Colorize;

pub struct NetworkMonitor {
    sys: System,
    geoloc_cache: HashMap<String, GeoLocation>,
    forbidden_ports: Vec<u16>,
    high_traffic_threshold: u64, // bytes
}

impl NetworkMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing().with_user(UpdateKind::Always))
        );
        sys.refresh_all();
        
        Self {
            sys,
            geoloc_cache: HashMap::new(),
            forbidden_ports: vec![22, 23, 445, 3389], // SSH, Telnet, SMB, RDP
            high_traffic_threshold: 10 * 1024 * 1024, // 10MB threshold for demo
        }
    }

    pub async fn run(&mut self) -> Result<(), AppError> {
        println!("{}", "Starting Network Monitor...".green().bold());
        loop {
            self.sys.refresh_all();
            
            // 1. Check active connections (AnyDesk ports + Forbidden ports)
            if let Err(e) = self.check_connections().await {
                eprintln!("[ERROR] Connection check failed: {}", e);
            }
            
            // 2. Check DNS cache for AnyDesk domains
            if let Err(e) = self.check_dns_cache().await {
                eprintln!("[ERROR] DNS cache check failed: {}", e);
            }

            sleep(Duration::from_secs(10)).await;
        }
    }

    async fn check_connections(&mut self) -> Result<(), AppError> {
        let output = tokio::process::Command::new("powershell")
            .args(&["-Command", "Get-NetTCPConnection | Select-Object LocalAddress, LocalPort, RemoteAddress, RemotePort, State, OwningProcess | ConvertTo-Json"])
            .output()
            .await
            .map_err(|e| AppError::MonitorError(format!("Failed to run powershell: {}", e)))?;

        if !output.status.success() {
            return Ok(());
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        if json_str.trim().is_empty() { return Ok(()); }

        let connections: Value = serde_json::from_str(&json_str).unwrap_or(Value::Null);

        let process_conns = |conn: &Value| {
            let remote_port = conn["RemotePort"].as_u64().unwrap_or(0) as u16;
            let remote_addr = conn["RemoteAddress"].as_str().unwrap_or("").to_string();
            let process_id = conn["OwningProcess"].as_u64().unwrap_or(0) as u32;
            let local_addr = conn["LocalAddress"].as_str().unwrap_or("").to_string();

            // Ignore local/loopback for some checks
            if remote_addr == "0.0.0.0" || remote_addr == "127.0.0.1" || remote_addr == "::" {
                return;
            }

            // A. Detect AnyDesk connections (Ports 6568, 443)
            if remote_port == 6568 || remote_port == 443 {
                let is_anydesk = self.sys.process(sysinfo::Pid::from_u32(process_id))
                    .map(|p| p.name().to_string_lossy().to_lowercase().contains("anydesk"))
                    .unwrap_or(false);

                if is_anydesk {
                    println!("{} AnyDesk connection: {}:{} (PID: {})", 
                        "[NETWORK]".cyan(), remote_addr, remote_port, process_id);
                    
                    // Trigger geolocation (async background task would be better but we'll do it simple here)
                    // We can't easily await inside this closure if it's not async, so we'll just log IP for now
                    // and maybe do a batch geoloc later.
                }
            }

            // B. Forbidden Ports
            if self.forbidden_ports.contains(&remote_port) {
                println!("{} Traffic on forbidden port {} to {} (PID: {})", 
                    "[ALERT]".red().bold(), remote_port, remote_addr, process_id);
            }
        };

        match connections {
            Value::Array(list) => {
                for conn in list { process_conns(&conn); }
            },
            Value::Object(conn) => {
                process_conns(&Value::Object(conn));
            },
            _ => {}
        }

        Ok(())
    }

    async fn check_dns_cache(&mut self) -> Result<(), AppError> {
        let output = tokio::process::Command::new("powershell")
            .args(&["-Command", "Get-DnsClientCache | Where-Object Name -like '*.net.anydesk.com' | Select-Object Name, Data, Type | ConvertTo-Json"])
            .output()
            .await
            .map_err(|e| AppError::MonitorError(format!("Failed to run powershell: {}", e)))?;

        let json_str = String::from_utf8_lossy(&output.stdout);
        if json_str.trim().is_empty() || json_str.trim() == "null" { return Ok(()); }
        
        let dns_entries: Value = serde_json::from_str(&json_str).unwrap_or(Value::Null);
        
        let process_dns = |entry: &Value| {
            let name = entry["Name"].as_str().unwrap_or("unknown");
            println!("{} AnyDesk DNS query detected: {}", "[DNS]".yellow(), name);
        };

        match dns_entries {
            Value::Array(entries) => {
                for entry in entries { process_dns(&entry); }
            }
            Value::Object(entry) => {
                process_dns(&Value::Object(entry));
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn geolocate_ip(&mut self, ip: &str) -> Option<GeoLocation> {
        if self.geoloc_cache.contains_key(ip) {
            return self.geoloc_cache.get(ip).cloned();
        }

        // Simulating Geolocation since we added reqwest
        let url = format!("http://ip-api.com/json/{}?fields=status,country,city,isp", ip);
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
