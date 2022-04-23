//! Version management

use std::process::Command;

use log::{debug, warn};
use semver::{BuildMetadata, Prerelease, Version};

use crate::{
    config::Config,
    conventional::ConventionalCommitMessage,
    error::{Error, Result},
    git::{git_get_tags, git_log, GitTag},
};

/// GitTag with version information
#[derive(Debug, Clone, Eq)]
pub struct SemverGitTag {
    /// Semver version
    pub version: Version,
    /// Git tag
    pub tag: GitTag,
}

impl PartialEq for SemverGitTag {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl PartialOrd for SemverGitTag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl Ord for SemverGitTag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

/// Trait to convert GitTag into SemverGitTag
pub trait IntoSemverGitTag {
    /// Converts GitTag into SemverGitTag
    fn into_semver(self) -> Result<SemverGitTag>;
}

impl IntoSemverGitTag for GitTag {
    fn into_semver(self) -> Result<SemverGitTag> {
        let version = semver::Version::parse(&self.tag)?;
        Ok(SemverGitTag { version, tag: self })
    }
}

/// Trait to convert Vec<GitTag> into Vec<SemverGitTag>
pub trait IntoSemverGitTags {
    /// Converts GitTag into SemverGitTag
    fn into_semver(self) -> Result<Vec<SemverGitTag>>;
}

impl IntoSemverGitTags for Vec<GitTag> {
    fn into_semver(self) -> Result<Vec<SemverGitTag>> {
        let mut tags = Vec::<SemverGitTag>::new();
        for tag in self {
            tags.push(tag.into_semver()?);
        }
        Ok(tags)
    }
}

/// Increments the repo version
///
/// ## Notes
///
/// The latest version is the latest tag, sorted by semver version,
/// and all commits after that tag are considered to be part of the next version.
pub fn bump_repo_version(config: &Config) -> Result<(Version, Option<Version>)> {
    let repo_version_opt = get_latest_repo_tag()?;

    let log_range = match &repo_version_opt {
        Some(v) => format!("{}..", v.tag.hash),
        None => "".to_string(),
    };
    let commits = git_log(&log_range)?;

    if commits.is_empty() {
        return Err(Error::NoCommits(
            "Cannot bump without new commits".to_string(),
        ));
    }

    let mut conv_commits: Vec<ConventionalCommitMessage> = vec![];
    for c in commits {
        match ConventionalCommitMessage::parse(&c.message, &config.valid_commit_types()) {
            Ok(conv_commit) => {
                debug!("commit to version |> {}", c.message);
                conv_commits.push(conv_commit);
            }
            Err(err) => {
                // NB: skip invalid commits
                warn!(
                    "Invalid conventional commit ({}) |> skipped ({})",
                    c.id, err
                );
            }
        }
    }

    let curr_version = repo_version_opt.map(|v| v.version);

    let next_version = match &curr_version {
        None => Version::new(0, 0, 1),
        Some(curr) => {
            let mut has_minor_change = false;
            let mut has_major_change = false;
            for c in conv_commits {
                if config.type_is_minor_inc(&c.r#type) {
                    has_minor_change = true;
                }
                if c.breaking_change.is_some() {
                    has_major_change = true;
                }
            }

            let mut next = curr.clone();
            next.pre = Prerelease::EMPTY;
            next.build = BuildMetadata::EMPTY;
            if curr.major > 0 {
                if has_major_change {
                    next.major += 1;
                    next.minor = 0;
                    next.patch = 0;
                } else if has_minor_change {
                    next.minor += 1;
                    next.patch = 0;
                } else {
                    next.patch += 1;
                }
            } else {
                // pre 1.0.0
                if has_major_change {
                    next.minor += 1;
                    next.patch = 0;
                } else {
                    next.patch += 1;
                }
            }

            next
        }
    };

    Ok((next_version, curr_version))
}

/// Returns the last repo version
///
/// NB:ordered by semver version number, not timestamp, or tag string
pub fn get_latest_repo_tag() -> Result<Option<SemverGitTag>> {
    let tags = git_get_tags()?;
    let mut versions = tags.into_semver()?;

    // sort by ascending order
    versions.sort();

    // get the latest version
    Ok(versions.last().cloned())
}

/// Executes the extra bump commands
pub fn exec_bump_commands(config: &Config, version: &str) -> Result<Vec<String>> {
    // execute other commands to bump the package(s) version
    let mut executed_cmds = Vec::<String>::new();
    for cfg_command in &config.release.bump_commands {
        let cmd_str = format!("{} ($VERSION={})", cfg_command, version);
        let cmd_args: Vec<&str> = cfg_command.split(' ').collect();
        match Command::new(&cmd_args[0])
            .env("VERSION", version)
            .args(&cmd_args[0..])
            .output()
        {
            Ok(_) => {
                executed_cmds.push(cmd_str);
            }
            Err(err) => {
                return Err(Error::InternalError(format!(
                    "✗ Failed to execute command '{cmd_str}': {err}"
                )));
            }
        };
    }
    Ok(executed_cmds)
}
