use anydesk_pivot_detector::config::{AppConfig, Cli, Commands};
use anydesk_pivot_detector::monitors::{ProcessMonitor, FileWatcher, NetworkMonitor};
use anydesk_pivot_detector::analyzers::{PivotDetector, RuleEngine, AnomalyScorer};
use anydesk_pivot_detector::reporters::{JsonReporter, ConsoleReporter};
use anydesk_pivot_detector::parsers::{parse_system_conf, parse_trace_file};
use clap::Parser;
use tokio::sync::mpsc;
use tokio::signal;
use colored::Colorize;
use std::path::PathBuf;

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

    let console = ConsoleReporter;
    let json_reporter = JsonReporter::new(config.app.report_output_dir.clone());
    let rule_engine = RuleEngine::new(config.clone());
    let mut scorer = AnomalyScorer::new(config.monitor.alert_threshold as f32 * 10.0);
    let detector = PivotDetector::new();

    println!("{} {}", "AnyDesk Pivot Detector:".blue().bold(), config.app.name.cyan());
    
    // 3. Dispatch commands
    match cli.command {
        // 10.1.1: `scan` - Tek seferlik log tarama ve analiz
        Some(Commands::Scan { path }) => {
            let scan_path = path.unwrap_or_else(|| config.anydesk.trace_path.clone().into());
            println!("{} {:?}", "Scanning logs at:".yellow(), scan_path);
            
            let connections = parse_trace_file(&scan_path)?;
            let mut all_alerts = Vec::new();

            // Real analysis would involve more than just connections, but we'll use detector for now
            let content = std::fs::read_to_string(&scan_path).unwrap_or_default();
            for line in content.lines() {
                let alerts = detector.analyze_line(line);
                all_alerts.extend(alerts);
            }

            console.report(&all_alerts);
            let report_path = json_reporter.report(&all_alerts)?;
            println!("{} {}", "Report saved to:".green(), report_path);
        }

        // 10.1.2: `monitor` - Surekli canli izleme modu
        Some(Commands::Monitor) => {
            println!("{}", "Starting real-time monitoring system...".green().bold());
            
            let (tx, mut rx) = mpsc::channel(100);

            let mut process_monitor = ProcessMonitor::new(config.monitor.suspicious_processes.clone());
            let mut file_watcher = FileWatcher::new(tx)?;
            let mut network_monitor = NetworkMonitor::new();

            file_watcher.watch(&config.anydesk.trace_path)?;

            // 10.3: Async task orchestration (tokio)
            let process_handle = tokio::spawn(async move {
                let _ = process_monitor.run().await;
            });

            let network_handle = tokio::spawn(async move {
                let _ = network_monitor.run().await;
            });

            let file_handle = tokio::spawn(async move {
                while let Some(event) = rx.recv().await {
                    let new_lines = file_watcher.handle_event(event);
                    for line in new_lines {
                        let alerts = detector.analyze_line(&line);
                        if !alerts.is_empty() {
                            console.report(&alerts);
                            let _ = json_reporter.report(&alerts);
                        }
                    }
                }
            });

            println!("{}", "Monitoring active. Press Ctrl+C to stop.".dimmed());
            
            // 10.2: Graceful shutdown (Ctrl+C) destegi
            tokio::select! {
                _ = signal::ctrl_c() => {
                    println!("\n{}", "Shutdown signal received. Exiting...".yellow());
                }
                _ = async {
                    let _ = tokio::join!(process_handle, network_handle, file_handle);
                } => {}
            }
        }

        // 10.1.3: `report` - Gecmis tarama sonuclarini raporla
        Some(Commands::Report { output }) => {
            let report_dir = output.unwrap_or_else(|| PathBuf::from(&config.app.report_output_dir));
            println!("Listing reports in: {:?}", report_dir);
            if let Ok(entries) = std::fs::read_dir(report_dir) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name();
                    let metadata = entry.metadata()?;
                    println!("- {:<40} {:>10} bytes", file_name.to_string_lossy(), metadata.len());
                }
            }
        }

        // 10.1.4: `config-check`
        Some(Commands::ConfigCheck) => {
            println!("{}", "Checking AnyDesk security configuration...".yellow().bold());
            match parse_system_conf(&config.anydesk.system_conf_path) {
                Ok(sys_conf) => {
                    println!("- AnyDesk ID: {:?}", sys_conf.anydesk_id.unwrap_or_default().green());
                    println!("- Unattended Access: {}", if sys_conf.unattended_access { "ENABLED (High Risk)".red() } else { "DISABLED".green() });
                    println!("- Interactive Access: {:?}", sys_conf.interactive_access);
                }
                Err(e) => println!("{} Failed to parse config: {}", "[ERROR]".red(), e),
            }
        }

        // 10.1.5: `harden`
        Some(Commands::Harden) => {
            println!("{}", "Generating hardening recommendations...".magenta().bold());
            println!("1. {} Set 'ad.security.unattended_access=false' in system.conf", "[FIX]".cyan());
            println!("2. {} Restrict access to known IDs only in ACL settings.", "[FIX]".cyan());
            println!("3. {} Enable 2FA on the AnyDesk web portal.", "[FIX]".cyan());
        }

        _ => {
            println!("No command specified. Use --help for usage.");
        }
    }

    Ok(())
}
