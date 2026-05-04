use crate::config::ActiveResponseSettings;
use crate::models::alert::{Alert, AlertSeverity};
use crate::errors::app_error::AppError;
use sysinfo::System;
use colored::Colorize;

pub struct ResponseManager {
    settings: ActiveResponseSettings,
    sys: System,
}

impl ResponseManager {
    pub fn new(settings: ActiveResponseSettings) -> Self {
        Self {
            settings,
            sys: System::new_all(),
        }
    }

    pub async fn handle_alert(&mut self, alert: &Alert) -> Result<(), AppError> {
        if alert.severity == AlertSeverity::Critical {
            if self.settings.enable_process_kill {
                self.execute_process_kill(alert).await?;
            }
            
            if self.settings.enable_ip_blocking {
                self.execute_ip_blocking(alert).await?;
            }

            // Always attempt session termination for critical alerts if enabled (could be a separate flag)
            self.execute_session_termination(alert).await?;
        }
        Ok(())
    }

    async fn confirm_action(&self, action_desc: &str) -> bool {
        if self.settings.auto_confirm {
            return true;
        }
        
        print!("{} Proceed with: {}? [y/N]: ", "[CONFIRM]".yellow().bold(), action_desc);
        use std::io::Write;
        let _ = std::io::stdout().flush();
        
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            return input.trim().to_lowercase() == "y";
        }
        false
    }

    async fn execute_process_kill(&mut self, _alert: &Alert) -> Result<(), AppError> {
        self.sys.refresh_all();
        
        for (pid, process) in self.sys.processes() {
            let name = process.name().to_string_lossy().to_lowercase();
            if name.contains("anydesk") {
                let desc = format!("Killing process {} (PID: {:?})", name, pid);
                if self.confirm_action(&desc).await {
                    println!("{} Killing suspicious process: {} (PID: {:?})", "[KILL]".red(), name, pid);
                    process.kill();
                }
            }
        }
        
        Ok(())
    }

    async fn execute_ip_blocking(&self, alert: &Alert) -> Result<(), AppError> {
        let remote_ip = alert.evidence["remote_address"].as_str()
            .or_else(|| alert.evidence["remote_ip"].as_str());

        if let Some(ip) = remote_ip {
            let desc = format!("Blocking IP address: {}", ip);
            if self.confirm_action(&desc).await {
                println!("{} Blocking IP: {}", "[BLOCK]".red(), ip);
                
                #[cfg(windows)]
                {
                    let _ = tokio::process::Command::new("netsh")
                        .args(["advfirewall", "firewall", "add", "rule", 
                               &format!("name=BlockAnyDeskPivot_{}", ip), 
                               "dir=in", "action=block", &format!("remoteip={}", ip)])
                        .output()
                        .await;
                }

                #[cfg(target_os = "linux")]
                {
                    let _ = tokio::process::Command::new("iptables")
                        .args(["-A", "INPUT", "-s", ip, "-j", "DROP"])
                        .output()
                        .await;
                }
            }
        }
        Ok(())
    }

    async fn execute_session_termination(&self, _alert: &Alert) -> Result<(), AppError> {
        let desc = "Terminating all active AnyDesk sessions";
        if self.confirm_action(desc).await {
            println!("{} Terminating AnyDesk sessions...", "[SESSION]".red());
            
            #[cfg(windows)]
            {
                let anydesk_path = "C:\\Program Files (x86)\\AnyDesk\\AnyDesk.exe";
                let _ = tokio::process::Command::new(anydesk_path)
                    .arg("--remove-password")
                    .output()
                    .await;
            }

            #[cfg(not(windows))]
            {
                // Placeholder for Linux/macOS AnyDesk session termination
                println!("{} Session termination not yet implemented for this platform", "[INFO]".dimmed());
            }
        }
        Ok(())
    }
}
