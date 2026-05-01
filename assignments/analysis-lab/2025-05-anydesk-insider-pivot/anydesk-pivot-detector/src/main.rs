use anydesk_pivot_detector::config::{AppConfig, Cli, Commands};
use anydesk_pivot_detector::monitors::{ProcessMonitor, FileWatcher, NetworkMonitor};
use clap::Parser;
use tokio::sync::mpsc;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse CLI arguments
    let cli = Cli::parse();

    // 2. Load Configuration
    let config = match AppConfig::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    println!("{} {}", "AnyDesk Pivot Detector:".blue().bold(), config.app.name.cyan());
    
    // 3. Dispatch commands
    match cli.command {
        Some(Commands::Scan { path }) => {
            let scan_path = path.unwrap_or_else(|| config.anydesk.trace_path.clone().into());
            println!("{} {:?}", "Scanning logs at:".yellow(), scan_path);
            // Scan implementation...
        }
        Some(Commands::Monitor) => {
            println!("{}", "Starting real-time monitoring system...".green().bold());
            
            // Channel for file watcher events
            let (tx, mut rx) = mpsc::channel(100);

            // Initialize monitors
            let mut process_monitor = ProcessMonitor::new(config.monitor.suspicious_processes.clone());
            let mut file_watcher = FileWatcher::new(tx)?;
            let mut network_monitor = NetworkMonitor::new();

            // Set up file watching for AnyDesk trace files
            file_watcher.watch(&config.anydesk.trace_path)?;
            if !config.anydesk.svc_trace_path.is_empty() {
                file_watcher.watch(&config.anydesk.svc_trace_path)?;
            }

            // A. Run Process Monitor
            let process_handle = tokio::spawn(async move {
                if let Err(e) = process_monitor.run().await {
                    eprintln!("[ERROR] Process monitor failed: {}", e);
                }
            });

            // B. Run Network Monitor
            let network_handle = tokio::spawn(async move {
                if let Err(e) = network_monitor.run().await {
                    eprintln!("[ERROR] Network monitor failed: {}", e);
                }
            });

            // C. Run File Watcher Handler (in main loop or separate task)
            let file_handle = tokio::spawn(async move {
                while let Some(event) = rx.recv().await {
                    let new_lines = file_watcher.handle_event(event);
                    for line in new_lines {
                        println!("{} {}", "[FILE]".magenta(), line.trim());
                        // Further parsing of lines for pivot events...
                    }
                }
            });

            println!("{}", "Monitoring active. Press Ctrl+C to stop.".dimmed());
            
            // Wait for all handles
            let _ = tokio::join!(process_handle, network_handle, file_handle);
        }
        _ => {
            println!("No command specified. Use --help for usage.");
        }
    }

    Ok(())
}
