use notify::{Watcher, RecursiveMode, Config, Event};
use std::path::Path;
use crate::errors::app_error::AppError;
use tokio::sync::mpsc;

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
}

impl FileWatcher {
    pub fn new(tx: mpsc::Sender<Event>) -> Result<Self, AppError> {
        let watcher = notify::RecommendedWatcher::new(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        }, Config::default()).map_err(|e| AppError::MonitorError(format!("Failed to create watcher: {}", e)))?;

        Ok(Self { watcher })
    }

    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AppError> {
        self.watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)
            .map_err(|e| AppError::MonitorError(format!("Failed to watch path: {}", e)))
    }
}
