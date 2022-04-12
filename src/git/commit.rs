//! Git commits

use std::{fmt::Display, process::Command};

use chrono::{DateTime, Utc};

use crate::error::{Error, Result};

/// A git commit
#[derive(Debug, Clone)]
pub struct Commit {
    /// Commit hash
    pub hash: String,
    /// Commit timestamp
    pub timestamp: DateTime<Utc>,
    /// Author
    pub author: String,
    /// Subject
    pub subject: String,
}

impl Default for Commit {
    fn default() -> Self {
        Self {
            hash: Default::default(),
            timestamp: Utc::now(),
            author: Default::default(),
            subject: Default::default(),
        }
    }
}

impl Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "hash:{}", self.hash)?;
        writeln!(f, "ts:{}", self.timestamp.to_rfc3339())?;
        writeln!(f, "author:{}", self.author)?;
        writeln!(f, "subject:{}", self.subject)?;
        Ok(())
    }
}

/// Wrapper for `git commit`
pub fn git_commit(msg: &str) -> Result<(String, String)> {
    let mut cmd = Command::new("git");
    cmd.args(["commit", "-m", &msg]);
    let output = cmd.output().expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid stdout");
    let stderr = String::from_utf8(output.stderr).expect("Invalid stderr");

    if !output.status.success() {
        return Err(Error::InternalError(format!(
            "Failed to execute git commit: {stderr}"
        )));
    }

    Ok((stdout, stderr))
}
