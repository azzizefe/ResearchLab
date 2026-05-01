use anydesk_pivot_detector::parsers::{parse_system_conf, parse_trace_file};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_parse_system_conf_valid() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "ad.anynet.id=123456789").unwrap();
    writeln!(file, "ad.security.unattended_access=true").unwrap();
    writeln!(file, "ad.security.interactive_access=1").unwrap();
    
    let config = parse_system_conf(file.path()).unwrap();
    assert_eq!(config.anydesk_id, Some("123456789".to_string()));
    assert!(config.unattended_access);
    assert_eq!(config.interactive_access, "1");
}

#[test]
fn test_parse_system_conf_empty() {
    let file = NamedTempFile::new().unwrap();
    let config = parse_system_conf(file.path()).unwrap();
    assert_eq!(config.anydesk_id, None);
    assert!(!config.unattended_access);
}

#[test]
fn test_parse_trace_file_connections() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "2025-05-01 10:00:00.000  info anydesk - Incoming connection from 111 222 333").unwrap();
    writeln!(file, "2025-05-01 10:05:00.000  info anydesk - Connecting to 999 888 777").unwrap();
    
    let connections = parse_trace_file(file.path()).unwrap();
    assert_eq!(connections.len(), 2);
    assert_eq!(connections[0].remote_id, "111 222 333");
    assert_eq!(connections[1].remote_id, "999 888 777");
}

#[test]
fn test_parse_trace_file_invalid_date() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "INVALID_DATE  info anydesk - Incoming connection from 111 222 333").unwrap();
    
    let connections = parse_trace_file(file.path()).unwrap();
    assert_eq!(connections.len(), 0);
}
