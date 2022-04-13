//! Wrappers for the `git config` command.

use std::process::Command;

use crate::error::{Error, Result};

/// Returns the git origin URL.
pub fn get_config_origin_url() -> Result<String> {
    let output = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        return Err(Error::InternalError(
            "Failed to execute git config".to_string(),
        ));
    }

    let stdout = match String::from_utf8(output.stdout) {
        Ok(s) => s.trim().to_string(),
        Err(err) => {
            return Err(Error::InternalError(format!("Invalid stdout ({err})")));
        }
    };

    let stdout = stdout.strip_suffix(".git").unwrap();

    Ok(stdout.to_string())
}
