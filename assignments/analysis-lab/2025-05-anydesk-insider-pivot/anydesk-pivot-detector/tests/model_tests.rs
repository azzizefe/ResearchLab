use anydesk_pivot_detector::models::alert::{Alert, AlertSeverity};
use chrono::Utc;

#[test]
fn test_alert_serialization() {
    let alert = Alert {
        id: "test-uuid".to_string(),
        timestamp: Utc::now(),
        severity: AlertSeverity::High,
        title: "Unauthorized Pivot".to_string(),
        description: "Test".to_string(),
        source_module: "Detector".to_string(),
        evidence: serde_json::json!({"ip": "1.1.1.1"}),
    };

    let serialized = serde_json::to_string(&alert).unwrap();
    let deserialized: Alert = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.id, alert.id);
    assert_eq!(deserialized.title, alert.title);
}
