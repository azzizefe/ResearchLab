use notify::{Watcher, RecursiveMode, Config, Event, EventKind};
use std::path::{Path, PathBuf};
use crate::errors::app_error::AppError;
use tokio::sync::mpsc;
use std::fs::File;
use std::io::{Seek, SeekFrom, BufRead, BufReader};

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    last_positions: std::collections::HashMap<PathBuf, u64>,
}

impl FileWatcher {
    pub fn new(tx: mpsc::Sender<Event>) -> Result<Self, AppError> {
        let watcher = notify::RecommendedWatcher::new(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        }, Config::default()).map_err(|e| AppError::MonitorError(format!("Failed to create watcher: {}", e)))?;

        Ok(Self { 
            watcher,
            last_positions: std::collections::HashMap::new(),
        })
    }

    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AppError> {
        let p = path.as_ref().to_path_buf();
        // Initialize position to end of file
        if let Ok(metadata) = std::fs::metadata(&p) {
            self.last_positions.insert(p.clone(), metadata.len());
        }
        
        self.watcher.watch(&p, RecursiveMode::NonRecursive)
            .map_err(|e| AppError::MonitorError(format!("Failed to watch path: {}", e)))
    }

    pub fn handle_event(&mut self, event: Event) -> Vec<String> {
        let mut new_lines = Vec::new();
        if let EventKind::Modify(_) = event.kind {
            for path in event.paths {
                if let Some(&last_pos) = self.last_positions.get(&path) {
                    if let Ok(mut file) = File::open(&path) {
                        if let Ok(metadata) = file.metadata() {
                            let new_len = metadata.len();
                            if new_len > last_pos {
                                let _ = file.seek(SeekFrom::Start(last_pos));
                                let mut reader = BufReader::new(file);
                                let mut line = String::new();
                                while reader.read_line(&mut line).unwrap_or(0) > 0 {
                                    new_lines.push(line.clone());
                                    line.clear();
                                }
                                self.last_positions.insert(path, new_len);
                            } else if new_len < last_pos {
                                // File truncated
                                self.last_positions.insert(path, new_len);
                            }
                        }
                    }
                }
            }
        }
        new_lines
    }
}
