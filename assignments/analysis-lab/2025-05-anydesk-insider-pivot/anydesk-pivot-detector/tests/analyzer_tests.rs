use anydesk_pivot_detector::analyzers::{AnomalyScorer, RuleEngine};
use anydesk_pivot_detector::config::{
    AnyDeskSettings, AppConfig, AppSettings, MonitorSettings, NetworkSettings, ReportingSettings,
};
use anydesk_pivot_detector::models::alert::AlertSeverity;
use chrono::{TimeZone, Utc};

#[test]
fn test_anomaly_scorer_calculation() {
    let mut scorer = AnomalyScorer::new(50.0);
    let session_id = "123456789";

    // Add multiple alerts and check risk score
    let alert1 = create_mock_alert(AlertSeverity::Low);
    let alert2 = create_mock_alert(AlertSeverity::Medium);
    let alert3 = create_mock_alert(AlertSeverity::High);

    scorer.score_alert(&alert1, session_id); // weight ~10
    scorer.score_alert(&alert2, session_id); // weight ~30
    let final_score = scorer.score_alert(&alert3, session_id); // weight ~60

    assert!(final_score >= 100.0);
    assert_eq!(scorer.check_risk_level(final_score), "CRITICAL PIVOT RISK");
}

#[test]
fn test_rule_engine_off_hours() {
    let config = create_mock_config();
    let engine = RuleEngine::new(config);

    // 2:00 AM Utc
    let timestamp = Utc.with_ymd_and_hms(2025, 5, 1, 2, 0, 0).unwrap();
    let alert = engine.check_working_hours(timestamp);

    assert!(alert.is_some());
    assert_eq!(alert.unwrap().title, "Outside Working Hours Connection");
}

fn create_mock_alert(severity: AlertSeverity) -> anydesk_pivot_detector::models::alert::Alert {
    anydesk_pivot_detector::models::alert::Alert {
        id: "test".to_string(),
        timestamp: Utc::now(),
        severity,
        title: "Test Alert".to_string(),
        description: "Test Description".to_string(),
        source_module: "Test".to_string(),
        evidence: serde_json::json!({}),
    }
}

fn create_mock_config() -> AppConfig {
    AppConfig {
        app: AppSettings {
            name: "Test".into(),
            log_level: "info".into(),
            report_output_dir: "reports".into(),
        },
        anydesk: AnyDeskSettings {
            trace_path: "".into(),
            system_conf_path: "".into(),
            service_conf_path: "".into(),
            svc_trace_path: "".into(),
        },
        monitor: MonitorSettings {
            interval_secs: 5,
            suspicious_processes: vec!["cmd.exe".into()],
            alert_threshold: 50,
            working_hour_start: 9,
            working_hour_end: 17,
        },
        network: NetworkSettings {
            blocked_domains: vec![],
            blocked_ports: vec![],
            allowed_anydesk_ids: vec!["111222333".into()],
        },
        reporting: ReportingSettings {
            format: "json".into(),
            enable_syslog: false,
            syslog_server: "".into(),
        },
    }
}
