use crate::errors::app_error::AppError;
use notify::{Config, Event, EventKind, RecursiveMode, Watcher};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    last_positions: std::collections::HashMap<PathBuf, u64>,
}

impl FileWatcher {
    pub fn new(tx: mpsc::Sender<Event>) -> Result<Self, AppError> {
        let watcher = notify::RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                if let Ok(event) = res {
                    let _ = tx.blocking_send(event);
                }
            },
            Config::default(),
        )
        .map_err(|e| AppError::MonitorError(format!("Failed to create watcher: {e}")))?;

        let mut last_positions = std::collections::HashMap::new();
        if let Ok(data) = std::fs::read_to_string("reports/checkpoint.json") {
            if let Ok(pos) = serde_json::from_str(&data) {
                last_positions = pos;
            }
        }

        Ok(Self {
            watcher,
            last_positions,
        })
    }

    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AppError> {
        let p = path.as_ref().to_path_buf();
        // If not in checkpoint, initialize position to end of file
        if !self.last_positions.contains_key(&p) {
            if let Ok(metadata) = std::fs::metadata(&p) {
                self.last_positions.insert(p.clone(), metadata.len());
            }
        }

        self.watcher
            .watch(&p, RecursiveMode::NonRecursive)
            .map_err(|e| AppError::MonitorError(format!("Failed to watch path: {e}")))
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
                                self.save_checkpoints();
                            } else if new_len < last_pos {
                                // File truncated (potential rotation)
                                self.last_positions.insert(path, 0); // Start from beginning
                                self.save_checkpoints();
                            }
                        }
                    }
                }
            }
        }
        new_lines
    }

    fn save_checkpoints(&self) {
        if let Ok(data) = serde_json::to_string(&self.last_positions) {
            let _ = std::fs::create_dir_all("reports");
            let _ = std::fs::write("reports/checkpoint.json", data);
        }
    }
}
