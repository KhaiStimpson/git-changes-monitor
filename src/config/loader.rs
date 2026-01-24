use std::fs;
use std::path::PathBuf;

use color_eyre::eyre::{eyre, Result};

use super::types::Config;

/// Configuration file loader
pub struct ConfigLoader {
    /// Path to the configuration file
    config_path: PathBuf,
}

impl ConfigLoader {
    /// Create a new config loader
    /// If custom_path is provided, use it; otherwise use the default location
    pub fn new(custom_path: Option<PathBuf>) -> Result<Self> {
        let config_path = if let Some(path) = custom_path {
            path
        } else {
            Self::default_config_path()?
        };

        Ok(Self { config_path })
    }

    /// Get the default configuration file path
    fn default_config_path() -> Result<PathBuf> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| eyre!("Could not determine config directory"))?;

        Ok(config_dir.join("git-file-monitor").join("gfm.json"))
    }

    /// Load the configuration
    pub fn load(&self) -> Result<Config> {
        if self.config_path.exists() {
            let content = fs::read_to_string(&self.config_path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config and save it
            let config = Config::default();
            self.save(&config)?;
            Ok(config)
        }
    }

    /// Save the configuration
    pub fn save(&self, config: &Config) -> Result<()> {
        // Create parent directories if they don't exist
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_path, content)?;

        Ok(())
    }

    /// Get the configuration file path
    #[allow(dead_code)]
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
}
