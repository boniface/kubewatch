use super::{command::CommandRunner, config::Config, file_state::FileStateManager};
use anyhow::Result;
use log::{error, info};
use notify::{Event, Watcher};
use std::sync::mpsc::Receiver;

pub struct FileWatcher {
    config: Config,
    state_manager: FileStateManager,
}

impl FileWatcher {
    pub fn new(config: Config) -> Self {
        Self {
            state_manager: FileStateManager::new(config.debounce_time),
            config,
        }
    }

    pub fn watch(&mut self) -> Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                tx.send(event).unwrap_or_default();
            }
        })?;

        watcher.watch(&self.config.watch_dir, notify::RecursiveMode::Recursive)?;
        info!(
            "Watching directory: {:?}",
            self.config.watch_dir.canonicalize()?
        );

        self.process_events(rx)
    }

    fn process_events(&mut self, rx: Receiver<Event>) -> Result<()> {
        let mut changed_files = Vec::new();
        let command_runner = CommandRunner::new(&self.config);

        for event in rx {
            if let Some(paths) = self.filter_relevant_changes(&event) {
                changed_files.clear();

                for path in paths {
                    match self.state_manager.check_changed(&path) {
                        Ok(true) => changed_files.push(path),
                        Ok(false) => {} // File hasn't changed enough to process
                        Err(e) => {
                            error!("Error checking file state for {:?}: {}", path, e);
                            // Continue processing other files
                        }
                    }
                }

                if !changed_files.is_empty() {
                    match command_runner.execute_for_files(&changed_files) {
                        Ok(_) => info!(
                            "Successfully executed command for {} files",
                            changed_files.len()
                        ),
                        Err(e) => {
                            error!("Command execution failed: {}", e);
                            // Continue with next events, don't propagate the error
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn filter_relevant_changes(&self, event: &notify::Event) -> Option<Vec<std::path::PathBuf>> {
        if !event.kind.is_modify() && !event.kind.is_create() {
            return None;
        }

        let paths: Vec<_> = event
            .paths
            .iter()
            .filter(|path| {
                // Check if extension matches
                let extension_matches = path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| self.config.file_extensions.iter().any(|e| e == ext))
                    .unwrap_or(false);

                // Check if filename has a valid prefix
                let prefix_matches = if self.config.file_prefixes.is_empty() {
                    // If no prefixes specified, accept all files (backward compatibility)
                    true
                } else {
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .map(|name| {
                            self.config
                                .file_prefixes
                                .iter()
                                .any(|prefix| name.starts_with(prefix))
                        })
                        .unwrap_or(false)
                };

                extension_matches && prefix_matches
            })
            .cloned()
            .collect();

        if paths.is_empty() { None } else { Some(paths) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_filter_relevant_changes() {
        let dir = tempdir().unwrap();
        
        // Create files with different extensions and prefixes
        let yaml_file = dir.path().join("dev-test.yaml");
        let yml_file = dir.path().join("prod-test.yml");
        let non_matching_ext = dir.path().join("dev-test.txt");
        let non_matching_prefix = dir.path().join("test.yaml");
        
        File::create(&yaml_file).unwrap();
        File::create(&yml_file).unwrap();
        File::create(&non_matching_ext).unwrap();
        File::create(&non_matching_prefix).unwrap();

        let config = Config {
            watch_dir: dir.path().to_path_buf(),
            command: "echo".to_string(),
            file_extensions: vec!["yaml".to_string(), "yml".to_string()],
            file_prefixes: vec!["dev-".to_string(), "prod-".to_string()],
            debounce_time: None,
        };
        
        let watcher = FileWatcher::new(config);
        
        // Test with a modify event
        let event = Event {
            kind: notify::EventKind::Modify(notify::event::ModifyKind::Data(
                notify::event::DataChange::Content,
            )),
            paths: vec![
                yaml_file.clone(),
                yml_file.clone(),
                non_matching_ext.clone(),
                non_matching_prefix.clone(),
            ],
            attrs: notify::event::EventAttributes::new(),
        };
        
        let filtered = watcher.filter_relevant_changes(&event);
        assert!(filtered.is_some());
        
        let paths = filtered.unwrap();
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&yaml_file));
        assert!(paths.contains(&yml_file));
        assert!(!paths.contains(&non_matching_ext));
        assert!(!paths.contains(&non_matching_prefix));
    }
}
