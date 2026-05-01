use crate::errors::app_error::AppError;
use crate::models::alert::{Alert, AlertSeverity};
use chrono::Utc;
use std::io::Write;
use std::net::{TcpStream, UdpSocket};

pub enum SyslogProtocol {
    Udp,
    Tcp,
}

pub struct SyslogReporter {
    server: String,
    protocol: SyslogProtocol,
}

impl SyslogReporter {
    #[must_use] 
    pub fn new(server: String, protocol: SyslogProtocol) -> Self {
        Self { server, protocol }
    }

    /// 9.3.1: Syslog formatinda (RFC 5424) uyari gonderme
    /// Format: <PRI>VERSION TIMESTAMP HOSTNAME APP-NAME PROCID MSGID STRUCTURED-DATA MSG
    fn format_rfc5424(&self, alert: &Alert) -> String {
        let pri = match alert.severity {
            AlertSeverity::Low => 14,      // info
            AlertSeverity::Medium => 12,   // warning
            AlertSeverity::High => 11,     // error
            AlertSeverity::Critical => 10, // alert
        };

        format!(
            "<{}>1 {} {} anydesk-pivot-detector - - - {}",
            pri,
            Utc::now().to_rfc3339(),
            "localhost",
            alert.description
        )
    }

    /// 9.3.3: SIEM entegrasyonu icin CEF format destegi
    /// Format: CEF:Version|Device Vendor|Device Product|Device Version|Device Event Class ID|Name|Severity|[Extension]
    #[must_use] 
    pub fn format_cef(&self, alert: &Alert) -> String {
        let severity_val = match alert.severity {
            AlertSeverity::Low => 3,
            AlertSeverity::Medium => 5,
            AlertSeverity::High => 8,
            AlertSeverity::Critical => 10,
        };

        format!(
            "CEF:0|ResearchLab|AnyDeskPivotDetector|0.1.0|PIVOT_EVENT|{}|{}|msg={} src_module={}",
            alert.title, severity_val, alert.description, alert.source_module
        )
    }

    /// 9.3.2: UDP/TCP syslog destegi
    pub fn send(&self, alert: &Alert, use_cef: bool) -> Result<(), AppError> {
        let message = if use_cef {
            self.format_cef(alert)
        } else {
            self.format_rfc5424(alert)
        };

        let message_with_newline = format!("{message}\n");

        match self.protocol {
            SyslogProtocol::Udp => {
                let socket = UdpSocket::bind("0.0.0.0:0").map_err(AppError::IoError)?;
                socket
                    .send_to(message_with_newline.as_bytes(), &self.server)
                    .map_err(AppError::IoError)?;
            }
            SyslogProtocol::Tcp => {
                let mut stream = TcpStream::connect(&self.server).map_err(AppError::IoError)?;
                stream
                    .write_all(message_with_newline.as_bytes())
                    .map_err(AppError::IoError)?;
            }
        }
        Ok(())
    }
}
