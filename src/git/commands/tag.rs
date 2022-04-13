//! Get/Set tags

use std::process::Command;

use chrono::{DateTime, FixedOffset, Utc};

use crate::{
    error::{Error, Result},
    git::tag::GitTag,
};

/// Wrapper for `git tag --list`
pub fn git_get_tags() -> Result<Vec<GitTag>> {
    let output = Command::new("git")
        .args([
            "tag",
            "--list",
            "--format=%(refname:short)|%(creatordate:iso-strict)|%(objectname)",
        ])
        .output()?;
    if !output.status.success() {
        return Err(Error::InternalError("Failed to get git tags".to_string()));
    }

    let mut tags: Vec<_> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.splitn(3, '|').collect();
            if parts.len() != 3 {
                panic!("Invalid tag line: {}", line);
            }
            let tag_str = parts[0];
            let dt_str = parts[1];
            let hash_str = parts[2];

            GitTag {
                tag: tag_str.to_string(),
                hash: hash_str.to_string(),
                date: DateTime::<FixedOffset>::parse_from_rfc3339(dt_str)
                    .unwrap()
                    .with_timezone(&Utc),
                message: None,
            }
        })
        .collect();

    Ok(tags)
}

/// Wrapper for `git tag $t -a -m $m`
pub fn git_set_tag(tag: &str, msg: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["tag", tag, "-a", "-m", msg])
        .output()?;

    let stderr = String::from_utf8(output.stderr).expect("Invalid stderr");

    if !output.status.success() {
        return Err(Error::InternalError(format!(
            "Failed to set git tag: {stderr}"
        )));
    }

    Ok(())
}
