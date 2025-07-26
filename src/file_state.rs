use log::debug;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug)]
pub struct FileState {
    last_modified: SystemTime,
    last_processed: Instant,
}

#[derive(Debug)]
pub struct FileStateManager {
    states: HashMap<PathBuf, FileState>,
    debounce_time: Option<u64>,
}

impl Default for FileStateManager {
    fn default() -> Self {
        Self {
            states: HashMap::new(),
            debounce_time: None,
        }
    }
}

impl FileStateManager {
    pub fn new(debounce_time: Option<u64>) -> Self {
        Self {
            states: HashMap::new(),
            debounce_time,
        }
    }

    pub fn check_changed(&mut self, path: &PathBuf) -> anyhow::Result<bool> {
        let metadata = std::fs::metadata(path)?;
        let modified_time = metadata.modified()?;
        let now = Instant::now();

        let changed = match self.states.get(path) {
            Some(state) => {
                let content_changed = modified_time > state.last_modified;
                let debounce_passed = match self.debounce_time {
                    Some(ms) => {
                        now.duration_since(state.last_processed) > Duration::from_millis(ms)
                    }
                    None => true,
                };

                content_changed && debounce_passed
            }
            None => true,
        };

        if changed {
            debug!("File changed: {:?}", path);
            self.states.insert(
                path.clone(),
                FileState {
                    last_modified: modified_time,
                    last_processed: now,
                },
            );
        }

        Ok(changed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::thread::sleep;
    use tempfile::tempdir;

    #[test]
    fn test_first_file_detection() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let mut manager = FileStateManager::new(None);
        assert!(manager.check_changed(&file_path).unwrap());
    }

    #[test]
    fn test_unchanged_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let mut manager = FileStateManager::new(None);
        assert!(manager.check_changed(&file_path).unwrap());
        assert!(!manager.check_changed(&file_path).unwrap());
    }

    #[test]
    fn test_debounce() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();

        let mut manager = FileStateManager::new(Some(100));
        
        // First change is always detected
        assert!(manager.check_changed(&file_path).unwrap());
        
        // Update file content
        file.write_all(b"new content").unwrap();
        file.sync_all().unwrap();
        
        // Should not detect change before debounce time
        assert!(!manager.check_changed(&file_path).unwrap());
        
        // Wait for debounce time to pass
        sleep(Duration::from_millis(150));
        
        // Now update again
        file.write_all(b"newer content").unwrap();
        file.sync_all().unwrap();
        
        // Should detect change after debounce time
        assert!(manager.check_changed(&file_path).unwrap());
    }
}
