//! wrapper for `git tag`

use std::{fmt::Display, process::Command};

use chrono::{DateTime, FixedOffset, Utc};
use semver::Version;

use crate::error::{Error, Result};

/// Git tag
#[derive(Debug, Clone, Eq, Ord)]
pub struct Tag {
    /// Tag name
    pub tag: String,
    /// Tag commmit hash
    pub hash: String,
    /// Tag date
    pub date: DateTime<Utc>,
    /// Tag message
    pub message: Option<String>,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Tag) -> Option<std::cmp::Ordering> {
        Some(self.tag.cmp(&other.tag))
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.tag,
            self.message
                .clone()
                .map(|m| format!(" | {}", m))
                .unwrap_or_default()
        )
    }
}

impl Tag {
    /// Returns the semver version for a tag
    pub fn version(&self) -> Result<Version> {
        Ok(Version::parse(&self.tag)?)
    }
}

/// Wrapper for `git tag`
pub fn git_tag() -> Result<Vec<Tag>> {
    let output = Command::new("git")
        .args([
            "tag",
            "--list",
            "--format=%(refname:short)|%(creatordate:iso-strict)|%(objectname)",
        ])
        .output()?;
    if !output.status.success() {
        return Err(Error::InternalError("Failed to get git logs".to_string()));
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

            Tag {
                tag: tag_str.to_string(),
                hash: hash_str.to_string(),
                date: DateTime::<FixedOffset>::parse_from_rfc3339(dt_str)
                    .unwrap()
                    .with_timezone(&Utc),
                message: None,
            }
        })
        .collect();

    tags.sort();

    Ok(tags)
}
