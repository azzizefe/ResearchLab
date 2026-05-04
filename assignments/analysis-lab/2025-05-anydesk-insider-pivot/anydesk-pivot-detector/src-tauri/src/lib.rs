use anydesk_pivot_detector::models::alert::{Alert, AlertSeverity};
use anydesk_pivot_detector::config::AppConfig;
use anydesk_pivot_detector::parsers::parse_trace_file;
use anydesk_pivot_detector::analyzers::{PivotDetector, RuleEngine, AnomalyScorer};
use anydesk_pivot_detector::monitors::{FileWatcher, ProcessMonitor, NetworkMonitor};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 10.2.1: Guncel uyarilari getir
#[tauri::command]
async fn get_alerts() -> Result<Vec<Alert>, String> {
    Ok(Vec::new())
}

/// 10.2.2: Izlemeyi baslat
#[tauri::command]
async fn start_monitor(app: AppHandle) -> Result<(), String> {
    let config = AppConfig::load().map_err(|e| e.to_string())?;
    
    // Channels for different monitors
    let (file_tx, mut file_rx) = mpsc::channel(100);
    let (proc_tx, mut proc_rx) = mpsc::channel(100);
    let (net_tx, mut net_rx) = mpsc::channel(100);

    // Initialize Analyzers
    let detector = PivotDetector::new();
    let rule_engine = std::sync::Arc::new(RuleEngine::new(config.clone()));
    let scorer = std::sync::Arc::new(tokio::sync::Mutex::new(AnomalyScorer::new(config.monitor.alert_threshold as f32)));

    // Initialize Monitors
    let mut file_watcher = FileWatcher::new(file_tx).map_err(|e| e.to_string())?;
    let mut process_monitor = ProcessMonitor::new(config.monitor.suspicious_processes.clone(), proc_tx);
    let mut network_monitor = NetworkMonitor::new(net_tx);
    
    file_watcher.watch(&config.anydesk.trace_path).map_err(|e| e.to_string())?;

    // Helper to process alerts
    let process_alert = |alert: Alert, app: &AppHandle, scorer: &std::sync::Arc<tokio::sync::Mutex<AnomalyScorer>>| {
        let app_clone = app.clone();
        let scorer_clone = scorer.clone();
        tauri::async_runtime::spawn(async move {
            let mut s = scorer_clone.lock().await;
            let score = s.score_alert(&alert, "LOCAL_SESSION"); // Simplified session ID
            let _ = app_clone.emit("new-alert", &alert);
            let _ = app_clone.emit("risk-update", score);
        });
    };

    // 1. File Watcher Task
    let app_file = app.clone();
    let rule_engine_file = rule_engine.clone();
    let scorer_file = scorer.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(event) = file_rx.recv().await {
            let new_lines = file_watcher.handle_event(event);
            for line in new_lines {
                let _ = app_file.emit("log-entry", &line);
                
                // Detection Module 1: Pivot Detector (Log patterns)
                let alerts = detector.analyze_line(&line);
                for alert in alerts {
                    process_alert(alert, &app_file, &scorer_file);
                }

                // Detection Module 2: Rule Engine (Log-based rules)
                if let Some(alert) = rule_engine_file.check_unattended_access(&line) {
                    process_alert(alert, &app_file, &scorer_file);
                }
            }
        }
    });

    // 2. Process Monitor Task
    let app_proc = app.clone();
    let rule_engine_proc = rule_engine.clone();
    let scorer_proc = scorer.clone();
    tauri::async_runtime::spawn(async move {
        let _ = process_monitor.run().await;
    });
    tauri::async_runtime::spawn(async move {
        while let Some(event) = proc_rx.recv().await {
            if let Some(alert) = rule_engine_proc.check_suspicious_process(&event) {
                process_alert(alert, &app_proc, &scorer_proc);
            }
        }
    });

    // 3. Network Monitor Task
    let app_net = app.clone();
    let rule_engine_net = rule_engine.clone();
    let scorer_net = scorer.clone();
    tauri::async_runtime::spawn(async move {
        let _ = network_monitor.run().await;
    });
    tauri::async_runtime::spawn(async move {
        while let Some(event) = net_rx.recv().await {
            if let Some(alert) = rule_engine_net.check_network_scanning(&event) {
                process_alert(alert, &app_net, &scorer_net);
            }
        }
    });

    Ok(())
}

/// 10.2.3: Izlemeyi durdur
#[tauri::command]
async fn stop_monitor() -> Result<(), String> {
    Ok(())
}

/// 10.2.4: Tek seferlik tarama baslat
#[tauri::command]
async fn run_scan() -> Result<Vec<Alert>, String> {
    let detector = PivotDetector::new();
    let config = AppConfig::load().map_err(|e| e.to_string())?;
    let path = config.anydesk.trace_path;
    
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut alerts = Vec::new();
    for line in content.lines() {
        alerts.extend(detector.analyze_line(line));
    }
    
    Ok(alerts)
}

/// 10.2.5: Mevcut konfigurasyonu getir
#[tauri::command]
fn get_config() -> Result<AppConfig, String> {
    AppConfig::load().map_err(|e| e.to_string())
}

/// 10.2.6: AnyDesk durumunu kontrol et
#[tauri::command]
async fn check_anydesk_status() -> Result<bool, String> {
    Ok(true)
}

/// 10.2.7: Proses agacini getir
#[tauri::command]
async fn get_process_tree() -> Result<Vec<String>, String> {
    Ok(Vec::new())
}

/// 10.2.8: Ag baglantilari getir
#[tauri::command]
async fn get_network_connections() -> Result<Vec<String>, String> {
    Ok(Vec::new())
}

/// 10.2.9: Raporu disa aktar
#[tauri::command]
async fn export_report(format: String) -> Result<String, String> {
    Ok(format!("Report exported as {}", format))
}

/// 10.2.10: Risk skor gecmisi
#[tauri::command]
async fn get_score_history() -> Result<Vec<f32>, String> {
    Ok(Vec::new())
}

/// 10.2.11: Ayarlari guncelle
#[tauri::command]
async fn update_config(strict_mode: bool, monitor_files: bool) -> Result<(), String> {
    println!("Backend Config Updated: Strict={}, MonitorFiles={}", strict_mode, monitor_files);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            get_alerts, 
            start_monitor,
            stop_monitor,
            run_scan, 
            get_config,
            check_anydesk_status,
            get_process_tree,
            get_network_connections,
            export_report,
            get_score_history,
            update_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
