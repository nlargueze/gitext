//! Commit commands

use std::process::Command;

use crate::error::{Error, Result};

/// Wrapper for `git commit`
pub fn git_commit(msg: &str) -> Result<(String, String)> {
    let mut cmd = Command::new("git");
    cmd.args(["commit", "-m", msg]);
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

/// Wrapper for `git commit --amend`
pub fn git_commit_amend() -> Result<(String, String)> {
    let mut cmd = Command::new("git");
    cmd.args(["commit", "--amend", "--no-edit"]);
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
