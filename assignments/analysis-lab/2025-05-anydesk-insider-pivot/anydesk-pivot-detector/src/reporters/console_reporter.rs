use crate::models::alert::{Alert, AlertSeverity};
use colored::Colorize;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct AlertRow {
    #[tabled(rename = "Severity")]
    severity: String,
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "Source")]
    source: String,
}

pub struct ConsoleReporter;

impl ConsoleReporter {
    pub fn report(&self, alerts: &[Alert]) {
        if alerts.is_empty() {
            println!("No alerts to report.");
            return;
        }

        let rows: Vec<AlertRow> = alerts
            .iter()
            .map(|a| {
                let severity_str = match a.severity {
                    AlertSeverity::Low => "LOW".blue().to_string(),
                    AlertSeverity::Medium => "MEDIUM".yellow().to_string(),
                    AlertSeverity::High => "HIGH".red().to_string(),
                    AlertSeverity::Critical => "CRITICAL".red().bold().blink().to_string(),
                };
                AlertRow {
                    severity: severity_str,
                    title: a.title.clone(),
                    source: a.source_module.clone(),
                }
            })
            .collect();

        println!(
            "\n{}",
            "--- ANYDESK PIVOT DETECTION REPORT ---".green().bold()
        );
        println!("{}", Table::new(rows));
    }
}
