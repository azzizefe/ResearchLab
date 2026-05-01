# AnyDesk Pivot Detector - System Architecture

## Overview
The AnyDesk Pivot Detector is designed as a multi-layered security analysis tool. It combines static log analysis with real-time system monitoring to identify suspicious lateral movement (pivot) activity via AnyDesk.

## Modules

### 1. Parsers (`src/parsers/`)
- **ad_trace**: Analyzes AnyDesk `ad.trace` logs for incoming/outgoing connection events.
- **system_conf**: Extracts AnyDesk ID and security settings (Unattended Access, etc.) from `system.conf`.
- **service_conf**: Analyzes service-level feature configurations.

### 2. Monitors (`src/monitors/`)
- **ProcessMonitor**: Real-time tracking of AnyDesk and suspicious child processes (cmd.exe, powershell.exe).
- **FileWatcher**: Monitors AnyDesk log files for real-time appends.
- **NetworkMonitor**: Tracks active TCP connections and DNS queries related to AnyDesk infrastructure.

### 3. Analyzers (`src/analyzers/`)
- **PivotDetector**: Rule-based engine for identifying unauthorized connection patterns.
- **RuleEngine**: Enforces security policies like working hours, ACLs, and path validations.
- **AnomalyScorer**: Calculates a weighted risk score based on aggregated alerts.

### 4. Reporters (`src/reporters/`)
- **Console**: Human-readable terminal output.
- **JSON**: Machine-readable reports for SIEM integration.
- **Syslog**: RFC 5424 compliant remote logging.

### 5. Frontend (`frontend/`)
- **Tauri GUI**: React-based dashboard for real-time visualization and management.

## Data Flow
1. **Event Capture**: Monitors/Parsers gather raw telemetry.
2. **Analysis**: RuleEngine & PivotDetector flag suspicious activity.
3. **Scoring**: AnomalyScorer calculates risk level.
4. **Reporting**: Alerts are dispatched to configured reporters and UI.
