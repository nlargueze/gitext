//! Git push wrapper

use std::process::Command;

use crate::error::{Error, Result};

/// Wrapper for `git push`
pub fn git_push() -> Result<()> {
    let output = Command::new("git")
        .args(["push"])
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        return Err(Error::InternalError(
            "Failed to execute git push".to_string(),
        ));
    }
    Ok(())
}
