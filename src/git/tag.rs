//! Git tags

use std::{fmt::Display, process::Command};

use semver::Version;

use crate::error::{Error, Result};

/// Git tag
#[derive(Debug, Clone, Eq, Ord)]
pub struct GitTag {
    /// Tag name
    pub tag: String,
    /// Tag message
    pub message: Option<String>,
}

impl PartialEq for GitTag {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl PartialOrd for GitTag {
    fn partial_cmp(&self, other: &GitTag) -> Option<std::cmp::Ordering> {
        Some(self.tag.cmp(&other.tag))
    }
}

impl Display for GitTag {
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

impl GitTag {
    /// Creates a new tag
    pub fn new(tag: &str) -> Self {
        GitTag {
            tag: tag.to_string(),
            message: None,
        }
    }

    /// Returns the semver version
    pub fn version(&self) -> Result<Version> {
        Ok(Version::parse(&self.tag)?)
    }
}

/// Get all tags
///
/// This is a wrapper for `git log`
pub fn get_tags() -> Result<Vec<GitTag>> {
    let output = Command::new("git")
        .args([
            "tag",
            "--list",
            "--format=%(refname:short)|%(creatordate:short)",
        ])
        .output()?;
    if !output.status.success() {
        return Err(Error::InternalError("Failed to get git logs".to_string()));
    }

    let mut tags: Vec<_> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.splitn(2, '|').collect();
            if parts.len() != 2 {
                panic!("Invalid tag line: {}", line);
            }
            GitTag {
                tag: parts[0].to_string(),
                message: None,
            }
        })
        .collect();

    tags.sort();

    Ok(tags)
}

/// Returns the latest git version
pub fn get_latest_version() -> Result<Option<Version>> {
    let commits = get_tags()?;
    let mut versions = commits
        .iter()
        .map(|c| c.version())
        .collect::<Result<Vec<_>>>()?;

    versions.sort();

    Ok(versions.last().cloned())
}
