//! Configuration

use std::{collections::BTreeMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Configuration directory
pub const CONFIG_DIR: &str = ".gitt";

/// Configuration file name
pub const CONFIG_FILE: &str = "config.toml";

/// CommitTypeConfig
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommitTypeConfig {
    /// Description
    pub desc: String,
    /// Title
    pub title: String,
}

/// Commits config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitsConfig {
    /// Commit types
    pub types: BTreeMap<String, CommitTypeConfig>,
}

impl Default for CommitsConfig {
    fn default() -> Self {
        let mut types = BTreeMap::new();
        types.insert(
            "feat".to_string(),
            CommitTypeConfig {
                desc: "A new feature".to_string(),
                title: "New features".to_string(),
            },
        );
        types.insert(
            "fix".to_string(),
            CommitTypeConfig {
                desc: "A bug fix".to_string(),
                title: "Bug fixes".to_string(),
            },
        );
        types.insert(
            "docs".to_string(),
            CommitTypeConfig {
                desc: "Documentation".to_string(),
                title: "Documentation".to_string(),
            },
        );
        types.insert(
            "style".to_string(),
            CommitTypeConfig {
                desc: "Code styling".to_string(),
                title: "Code styling".to_string(),
            },
        );
        types.insert(
            "refactor".to_string(),
            CommitTypeConfig {
                desc: "Code refactoring".to_string(),
                title: "Code refactoring".to_string(),
            },
        );
        types.insert(
            "perf".to_string(),
            CommitTypeConfig {
                desc: "Performance Improvements".to_string(),
                title: "Performance Improvements".to_string(),
            },
        );
        types.insert(
            "test".to_string(),
            CommitTypeConfig {
                desc: "Tests".to_string(),
                title: "Tests".to_string(),
            },
        );
        types.insert(
            "build".to_string(),
            CommitTypeConfig {
                desc: "Build system".to_string(),
                title: "Build system".to_string(),
            },
        );
        types.insert(
            "ci".to_string(),
            CommitTypeConfig {
                desc: "Continuous Integration".to_string(),
                title: "Continuous Integration".to_string(),
            },
        );
        types.insert(
            "cd".to_string(),
            CommitTypeConfig {
                desc: "Continuous Delivery".to_string(),
                title: "Continuous Delivery".to_string(),
            },
        );
        types.insert(
            "chore".to_string(),
            CommitTypeConfig {
                desc: "Other changes".to_string(),
                title: "Other changes".to_string(),
            },
        );

        Self { types }
    }
}

/// Changelog config
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChangeLogConfig {}

/// Configuration object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Commits config
    pub commits: CommitsConfig,
    /// Change log config
    pub changelog: ChangeLogConfig,
}

impl Config {
    /// Loads the configuration file from the current directory
    pub fn load(repo_path: &PathBuf) -> Result<Self> {
        let file = repo_path.join(CONFIG_DIR).join(CONFIG_FILE);
        Self::from_file(&file)
    }

    /// Saves a [Configuration] to file
    pub fn save(&self, repo_path: &PathBuf) -> Result<()> {
        let cfg_str = toml::to_string(self)?;
        if !repo_path.join(CONFIG_DIR).exists() {
            fs::create_dir(repo_path.join(CONFIG_DIR))?;
        }
        fs::write(repo_path.join(CONFIG_DIR).join(CONFIG_FILE), cfg_str)?;
        Ok(())
    }

    /// Checks if a repo is already initialized
    pub fn is_initialized(repo_path: &PathBuf) -> bool {
        repo_path.join(CONFIG_DIR).join(CONFIG_FILE).exists()
    }

    /// Parses a file and returns a configuration object
    pub fn from_file(repo_path: &PathBuf) -> Result<Self> {
        let cfg_str = fs::read_to_string(repo_path)?;
        Ok(toml::from_str::<Config>(&cfg_str)?)
    }
}
