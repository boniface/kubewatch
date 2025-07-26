use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub watch_dir: PathBuf,
    pub command: String,
    pub file_extensions: Vec<String>,
    pub file_prefixes: Vec<String>,
    pub debounce_time: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            watch_dir: PathBuf::from("/tmp/"),
            command: "kubectl apply -f".to_string(),
            file_extensions: vec!["yaml".to_string(), "yml".to_string()],
            file_prefixes: vec![
                "dev-".to_string(),
                "prod-".to_string(),
                "staging-".to_string(),
            ],
            debounce_time: Some(200),
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config: Config = std::fs::read_to_string("config.yaml")
            .ok()
            .and_then(|content| serde_yaml::from_str(&content).ok())
            .unwrap_or_default();

        if config.command.contains("kubectl") && !Config::kubectl_exists() {
            println!("kubectl not found. Please install it with the following instructions:");
            println!("{}", Config::get_kubectl_install_instructions());
        }

        Ok(config)
    }

    pub fn kubectl_exists() -> bool {
        Command::new("which")
            .arg("kubectl")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn get_kubectl_install_instructions() -> String {
        "To install kubectl on Ubuntu, run the following commands:\n\
            \n\
            1. Update the apt package index and install required packages:\n\
               sudo apt-get update\n\
               sudo apt-get install -y apt-transport-https ca-certificates curl\n\
            \n\
            2. Download the Google Cloud public signing key:\n\
               curl -fsSL https://packages.cloud.google.com/apt/doc/apt-key.gpg | sudo gpg --dearmor -o /usr/share/keyrings/kubernetes-archive-keyring.gpg\n\
            \n\
            3. Add the Kubernetes apt repository:\n\
               echo \"deb [signed-by=/usr/share/keyrings/kubernetes-archive-keyring.gpg] https://apt.kubernetes.io/ kubernetes-xenial main\" | sudo tee /etc/apt/sources.list.d/kubernetes.list\n\
            \n\
            4. Update apt package index and install kubectl:\n\
               sudo apt-get update\n\
               sudo apt-get install -y kubectl\n\
            \n\
            5. Verify installation:\n\
               kubectl version --client".to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.watch_dir, PathBuf::from("/tmp/"));
        assert_eq!(config.command, "kubectl apply -f");
        assert_eq!(config.file_extensions, vec!["yaml", "yml"]);
        assert_eq!(config.file_prefixes, vec!["dev-", "prod-", "staging-"]);
        assert_eq!(config.debounce_time, Some(200));
    }

    #[test]
    fn test_load_from_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.yaml");
        let test_config = r#"
        watch_dir: "/test/path"
        command: "echo"
        file_extensions:
          - json
        file_prefixes:
          - test-
        debounce_time: 100
        "#;
        fs::write(&config_path, test_config).unwrap();

        // Temporarily override the current directory to our test directory
        let old_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        let config = Config::load().unwrap();

        assert_eq!(config.watch_dir, PathBuf::from("/test/path"));
        assert_eq!(config.command, "echo");
        assert_eq!(config.file_extensions, vec!["json"]);
        assert_eq!(config.file_prefixes, vec!["test-"]);
        assert_eq!(config.debounce_time, Some(100));

        // Restore original directory
        std::env::set_current_dir(old_dir).unwrap();
    }
}