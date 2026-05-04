use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct Metrics {
    pub alerts_total: AtomicU64,
    pub connections_monitored: AtomicU64,
    pub processes_scanned: AtomicU64,
    pub bytes_sent: AtomicU64,
    pub bytes_received: AtomicU64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            alerts_total: AtomicU64::new(0),
            connections_monitored: AtomicU64::new(0),
            processes_scanned: AtomicU64::new(0),
            bytes_sent: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
        }
    }
}

#[derive(Clone)]
pub struct MetricsManager {
    inner: Arc<Metrics>,
}

impl MetricsManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Metrics::new()),
        }
    }

    pub fn inc_alerts(&self) {
        self.inner.alerts_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_connections(&self) {
        self.inner.connections_monitored.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_processes(&self) {
        self.inner.processes_scanned.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_report(&self) -> String {
        format!(
            "Metrics Report:\n- Total Alerts: {}\n- Connections Monitored: {}\n- Processes Scanned: {}",
            self.inner.alerts_total.load(Ordering::Relaxed),
            self.inner.connections_monitored.load(Ordering::Relaxed),
            self.inner.processes_scanned.load(Ordering::Relaxed)
        )
    }
}
