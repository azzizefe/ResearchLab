use anydesk_pivot_detector::analyzers::PivotDetector;

#[test]
fn test_scenario_4_false_positive_check() {
    let detector = PivotDetector::new();
    
    // Simulate normal, non-threatening log lines
    let normal_lines = vec![
        "2025-05-01 10:00:00.000  info anydesk - Application started",
        "2025-05-01 10:05:00.000  info anydesk - Looking for local services",
        "2025-05-01 10:10:00.000  info anydesk - Log file rotated",
    ];
    
    let mut all_alerts = Vec::new();
    for line in normal_lines {
        all_alerts.extend(detector.analyze_line(line));
    }
    
    // Verification: Low severity startup alert is acceptable, but no high/critical
    assert!(all_alerts.iter().all(|a| a.severity != anydesk_pivot_detector::models::alert::AlertSeverity::High));
    assert!(all_alerts.iter().all(|a| a.severity != anydesk_pivot_detector::models::alert::AlertSeverity::Critical));
}
