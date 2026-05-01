use crate::errors::app_error::AppError;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan `AnyDesk` logs for historical pivot activity
    Scan {
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Start real-time monitoring
    Monitor,
    /// Generate a report from previous scans
    Report {
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Check `AnyDesk` security configuration
    ConfigCheck,
    /// Apply hardening recommendations
    Harden,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub app: AppSettings,
    pub anydesk: AnyDeskSettings,
    pub monitor: MonitorSettings,
    pub network: NetworkSettings,
    pub reporting: ReportingSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub name: String,
    pub log_level: String,
    pub report_output_dir: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnyDeskSettings {
    pub trace_path: String,
    pub system_conf_path: String,
    pub service_conf_path: String,
    pub svc_trace_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonitorSettings {
    pub interval_secs: u64,
    pub suspicious_processes: Vec<String>,
    pub alert_threshold: u32,
    pub working_hour_start: u32,
    pub working_hour_end: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkSettings {
    pub blocked_domains: Vec<String>,
    pub blocked_ports: Vec<u16>,
    pub allowed_anydesk_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReportingSettings {
    pub format: String,
    pub enable_syslog: bool,
    pub syslog_server: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, AppError> {
        // Load .env file
        dotenv::dotenv().ok();

        // Load default.toml
        let config_str = std::fs::read_to_string("config/default.toml")
            .map_err(|e| AppError::ConfigError(format!("Failed to read config file: {e}")))?;

        let mut config: AppConfig = toml::from_str(&config_str)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse config file: {e}")))?;

        // Override with environment variables
        if let Ok(val) = std::env::var("APP_LOG_LEVEL") {
            config.app.log_level = val;
        }
        if let Ok(val) = std::env::var("ANYDESK_TRACE_PATH") {
            config.anydesk.trace_path = val;
        }
        // ... (add other overrides as needed)

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), AppError> {
        // Basic validation: check if paths exist (just examples)
        // Note: %AppData% needs to be expanded on Windows
        Ok(())
    }
}
