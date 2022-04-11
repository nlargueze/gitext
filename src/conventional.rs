//! Conventional commit

use std::{io::BufRead, string::ToString};

use indoc::formatdoc;
use regex::Regex;

use crate::{
    config::Config,
    error::{Error, Result},
};

/// Conventional commit
#[derive(Debug)]
pub struct ConventionalCommit {
    /// Commit type
    pub r#type: String,
    /// Commit scope
    pub scope: Option<String>,
    /// Commit description
    pub description: String,
    /// Commit body
    pub body: Option<String>,
    /// Breaking change
    pub breaking_change: Option<String>,
}

impl ConventionalCommit {
    /// Parses a conventionl commit from a string
    pub fn parse(s: &str, cfg: &Config) -> Result<Self> {
        let mut r#type = String::new();
        let mut scope: Option<String> = None;
        let mut description = String::new();
        let mut body: Option<String> = None;
        let mut breaking_change: Option<String> = None;

        // cf. https://2fd.github.io/rust-regex-playground
        let regex_prefix = Regex::new(r"(?P<type>[[:word:]]+)(?P<scope>(\([0-9A-Za-z_\s]+\))?)!?")
            .expect("Invalid regex");

        for (i, line_res) in s.as_bytes().lines().enumerate() {
            let line = line_res?;
            // eprintln!("line {i}");
            // eprintln!("{line}");

            if i == 0 {
                let parts: Vec<_> = line.splitn(2, ":").collect();
                if parts.len() != 2 {
                    return Err(Error::InvalidCommit(
                        "Invalid commit: missing description".to_string(),
                    ));
                }
                let prefix = parts[0].trim().to_string();
                description = parts[1].trim().to_string();
                if let Some(capts) = regex_prefix.captures(&prefix) {
                    r#type = match capts.name("type") {
                        Some(m) => {
                            let t = m.as_str().to_string();
                            eprintln!("TYPE:{}", t);
                            if !cfg.commits.types.contains_key(&t) {
                                return Err(Error::InvalidCommit(format!(
                                    "Invalid commit type: {}",
                                    t
                                )));
                            }
                            t
                        }
                        None => {
                            return Err(Error::InvalidCommit(format!(
                                "Missing commit type: {}",
                                line
                            )));
                        }
                    };
                    scope = match capts.name("scope") {
                        Some(m) => {
                            let s = m.as_str().trim().to_string();
                            eprintln!("SCOPE:{}", s);
                            if s.is_empty() {
                                None
                            } else {
                                Some(s.trim_start_matches('(').trim_end_matches(')').to_string())
                            }
                        }
                        None => None,
                    };

                    // eprintln!("type: {:?}", r#type);
                    // eprintln!("scope: {:?}", scope);
                    // eprintln!("desc: {:?}", description);
                } else {
                    return Err(Error::InvalidCommit(format!("Invalid commit: {}", line)));
                }
            } else {
                // other lines > 0
                if line.starts_with("BREAKING_CHANGE:") {
                    breaking_change = Some(line.trim_start_matches("BREAKING_CHANGE:").to_string());
                } else {
                    // add to body
                    if let Some(b) = body {
                        body = Some(format!("{}\n{}", b, line));
                    } else {
                        body = Some(line);
                    }
                }
            }
        }

        Ok(ConventionalCommit {
            r#type,
            scope,
            description,
            body,
            breaking_change,
        })
    }
}

impl ToString for ConventionalCommit {
    fn to_string(&self) -> String {
        formatdoc!(
            "
            {}{}{}: {}
            {}
            {}
            ",
            self.r#type,
            self.scope
                .clone()
                .map(|s| format!("({})", s))
                .unwrap_or_default(),
            self.breaking_change
                .clone()
                .map(|_| "!")
                .unwrap_or_default(),
            self.description,
            self.body
                .clone()
                .map(|s| format!("\n{}", s))
                .unwrap_or_default(),
            self.breaking_change
                .clone()
                .map(|s| format!("\nBREAKING CHANGE: {}", s))
                .unwrap_or_default(),
        )
    }
}
