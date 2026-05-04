use anydesk_pivot_detector::analyzers::{AnomalyScorer, PivotDetector, RuleEngine};
use anydesk_pivot_detector::config::{
    AnyDeskSettings, AppConfig, AppSettings, ElasticsearchSettings, MonitorSettings, NetworkSettings,
    NotificationSettings, ReportingSettings,
};
use anydesk_pivot_detector::models::alert::AlertSeverity;

#[test]
fn test_scenario_1_insider_pivot() {
    let config = create_mock_config();
    let detector = PivotDetector::new();
    let engine = RuleEngine::new(config.clone());
    let mut scorer = AnomalyScorer::new(50.0);

    // 1. Simulate unauthorized ID connection in log
    let line = "2025-05-01 22:00:00.000  info anydesk - Incoming connection from 666 666 666";

    // 2. Detection
    let mut alerts = detector.analyze_line(line);

    // 3. Rule Engine (ACL & Hours)
    if let Some(a) = engine.check_acl("666 666 666") {
        alerts.push(a);
    }
    if let Some(a) = engine.check_working_hours(chrono::Utc::now()) {
        alerts.push(a);
    }

    // 4. Scoring
    let mut final_score = 0.0;
    for alert in &alerts {
        final_score = scorer.score_alert(alert, "666 666 666");
    }

    // 5. Verification
    assert!(alerts.iter().any(|a| a.title == "Unauthorized AnyDesk ID"));
    assert!(final_score >= 100.0); // Medium (Log) + High (ACL) + Medium (Hours)
    assert_eq!(scorer.check_risk_level(final_score), "CRITICAL PIVOT RISK");
}

#[test]
fn test_scenario_2_portable_anydesk() {
    let config = create_mock_config();
    let engine = RuleEngine::new(config);

    // Simulate AnyDesk running from Downloads folder (Portable)
    let path = "C:\\Users\\Public\\Downloads\\AnyDesk.exe";
    let alert = engine.check_portable_anydesk(path);

    assert!(alert.is_some());
    assert_eq!(alert.unwrap().title, "Portable AnyDesk Execution");
}

#[test]
fn test_scenario_3_data_exfiltration() {
    let config = create_mock_config();
    let engine = RuleEngine::new(config);

    // Simulate sensitive file access
    let path = "C:\\Windows\\System32\\config\\SAM";
    let alert = engine.check_sensitive_file_access(path);

    assert!(alert.is_some());
    assert_eq!(alert.unwrap().severity, AlertSeverity::Critical);
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
            suspicious_processes: vec![],
            alert_threshold: 50,
            working_hour_start: 9,
            working_hour_end: 17,
        },
        network: NetworkSettings {
            blocked_domains: vec![],
            blocked_ports: vec![],
            allowed_anydesk_ids: vec!["111222333".into()], // 666 666 666 is NOT allowed
            malicious_ips: vec![],
            high_data_threshold_bytes: 1024,
        },
        reporting: ReportingSettings {
            format: "json".into(),
            enable_syslog: false,
            syslog_server: "".into(),
            syslog_port: 514,
            enable_elasticsearch: false,
        },
        elasticsearch: ElasticsearchSettings {
            url: "http://localhost:9200".into(),
            index: "anydesk-alerts".into(),
        },
        notifications: NotificationSettings {
            enable_slack: false,
            slack_webhook_url: "".into(),
            enable_email: false,
            smtp_server: "".into(),
            smtp_port: 587,
            email_to: "".into(),
            enable_webhook: false,
            webhook_url: "".into(),
        },
    }
}
