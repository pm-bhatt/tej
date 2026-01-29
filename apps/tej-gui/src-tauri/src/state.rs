use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tej_core::SpeedTestResult;

const MAX_HISTORY: usize = 100;

pub struct AppState {
    pub history: Mutex<Vec<SpeedTestResult>>,
    pub data_dir: PathBuf,
}

impl AppState {
    pub fn new(data_dir: PathBuf) -> Self {
        let history = Self::load_history(&data_dir).unwrap_or_default();
        Self {
            history: Mutex::new(history),
            data_dir,
        }
    }

    fn history_path(data_dir: &Path) -> PathBuf {
        data_dir.join("history.json")
    }

    fn load_history(data_dir: &Path) -> Option<Vec<SpeedTestResult>> {
        let path = Self::history_path(data_dir);
        let data = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&data).ok()
    }

    pub fn get_history(&self) -> Vec<SpeedTestResult> {
        self.history.lock().map(|h| h.clone()).unwrap_or_default()
    }

    pub fn save_result(&self, result: SpeedTestResult) {
        let mut history = match self.history.lock() {
            Ok(h) => h,
            Err(poisoned) => {
                eprintln!("History mutex poisoned, recovering");
                poisoned.into_inner()
            }
        };
        history.push(result);
        if history.len() > MAX_HISTORY {
            let excess = history.len() - MAX_HISTORY;
            history.drain(0..excess);
        }
        let path = Self::history_path(&self.data_dir);
        match serde_json::to_string_pretty(&*history) {
            Ok(json) => {
                if let Err(e) = std::fs::create_dir_all(&self.data_dir) {
                    eprintln!("Failed to create data dir: {e}");
                    return;
                }
                if let Err(e) = std::fs::write(&path, json) {
                    eprintln!("Failed to write history: {e}");
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize history: {e}");
            }
        }
    }
}
