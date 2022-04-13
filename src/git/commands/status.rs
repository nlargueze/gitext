//! Wrapper for `git status` command.

use std::process::Command;

use crate::error::{Error, Result};

/// Wrapper for `git status --porcelain`
pub fn git_status_porcelain() -> Result<(String, String)> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid stdout");
    let stderr = String::from_utf8(output.stderr).expect("Invalid stderr");

    if !output.status.success() {
        return Err(Error::InternalError(format!(
            "Failed to execute git add: {stderr}"
        )));
    }

    Ok((stdout, stderr))
}
