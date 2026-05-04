#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, clippy::unwrap_used)]
#![allow(
    clippy::too_many_lines,
    clippy::items_after_statements,
    clippy::cast_precision_loss,
    clippy::cloned_ref_to_slice_refs,
    clippy::unnecessary_debug_formatting
)]
use anydesk_pivot_detector::analyzers::{AnomalyScorer, PivotDetector, RuleEngine};
use anydesk_pivot_detector::config::{AppConfig, Cli, Commands};
use anydesk_pivot_detector::actions::ResponseManager;
use anydesk_pivot_detector::utils::metrics::MetricsManager;
use anydesk_pivot_detector::utils::geoip_updater;
use anydesk_pivot_detector::utils::wal::WriteAheadLog;
use anydesk_pivot_detector::monitors::{FileWatcher, NetworkMonitor, ProcessMonitor};
use anydesk_pivot_detector::parsers::parse_system_conf;
use anydesk_pivot_detector::reporters::{
    ConsoleReporter, ElasticsearchReporter, JsonReporter, NotificationManager, SyslogProtocol,
    SyslogReporter,
};
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;
use tokio::signal;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse CLI arguments
    // 1. Initialize Tracing (Structured Logging)
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    tracing::info!("Starting AnyDesk Pivot Detector");

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
    let es_reporter = std::sync::Arc::new(ElasticsearchReporter::new(
        config.elasticsearch.url.clone(),
        config.elasticsearch.index.clone(),
    ));
    let syslog_reporter = std::sync::Arc::new(SyslogReporter::new(
        format!("{}:{}", config.reporting.syslog_server, config.reporting.syslog_port),
        SyslogProtocol::Udp,
    ));
    let notification_manager = std::sync::Arc::new(NotificationManager::new(config.notifications.clone()));
    let detector = PivotDetector::new();

    println!(
        "{} {}",
        "AnyDesk Pivot Detector:".blue().bold(),
        config.app.name.cyan()
    );

    // 3. Dispatch commands
    match cli.command {
        // 10.1.1: `scan` - Tek seferlik log tarama ve analiz
        Some(Commands::Scan { path }) => {
            let scan_path = path.unwrap_or_else(|| config.anydesk.trace_path.clone().into());
            println!("{} {:?}", "Scanning logs at:".yellow(), scan_path);

            let rule_engine = RuleEngine::new(config.clone());
            let mut scorer = AnomalyScorer::new(config.monitor.alert_threshold as f32);
            let mut all_alerts = Vec::new();

            // 1. Analyze logs for patterns
            let content = std::fs::read_to_string(&scan_path).map_err(|e| {
                anyhow::anyhow!("Failed to read log file: {e}")
            })?;
            for line in content.lines() {
                let alerts = detector.analyze_line(line);
                for alert in alerts {
                    let _ = scorer.score_alert(&alert, "SCAN_SESSION");
                    all_alerts.push(alert);
                }
                if let Some(alert) = rule_engine.check_unattended_access(line) {
                    let _ = scorer.score_alert(&alert, "SCAN_SESSION");
                    all_alerts.push(alert);
                }
            }

            console.report(&all_alerts);
            let report_path = json_reporter.report(&all_alerts)?;
            println!("{} {}", "Report saved to:".green(), report_path);

            if config.reporting.enable_elasticsearch {
                let es = es_reporter.clone();
                let alerts = all_alerts.clone();
                if let Err(e) = es.report(&alerts).await {
                    tracing::error!("Failed to report to Elasticsearch: {}", e);
                }
            }
        }

        // 10.1.2: `monitor` - Surekli canli izleme modu
        Some(Commands::Monitor) => {
            tracing::info!(target: "monitor", "Starting real-time monitoring system...");

            let (file_tx, mut file_rx) = mpsc::channel(100);
            let (proc_tx, mut proc_rx) = mpsc::channel(100);
            let (net_tx, mut net_rx) = mpsc::channel(100);

            let mut process_monitor = ProcessMonitor::new(config.monitor.suspicious_processes.clone(), proc_tx);
            let mut file_watcher = FileWatcher::new(file_tx)?;
            let mut network_monitor = NetworkMonitor::new(net_tx, config.network.geoip_db_path.clone());

            file_watcher.watch(&config.anydesk.trace_path)?;

            let rule_engine = std::sync::Arc::new(RuleEngine::new(config.clone()));
            let mut scorer = AnomalyScorer::new(config.monitor.alert_threshold as f32);

            // 10.3: Async task orchestration (tokio)
            let process_handle = tokio::spawn(async move {
                let _ = process_monitor.run().await;
            });

            let network_handle = tokio::spawn(async move {
                let _ = network_monitor.run().await;
            });

            // 16.3: GeoIP Background Updater (Every 7 days)
            let geoip_config = config.clone();
            tokio::spawn(async move {
                loop {
                    if let (Some(url), Some(key), Some(path)) = (
                        &geoip_config.network.geoip_update_url,
                        &geoip_config.network.maxmind_license_key,
                        &geoip_config.network.geoip_db_path,
                    ) {
                        if key != "YOUR_LICENSE_KEY_HERE" {
                            if let Err(e) = geoip_updater::update_geoip_db(url, key, path).await {
                                tracing::error!("Failed to update GeoIP database: {}", e);
                            }
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(7 * 24 * 60 * 60)).await;
                }
            });

            let reporter_es = es_reporter.clone();
            let reporter_syslog = syslog_reporter.clone();
            let notifications = notification_manager.clone();
            let rule_engine_loop = rule_engine.clone();
            let metrics = MetricsManager::new();
            let metrics_loop = metrics.clone();
            let response_manager = std::sync::Arc::new(tokio::sync::Mutex::new(ResponseManager::new(config.active_response.clone())));
            let response_manager_loop = response_manager.clone();

            let wal = std::sync::Arc::new(WriteAheadLog::new("reports/pending_logs.wal")?);
            let wal_startup = wal.clone();
            
            let event_loop = tokio::spawn(async move {
                // Initial check for pending logs in WAL (e.g. after a crash)
                if wal_startup.has_content() {
                    if let Ok(pending_lines) = wal_startup.read_and_clear() {
                        println!("{} Processing {} pending log lines from WAL...", "[RECOVERY]".yellow(), pending_lines.len());
                        for line in pending_lines {
                            let alerts = detector.analyze_line(&line);
                            for alert in alerts {
                                let score = scorer.score_alert(&alert, "RECOVERY_SESSION");
                                handle_alert(&alert, score, &config, &console, &reporter_es, &reporter_syslog, &notifications, &response_manager_loop, &metrics_loop).await;
                            }
                        }
                    }
                }

                loop {
                    tokio::select! {
                        Some(event) = file_rx.recv() => {
                            let new_lines = file_watcher.handle_event(event);
                            if !new_lines.is_empty() {
                                // 17.2: Buffering/WAL
                                let _ = wal.append(&new_lines);
                                
                                for line in new_lines {
                                    let alerts = detector.analyze_line(&line);
                                    for alert in alerts {
                                        let score = scorer.score_alert(&alert, "CLI_SESSION");
                                        handle_alert(&alert, score, &config, &console, &reporter_es, &reporter_syslog, &notifications, &response_manager_loop, &metrics_loop).await;
                                    }
                                }
                                
                                // Clear WAL after successful batch processing
                                let _ = wal.read_and_clear();
                            }
                        }
                        Some(event) = proc_rx.recv() => {
                            if let Some(alert) = rule_engine_loop.check_suspicious_process(&event) {
                                let score = scorer.score_alert(&alert, "PROC_SESSION");
                                handle_alert(&alert, score, &config, &console, &reporter_es, &reporter_syslog, &notifications, &response_manager_loop, &metrics_loop).await;
                            }
                        }
                        Some(event) = net_rx.recv() => {
                            if let Some(alert) = rule_engine_loop.check_network_scanning(&event) {
                                let score = scorer.score_alert(&alert, "NET_SESSION");
                                handle_alert(&alert, score, &config, &console, &reporter_es, &reporter_syslog, &notifications, &response_manager_loop, &metrics_loop).await;
                            }
                        }
                    }
                }
            });

            async fn handle_alert(
                alert: &anydesk_pivot_detector::models::alert::Alert,
                score: f32,
                config: &AppConfig,
                console: &ConsoleReporter,
                reporter_es: &std::sync::Arc<ElasticsearchReporter>,
                reporter_syslog: &std::sync::Arc<SyslogReporter>,
                notifications: &std::sync::Arc<NotificationManager>,
                response_manager: &std::sync::Arc<tokio::sync::Mutex<ResponseManager>>,
                metrics: &MetricsManager,
            ) {
                metrics.inc_alerts();
                console.report(&[alert.clone()]);
                
                if config.reporting.enable_elasticsearch {
                    let _ = reporter_es.report(&[alert.clone()]).await;
                }

                if score >= config.monitor.alert_threshold as f32 {
                    println!("{} Score: {}", "CRITICAL RISK DETECTED!".red().bold(), score);
                    
                    // 9.1: Notifications
                    let alert_clone = alert.clone();
                    let notifications_clone = notifications.clone();
                    tokio::spawn(async move {
                        if let Err(e) = notifications_clone.notify(&alert_clone).await {
                            tracing::error!("Failed to send notification: {}", e);
                        }
                    });

                    // 9.2: SIEM (Syslog)
                    if config.reporting.enable_syslog {
                        if let Err(e) = reporter_syslog.send(alert, true) {
                            tracing::error!("Failed to send syslog: {}", e);
                        }
                    }

                    // 15: Active Response
                    let mut manager: tokio::sync::MutexGuard<ResponseManager> = response_manager.lock().await;
                    if let Err(e) = manager.handle_alert(alert).await {
                        tracing::error!("Failed to execute active response: {}", e);
                    }
                }
            }

            println!("{}", "Monitoring active. Press Ctrl+C to stop.".dimmed());

            // 10.2: Graceful shutdown (Ctrl+C) destegi
            tokio::select! {
                _ = signal::ctrl_c() => {
                    println!("\n{}", "Shutdown signal received. Exiting...".yellow());
                }
                () = async {
                    let _ = tokio::join!(process_handle, network_handle, event_loop);
                } => {}
            }
        }

        // 10.1.3: `report` - Gecmis tarama sonuclarini raporla
        Some(Commands::Report { output }) => {
            let report_dir = output.unwrap_or_else(|| PathBuf::from(&config.app.report_output_dir));
            println!("Listing reports in: {report_dir:?}");
            if let Ok(entries) = std::fs::read_dir(report_dir) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name();
                    let metadata = entry.metadata()?;
                    println!(
                        "- {:<40} {:>10} bytes",
                        file_name.to_string_lossy(),
                        metadata.len()
                    );
                }
            }
        }

        // 10.1.4: `config-check`
        Some(Commands::ConfigCheck) => {
            println!(
                "{}",
                "Checking AnyDesk security configuration...".yellow().bold()
            );
            match parse_system_conf(&config.anydesk.system_conf_path) {
                Ok(sys_conf) => {
                    println!(
                        "- AnyDesk ID: {:?}",
                        sys_conf.anydesk_id.unwrap_or_default().green()
                    );
                    println!(
                        "- Unattended Access: {}",
                        if sys_conf.unattended_access {
                            "ENABLED (High Risk)".red()
                        } else {
                            "DISABLED".green()
                        }
                    );
                    println!("- Interactive Access: {:?}", sys_conf.interactive_access);
                }
                Err(e) => println!("{} Failed to parse config: {}", "[ERROR]".red(), e),
            }
        }

        // 10.1.5: `harden`
        Some(Commands::Harden) => {
            println!(
                "{}",
                "Generating hardening recommendations...".magenta().bold()
            );
            println!(
                "1. {} Set 'ad.security.unattended_access=false' in system.conf",
                "[FIX]".cyan()
            );
            println!(
                "2. {} Restrict access to known IDs only in ACL settings.",
                "[FIX]".cyan()
            );
            println!(
                "3. {} Enable 2FA on the AnyDesk web portal.",
                "[FIX]".cyan()
            );
        }

        _ => {
            println!("No command specified. Use --help for usage.");
        }
    }

    Ok(())
}
