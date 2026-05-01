use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("anydesk-pivot-detector").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("AnyDesk logs for historical pivot activity"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("anydesk-pivot-detector").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_cli_scan_missing_file() {
    let mut cmd = Command::cargo_bin("anydesk-pivot-detector").unwrap();
    cmd.arg("scan")
        .arg("--path")
        .arg("non_existent_file.trace")
        .assert()
        .failure();
}

#[test]
fn test_cli_config_check() {
    let mut cmd = Command::cargo_bin("anydesk-pivot-detector").unwrap();
    cmd.arg("config-check")
        .assert()
        .success();
}
