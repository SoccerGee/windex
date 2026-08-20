pub mod settings;

pub use settings::Config;

use anyhow::Result;
use log::{info, warn};
use std::fs;
use std::path::PathBuf;

/// Get the config file path
pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("windex")
        .join("config.toml")
}

/// Load configuration from file, or create default if it doesn't exist
pub fn load_config() -> Result<Config> {
    let path = config_path();

    if path.exists() {
        info!("Loading config from {:?}", path);
        let content = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    } else {
        info!("Config file not found, using defaults");

        // Create default config and optionally save it
        let config = Config::default();

        // Try to create the config file with defaults
        if let Some(parent) = path.parent() {
            if fs::create_dir_all(parent).is_ok() {
                match toml::to_string_pretty(&config) {
                    Ok(content) => {
                        if fs::write(&path, content).is_ok() {
                            info!("Created default config at {:?}", path);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to serialize default config: {}", e);
                    }
                }
            }
        }

        Ok(config)
    }
}
