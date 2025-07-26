mod command;
mod config;
mod file_state;
mod watcher;

use anyhow::{Context, Result};
use log::{error, info, LevelFilter};

fn setup_logging() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();
}

fn main() -> Result<()> {
    setup_logging();
    
    let config = config::Config::load().context("Failed to load configuration")?;

    // Show configuration summary
    info!("Starting file watcher with configuration:");
    info!("  Watch directory: {:?}", config.watch_dir);
    info!("  Command: {}", config.command);
    info!("  Extensions: {:?}", config.file_extensions);
    info!("  File prefixes: {:?}", config.file_prefixes);
    info!("  Debounce time: {:?}ms", config.debounce_time);

   
    let mut watcher = watcher::FileWatcher::new(config);

 match watcher.watch() {
     Ok(_) => info!("Watcher exited normally"),
     Err(e) => {
         error!("Watcher error: {}", e);
         error!("Please update the 'watch_dir' in config.yml to a valid directory and restart the application.");
     }
 }

    Ok(())
}
