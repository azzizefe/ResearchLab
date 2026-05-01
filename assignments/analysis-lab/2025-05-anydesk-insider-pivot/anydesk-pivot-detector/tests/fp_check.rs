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
    assert!(
        all_alerts
            .iter()
            .all(|a| a.severity != anydesk_pivot_detector::models::alert::AlertSeverity::High)
    );
    assert!(
        all_alerts
            .iter()
            .all(|a| a.severity != anydesk_pivot_detector::models::alert::AlertSeverity::Critical)
    );
}
#[test]
fn test_measure_fp_rate() {
    let detector = PivotDetector::new();
    let mut total_lines = 0;
    let mut total_alerts = 0;

    // Generate 1000 lines of normal activity noise
    for i in 0..1000 {
        let line = format!("2025-05-01 12:00:{:02}.000 info noise - Background message #{}", i % 60, i);
        let alerts = detector.analyze_line(&line);
        total_lines += 1;
        total_alerts += alerts.len();
    }

    let fp_rate = (total_alerts as f64 / total_lines as f64) * 100.0;
    println!("Measured False Positive Rate: {:.4}%", fp_rate);
    
    // Threshold: FP rate on pure noise should be very close to 0%
    assert!(fp_rate < 0.5, "False positive rate on noise is too high: {:.4}%", fp_rate);
}
