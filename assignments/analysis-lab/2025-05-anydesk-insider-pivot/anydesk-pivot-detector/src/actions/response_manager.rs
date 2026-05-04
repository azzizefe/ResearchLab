use crate::config::ActiveResponseSettings;
use crate::models::alert::{Alert, AlertSeverity};
use crate::errors::app_error::AppError;
use sysinfo::{System, Pid};
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
        }
        Ok(())
    }

    async fn execute_process_kill(&mut self, alert: &Alert) -> Result<(), AppError> {
        // Look for suspicious processes mentioned in the alert or AnyDesk itself
        self.sys.refresh_all();
        
        println!("{} Initiating active response: Process Kill", "[ACTION]".magenta().bold());
        
        // This is a simplified logic: in a real app, we'd pass the PID from the monitor to the alert
        // For now, we'll look for AnyDesk processes if it's a critical AnyDesk-related pivot
        for (pid, process) in self.sys.processes() {
            let name = process.name().to_string_lossy().to_lowercase();
            if name.contains("anydesk") {
                if !self.settings.auto_confirm {
                    println!("{} Killing process {} (PID: {:?})? [y/N]", "[CONFIRM]".yellow(), name, pid);
                    // In a headless environment, we might skip this or use auto_confirm
                    if !self.settings.auto_confirm {
                        continue; 
                    }
                }
                
                println!("{} Killing suspicious process: {} (PID: {:?})", "[KILL]".red(), name, pid);
                process.kill();
            }
        }
        
        Ok(())
    }

    async fn execute_ip_blocking(&self, alert: &Alert) -> Result<(), AppError> {
        // Simplified IP blocking logic
        // In a real app, parse remote_address from alert metadata and run firewall commands
        println!("{} Initiating active response: IP Blocking (Not fully implemented)", "[ACTION]".magenta().bold());
        Ok(())
    }
}
