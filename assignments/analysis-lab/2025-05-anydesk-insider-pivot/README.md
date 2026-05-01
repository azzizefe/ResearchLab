# AnyDesk Insider Pivot Detection System 🛡️

![Rust](https://img.shields.io/badge/rust-1.78+-orange.svg)
![Tauri](https://img.shields.io/badge/tauri-v2-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![CI](https://img.shields.io/badge/CI-GitHub%20Actions-brightgreen.svg)

A sophisticated detection and monitoring system designed to identify and mitigate **AnyDesk Insider Pivot** attacks. This tool provides real-time visibility into unauthorized remote access, lateral movement, and data exfiltration attempts leveraging AnyDesk's proprietary protocol.

---

## 🚀 Overview

In modern enterprise environments, legitimate remote access tools like AnyDesk are frequently exploited by malicious insiders or compromised accounts to bypass perimeter security. This system provides a robust, Rust-powered detection engine that monitors AnyDesk's behavior, parses its forensic artifacts, and assigns risk scores to suspicious activities.

### Key Capabilities
*   **🔍 Advanced Log Parsing**: Deep analysis of `ad.trace`, `system.conf`, and `service.conf` to extract connection IDs, session durations, and security configurations.
*   **⏱️ Real-Time Monitoring**: Live tracking of AnyDesk processes, file system changes, and network activity.
*   **🧠 Threat Analysis Engine**: A rule-based engine paired with an anomaly scorer to detect mesai hours bypass, unauthorized ACL changes, and suspicious process spawning (e.g., `anydesk.exe` -> `cmd.exe`).
*   **🖥️ Desktop GUI (Tauri)**: A premium dashboard built with React and Tailwind CSS for visual monitoring and configuration management.
*   **📊 Multi-Format Reporting**: Exportable reports in JSON, Syslog (CEF/LEEF), and formatted console tables.
*   **🐳 Docker Ready**: Containerized deployment support for scalable monitoring environments.

---

## 🛠️ Technology Stack

| Component | Technology |
| :--- | :--- |
| **Core Engine** | Rust (Tokio, Serde, Notify) |
| **GUI Framework** | Tauri v2 |
| **Frontend** | Vite, React 19, TypeScript, Tailwind CSS 4 |
| **State Management** | Zustand & TanStack Query |
| **Testing** | Nextest, Tarpaulin, Criterion |
| **Deployment** | Docker & Docker Compose |

---

## 📂 Project Structure

```text
anydesk-pivot-detector/
├── src/                    # Rust Backend
│   ├── api/                # Tauri IPC Commands
│   ├── parsers/            # Log & Config Parsers
│   ├── monitors/           # Process, File & Network Monitors
│   ├── analyzers/          # Rule Engine & Anomaly Scoring
│   └── reporters/          # JSON, Console & Syslog Output
├── src-tauri/              # Tauri Configuration
├── frontend/               # React Dashboard (Vite + TS)
├── config/                 # Default configurations
├── tests/                  # Unit, Integration & E2E Tests
└── docker-compose.yml      # Container Orchestration
```

---

## 🚦 Getting Started

### Prerequisites
*   **Rust**: 1.78 or higher (`rustup update stable`)
*   **Node.js**: 18 LTS or higher
*   **Tauri CLI**: `cargo install tauri-cli`
*   **AnyDesk**: Installed or portable version for log generation

### Installation
1.  Clone the repository:
    ```bash
    git clone https://github.com/keyvanarasteh/ResearchLab.git
    cd ResearchLab/assignments/analysis-lab/2025-05-anydesk-insider-pivot
    ```
2.  Install frontend dependencies:
    ```bash
    cd frontend && pnpm install && cd ..
    ```
3.  Configure the environment:
    ```bash
    cp .env.example .env
    # Edit .env with your specific paths
    ```

### Running the Application
*   **Development Mode (GUI)**:
    ```bash
    cargo tauri dev
    ```
*   **CLI Monitor Mode**:
    ```bash
    cargo run -- monitor
    ```
*   **One-Time Scan**:
    ```bash
    cargo run -- scan --path "C:/Path/To/Logs"
    ```

---

## 🛡️ Detection Scenarios
This system is pre-configured to detect the following high-risk scenarios:
1.  **Insider Pivot**: Unauthorized connection from a non-whitelisted ID.
2.  **Shadow IT**: Execution of portable AnyDesk versions in unauthorized directories.
3.  **Persistence**: Unattended access enabled with weak or static passwords.
4.  **Lateral Movement**: Suspicious child processes (e.g., `net.exe`, `nmap.exe`) spawned via AnyDesk.
5.  **Exfiltration**: High-volume data transfers detected during remote sessions.

---

## 🧪 Testing & Quality
We maintain high standards for code quality and reliability:
*   **Unit Tests**: `cargo test`
*   **Lints**: `cargo clippy -- -D warnings`
*   **Coverage**: `cargo tarpaulin --out Html`
*   **Audit**: `cargo audit`

---

## 📝 License & Author
*   **Student ID**: `2420191044`
*   **Project UUID**: `1253dbcd-b308-4443-a29e-036bbe0b27c7`
*   **License**: MIT

---
*Created as part of the ResearchLab Security Analysis Assignments.*
