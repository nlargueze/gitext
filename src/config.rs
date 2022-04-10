//! Configuration

use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Configuration file name
pub const CONFIG_FILE: &str = "gitt.toml";

/// Configuration object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Configuration {}

impl Configuration {
    /// Parses a file and returns a configuration object
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let cfg_str = fs::read_to_string(path)?;
        Ok(toml::from_str::<Configuration>(&cfg_str)?)
    }

    /// Saves a [Configuration] to file
    pub fn to_file(&self, path: &PathBuf) -> Result<()> {
        let cfg_str = toml::to_string(self)?;
        fs::write(path, cfg_str)?;
        Ok(())
    }

    /// Checks if a directory is already initialized
    pub fn is_initialized(path: &PathBuf) -> bool {
        path.join(CONFIG_FILE).exists()
    }
}
