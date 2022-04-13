//! Version management

use semver::{BuildMetadata, Prerelease, Version};

use crate::{
    error::{Error, Result},
    git::git_get_tags,
};

/// Initial version
const INITIAL_VERSION: Version = Version::new(0, 0, 1);

/// Returns the last git tag as a semver version
pub fn get_repo_current_version() -> Result<Option<Version>> {
    let tags = git_get_tags()?;

    // convert to versions
    let mut versions = tags
        .iter()
        .map(|tag| Version::parse(&tag.tag).map_err(|err| Error::Semver(err)))
        .collect::<Result<Vec<_>>>()?;

    // sort by ascending order
    versions.sort();

    // get the latest version
    Ok(versions.last().cloned())
}

/// Trait that commits must implement for version management
pub trait SemverCommit {
    /// Checks if the commit has a backwards compatible API, and adds fucntionalities (vs fixing bugs)
    fn has_minor_change(&self) -> bool;
    /// Checks if the commit has an API incompatible breaking change.
    fn has_major_change(&self) -> bool;
}

/// Increments the version based on a list of conventional commits after the current version
pub fn increment_repo_version(commits: &Vec<impl SemverCommit>) -> Result<Version> {
    let curr_version = get_repo_current_version()?;

    let new_version = match curr_version {
        None => INITIAL_VERSION,
        Some(curr_version) => {
            let mut has_minor_change = false;
            let mut has_major_change = false;
            for c in commits {
                if c.has_minor_change() {
                    has_minor_change = true
                }
                if c.has_major_change() {
                    has_major_change = true
                }
            }

            let mut new_version = curr_version.clone();
            new_version.pre = Prerelease::EMPTY;
            new_version.build = BuildMetadata::EMPTY;
            if curr_version.major > 0 {
                if has_major_change {
                    new_version.major += 1;
                    new_version.minor = 0;
                    new_version.patch = 0;
                } else if has_minor_change {
                    new_version.minor += 1;
                    new_version.patch = 0;
                } else {
                    new_version.patch += 1;
                }
            } else {
                // pre 1.0.0
                if has_major_change {
                    new_version.minor += 1;
                    new_version.patch = 0;
                } else if has_minor_change {
                    new_version.patch += 1;
                } else {
                    new_version.patch += 1;
                }
            }

            new_version
        }
    };

    Ok(new_version)
}
