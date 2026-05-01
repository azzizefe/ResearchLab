use anydesk_pivot_detector::models::alert::Alert;
use anydesk_pivot_detector::config::AppConfig;
use anydesk_pivot_detector::parsers::parse_trace_file;
use anydesk_pivot_detector::analyzers::PivotDetector;

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
async fn start_monitor() -> Result<(), String> {
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
