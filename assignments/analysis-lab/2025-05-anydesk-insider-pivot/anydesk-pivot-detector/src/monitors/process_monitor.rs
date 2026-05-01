use sysinfo::{System, ProcessRefreshKind, RefreshKind};
use crate::models::process_event::{ProcessEvent, ProcessEventType};
use crate::errors::app_error::AppError;
use std::time::Duration;
use tokio::time::sleep;

pub struct ProcessMonitor {
    sys: System,
    suspicious_list: Vec<String>,
}

impl ProcessMonitor {
    pub fn new(suspicious_list: Vec<String>) -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::new().with_processes(ProcessRefreshKind::new().with_user())
        );
        sys.refresh_all();
        
        Self {
            sys,
            suspicious_list,
        }
    }

    pub async fn run(&mut self) -> Result<(), AppError> {
        loop {
            self.sys.refresh_all();
            let mut events = Vec::new();

            // 1. Identify AnyDesk processes
            let anydesk_pids: Vec<_> = self.sys.processes()
                .iter()
                .filter(|(_, p)| p.name().to_string_lossy().to_lowercase().contains("anydesk"))
                .map(|(pid, _)| *pid)
                .collect();

            // 2. Check for children of AnyDesk or suspicious processes
            for (pid, process) in self.sys.processes() {
                let pid_u32 = pid.as_u32();
                let name = process.name().to_string_lossy();
                let parent_pid = process.parent();

                if let Some(ppid) = parent_pid {
                    // Check if parent is AnyDesk
                    if anydesk_pids.contains(&ppid) {
                        let name_str = name.to_lowercase();
                        if self.suspicious_list.iter().any(|s| name_str.contains(s)) {
                            events.push(ProcessEvent {
                                timestamp: chrono::Utc::now(),
                                pid: pid_u32,
                                name: name.to_string(),
                                path: process.exe().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default(),
                                command_line: Some(process.cmd().iter().map(|s| s.to_string_lossy()).collect::<Vec<_>>().join(" ")),
                                event_type: ProcessEventType::SuspiciousActivity,
                                parent_pid: Some(ppid.as_u32()),
                            });
                        }
                    }
                }
            }

            for event in events {
                println!("[ALERT] Suspicious process detected: {} (PID: {}) spawned by AnyDesk", event.name, event.pid);
                // In a real app, we would send this to an analyzer or reporter
            }

            sleep(Duration::from_secs(5)).await;
        }
    }
}
