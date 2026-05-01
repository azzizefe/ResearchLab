use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Analysis error: {0}")]
    AnalysisError(String),

    #[error("Monitoring error: {0}")]
    MonitorError(String),

    #[error("Tauri error: {0}")]
    TauriError(String),

    #[error("Unknown error")]
    Unknown,
}
