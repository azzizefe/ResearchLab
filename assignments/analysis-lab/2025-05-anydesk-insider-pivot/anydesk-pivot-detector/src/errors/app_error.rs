use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    /// 13.1.2: `ConfigError` (gecersiz ayar, dosya bulunamadi)
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Standard IO conversions
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// 13.1.1: `ParseError` (gecersiz log formati, eksik alan)
    #[error("Parse error: {0}")]
    ParseError(String),

    /// 13.1.4: `AnalyzerError` (kural motoru hatasi)
    #[error("Analyzer error: {0}")]
    AnalyzerError(String),

    /// 13.1.3: `MonitorError` (izleme basarisiz, erisim engellendi)
    #[error("Monitoring error: {0}")]
    MonitorError(String),

    /// 13.1.5: `ReportError` (dosya yazma hatasi, syslog baglanti hatasi)
    #[error("Report error: {0}")]
    ReportError(String),

    /// 13.1.6: `TauriError` (IPC hatasi, state hatasi)
    #[error("Tauri error: {0}")]
    TauriError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    /// Environment and dependency errors
    #[error("Environment error: {0}")]
    EnvError(#[from] dotenv::Error),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("Unknown error")]
    Unknown,
}
