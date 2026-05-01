use anydesk_pivot_detector::analyzers::PivotDetector;
use anydesk_pivot_detector::reporters::JsonReporter;
use tempfile::tempdir;

#[test]
fn test_detector_to_json_reporter_chain() {
    let dir = tempdir().unwrap();
    let report_dir = dir.path().to_str().unwrap().to_string();
    let reporter = JsonReporter::new(report_dir.clone());
    let detector = PivotDetector::new();
    
    // 1. Simulate a pivot line
    let line = "2025-05-01 10:00:00.000  info anydesk - Incoming connection from 666 666 666";
    let alerts = detector.analyze_line(line);
    assert!(!alerts.is_empty());
    
    // 2. Report alerts
    let report_path = reporter.report(&alerts).expect("Failed to write report");
    
    // 3. Verify report exists and contains alert
    let report_content = std::fs::read_to_string(report_path).unwrap();
    assert!(report_content.contains("New Incoming Connection"));
    assert!(report_content.contains("666 666 666"));
}
