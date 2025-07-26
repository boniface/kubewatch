use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn test_file_watcher_integration() {
    // Create a temporary directory for our test
    let dir = tempdir().unwrap();

    // Create a config file
    let config_content = format!(
        r#"
watch_dir: "{}"
command: "echo"
file_extensions:
  - yaml
  - yml
file_prefixes:
  - dev-
  - prod-
debounce_time: 100
"#,
        dir.path().to_str().unwrap().replace('\\', "/")
    );

    fs::write("config.yaml", config_content).unwrap();

    // Use a more reliable path to the binary
    let binary_path = std::env::current_dir()
        .unwrap()
        .join("target/debug/kubewatch");

    // Start the application in a separate process
    let mut child = Command::new(binary_path)
        .env("RUST_LOG", "info")
        .spawn()
        .expect("Failed to start application");

    // Give it time to start
    thread::sleep(Duration::from_secs(1));

    // Create a file that should be detected
    let test_file = dir.path().join("dev-test.yaml");
    let mut file = File::create(&test_file).unwrap();
    file.write_all(b"test: data").unwrap();
    file.sync_all().unwrap();

    // Wait for the file to be processed
    thread::sleep(Duration::from_secs(1));

    // Clean up
    child.kill().expect("Failed to kill process");

    // Test assertions would typically check logs or some other output
    // For a real integration test, you might check if the command was executed
}
