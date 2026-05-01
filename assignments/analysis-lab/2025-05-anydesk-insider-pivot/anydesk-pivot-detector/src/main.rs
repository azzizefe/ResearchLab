use anydesk_pivot_detector::config::{AppConfig, Cli};
use clap::Parser;

fn main() {
    // 1. Parse CLI arguments
    let cli = Cli::parse();

    // 2. Load Configuration
    let config = match AppConfig::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    println!("Starting AnyDesk Pivot Detector: {}", config.app.name);
    
    // 3. Dispatch commands
    match cli.command {
        Some(anydesk_pivot_detector::config::Commands::Scan { path }) => {
            println!("Scanning logs at: {:?}", path.unwrap_or_else(|| config.anydesk.trace_path.into()));
        }
        Some(anydesk_pivot_detector::config::Commands::Monitor) => {
            println!("Starting real-time monitor...");
        }
        _ => {
            println!("No command specified. Use --help for usage.");
        }
    }
}
