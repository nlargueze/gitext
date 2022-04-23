//! Changelog generator
//!
//! cf. https://keepachangelog.com/en/1.0.0/

use std::collections::BTreeMap;

use chrono::Utc;
use handlebars::Handlebars;
use indoc::indoc;
use log::warn;
use serde::Serialize;

use crate::{
    config::Config,
    conventional::ConventionalCommitMessage,
    error::Result,
    git::{get_config_origin_url, git_get_tags, git_log},
    utils::StringExt,
    version::IntoSemverGitTags,
};

/// Changelog template
const CHANGELOG_TEMPLATE: &str = indoc!(
    "# Changelog All notable changes to this project will be documented in this file.

    {{#each releases}}
    ## [{{this.version}}] - {{this.date}}
    {{#if this.history_url}}

    {{this.history_url}}
    {{/if}}

    {{#each this.groups}}
    ### {{this.title}}

    {{#each this.commits}}
    - {{this}}
    {{/each}}

    {{/each}}
    {{/each}}"
);

/// Changelog release group
#[derive(Debug, Serialize)]
struct ChangeLogReleaseGroup {
    title: String,
    commits: Vec<String>,
}

/// Changelog release
#[derive(Debug, Serialize)]
struct ChangeLogRelease {
    /// Release version
    version: String,
    /// Release date
    date: String,
    /// Release commit history link
    history_url: String,
    /// Commit group
    groups: BTreeMap<String, ChangeLogReleaseGroup>,
}

/// ChangeLog data
#[derive(Debug, Serialize)]
struct ChangeLogData {
    /// Commits grouped by release
    releases: Vec<ChangeLogRelease>,
}

/// Changelog
#[derive(Debug)]
pub struct ChangeLog {
    /// Change log template engine
    registry: Handlebars<'static>,
}

impl ChangeLog {
    /// Initializes the changelog
    pub fn init() -> Result<Self> {
        // init template registry
        let mut registry = Handlebars::new();
        // registry.set_strict_mode(true);
        registry.register_template_string("changelog", CHANGELOG_TEMPLATE)?;

        Ok(Self { registry })
    }

    /// Generates the change log file
    pub fn generate(&self, config: &Config, next_version: &str) -> Result<String> {
        // parse commits
        let mut data = ChangeLogData { releases: vec![] };
        data.releases.push(ChangeLogRelease {
            version: next_version.to_string(),
            date: Utc::now().format("%Y-%m-%d").to_string(),
            history_url: "".to_string(),
            groups: BTreeMap::new(),
        });

        // read all logs from the start of the repository (latest to earliest)
        let commits = git_log("")?;
        // read all tags from the repository
        let tags = git_get_tags()?.into_semver()?;

        for c in commits {
            let commit_tag = tags.iter().find(|t| t.tag.commit_hash == c.id);
            match commit_tag {
                Some(t) => {
                    // commit has a tag which means that it is another version
                    data.releases.push(ChangeLogRelease {
                        version: t.version.to_string(),
                        date: t.tag.date.format("%Y-%m-%d").to_string(),
                        history_url: "".to_string(),
                        groups: BTreeMap::new(),
                    });
                }
                None => {}
            }

            // > get type and subject from the message
            let (r#type, subject) =
                match ConventionalCommitMessage::parse(&c.message, &config.valid_commit_types()) {
                    Ok(c) => (c.r#type.clone(), c.subject.clone()),
                    Err(err) => {
                        // NB: add as a specific group
                        let mut short_id = c.id.clone();
                        short_id.truncate(7);
                        warn!("Commit ({}) is unconventional ({})", short_id, err);
                        let commit_msg_first_line = c.message.lines().next().unwrap();
                        (
                            "uncategorized".to_string(),
                            commit_msg_first_line.to_string(),
                        )
                    }
                };

            // add release commit group if unexisting
            let group = data
                .releases
                .last_mut()
                .unwrap()
                .groups
                .entry(r#type.clone())
                .or_insert_with(|| {
                    let title = if r#type == "uncategorized" {
                        "Uncategorized".to_string()
                    } else if let Some(t) = config.changelog.types.get(r#type.as_str()) {
                        t.clone()
                    } else {
                        r#type.clone()
                    };

                    ChangeLogReleaseGroup {
                        title,
                        commits: vec![],
                    }
                });

            // add commit to group
            group.commits.push(subject.to_uppercase_first());
        }

        // debug
        // eprintln!("{:#?}", data);

        // add history link
        // [Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/v1.0.0...HEAD
        // [1.0.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.0.2...v0.0.1
        let origin_url = get_config_origin_url()?;
        let mut from_ref: Option<String> = None;
        for release in data.releases.iter_mut().rev() {
            if let Some(ref_start) = from_ref {
                let ref_end = if release.version == "Unreleased" {
                    "HEAD".to_string()
                } else {
                    release.version.clone()
                };
                release.history_url = format!("{}/compare/{}...{}", origin_url, ref_start, ref_end);
            }
            from_ref = Some(release.version.clone());
        }

        // render template
        let txt = self.registry.render("changelog", &data)?;
        Ok(txt)
    }
}
