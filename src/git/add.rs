//! Git stage/add

use std::process::Command;

use crate::error::{Error, Result};

/// Wrapper for `git add -A`
pub fn git_add() -> Result<()> {
    let output = Command::new("git")
        .args(["add", "-A"])
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        return Err(Error::InternalError(
            "Failed to execute git add".to_string(),
        ));
    }
    Ok(())
}
