use crate::config::Config;
use anyhow::Result;
use log::{error, info};
use std::path::PathBuf;
use std::process::Command;

pub struct CommandRunner<'a> {
    config: &'a Config,
}

impl<'a> CommandRunner<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn execute_for_files(&self, files: &[PathBuf]) -> Result<()> {
        if self.config.command.contains("kubectl") && !Config::kubectl_exists() {
            error!("kubectl command not found. Please install kubectl first.");
            return Err(anyhow::anyhow!(
                "kubectl command not found. Please install kubectl first."
            ));
        }

        if files.is_empty() {
            info!("No files to process");
            return Ok(());
        }

        let mut parts = self.config.command.split_whitespace();
        let command = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Command is empty"))?;
        let args: Vec<&str> = parts.collect();

        info!("Executing command: {} with files: {:?}", command, files);

        let mut cmd = Command::new(command);
        cmd.args(args);


        for file in files {
            if let Some(file_str) = file.to_str() {
                cmd.arg(file_str);
            }
        }

        let output = cmd.output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("Command failed: {}", error_msg);
            return Err(anyhow::anyhow!("Command failed: {}", error_msg));
        } else {
            info!(
                "Command output: {}",
                String::from_utf8_lossy(&output.stdout)
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_command_execution() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let config = Config {
            watch_dir: PathBuf::from("/tmp"),
            command: "echo".to_string(),
            file_extensions: vec!["txt".to_string()],
            file_prefixes: vec![],
            debounce_time: None,
        };

        let runner = CommandRunner::new(&config);
        let result = runner.execute_for_files(&[file_path]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_command_error() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let config = Config {
            watch_dir: PathBuf::from("/tmp"),
            command: "nonexistent_command".to_string(),
            file_extensions: vec!["txt".to_string()],
            file_prefixes: vec![],
            debounce_time: None,
        };

        let runner = CommandRunner::new(&config);
        let result = runner.execute_for_files(&[file_path]);
        assert!(result.is_err());
    }
}
