use std::fs::{OpenOptions, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use crate::errors::app_error::AppError;

pub struct WriteAheadLog {
    path: PathBuf,
}

impl WriteAheadLog {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, AppError> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        Ok(Self { path })
    }

    pub fn append(&self, lines: &[String]) -> Result<(), AppError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| AppError::IoError(e))?;

        for line in lines {
            writeln!(file, "{}", line.trim_end()).map_err(|e| AppError::IoError(e))?;
        }
        Ok(())
    }

    pub fn read_and_clear(&self) -> Result<Vec<String>, AppError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.path).map_err(|e| AppError::IoError(e))?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

        // Clear the WAL after reading
        std::fs::remove_file(&self.path).map_err(|e| AppError::IoError(e))?;

        Ok(lines)
    }

    pub fn has_content(&self) -> bool {
        self.path.exists() && std::fs::metadata(&self.path).map(|m| m.len()).unwrap_or(0) > 0
    }
}
