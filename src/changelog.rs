//! Changelog generator
//!
//! cf. https://keepachangelog.com/en/1.0.0/

use std::marker::PhantomData;

use handlebars::Handlebars;
use serde::Serialize;

use crate::config::Config;

/// Changelog template
const CHANGELOG_TEMPLATE: &str = include_str!("changelog/template.md");

/// Changelog default file name
const FILE_NAME: &str = "CHANGELOG.md";

/// ChnageLogVersion
#[derive(Debug, Serialize)]
pub struct ChangeLogVersion {
    /// Version number
    pub version: String,
    /// Version date
    pub date: String,
    // /// Commits grouped by type
    // pub types: Vec<ChangeLogType>,
}

/// ChangeLog data
#[derive(Debug, Serialize)]
pub struct ChangeLogData {
    /// Commits grouped by version
    pub versions: Vec<ChangeLogVersion>,
}

/// Changelog
#[derive(Debug)]
pub struct ChangeLog<T: Serialize> {
    /// File name
    file_name: String,
    /// Template
    template: String,
    /// Template data
    data: PhantomData<T>,
}

impl<T: Serialize> Default for ChangeLog<T> {
    fn default() -> Self {
        Self {
            file_name: FILE_NAME.to_string(),
            template: CHANGELOG_TEMPLATE.to_string(),
            data: PhantomData,
        }
    }
}

impl<T: Serialize> ChangeLog<T> {
    /// Initializes the changelog
    pub fn init(config: &Config) -> Self {
        Self::default()
    }

    /// Generates the change log file
    pub fn generate(&self, data: &ChangeLogData) -> String {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);

        handlebars.render_template(&self.template, data).unwrap();
        String::new()
    }
}

// loop on ChangeLog Version + Date
// group by type and array of subjects
