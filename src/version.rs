//! Version management

use log::{debug, warn};
use semver::{BuildMetadata, Prerelease, Version};

use crate::{
    config::Config,
    conventional::ConventionalCommitMessage,
    error::{Error, Result},
    git::{git_get_tags, git_log, GitTag},
};

/// Repo version
#[derive(Debug, Clone, Eq)]
pub struct RepoVersion {
    /// Semver version
    pub version: Version,
    /// Git tag
    pub tag: GitTag,
}

impl PartialEq for RepoVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl PartialOrd for RepoVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl Ord for RepoVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

/// Increments the repo version
pub fn bump_repo_version(config: &Config) -> Result<(Version, Option<Version>)> {
    let repo_version_opt = get_latest_repo_version()?;

    let log_range = match &repo_version_opt {
        Some(v) => format!("{}..", v.tag.hash),
        None => "".to_string(),
    };
    let commits = git_log(&log_range)?;

    let mut conv_commits: Vec<ConventionalCommitMessage> = vec![];
    if conv_commits.is_empty() {
        return Err(Error::NoCommits(
            "Cannot bump without new commits".to_string(),
        ));
    }
    for c in commits {
        match ConventionalCommitMessage::parse(&c.message, &config.valid_types()) {
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
pub fn get_latest_repo_version() -> Result<Option<RepoVersion>> {
    let tags = git_get_tags()?;

    // convert to versions
    let mut versions = tags
        .iter()
        .map(|tag| {
            semver::Version::parse(&tag.tag)
                .map_err(Error::Semver)
                .map(|version| RepoVersion {
                    version,
                    tag: tag.clone(),
                })
        })
        .collect::<Result<Vec<_>>>()?;

    // sort by ascending order
    versions.sort();

    // get the latest version
    Ok(versions.last().cloned())
}
