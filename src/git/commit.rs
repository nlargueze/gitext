//! Git commits

use std::{fmt::Display, process::Command};

use chrono::{DateTime, FixedOffset, Utc};

use crate::error::{Error, Result};

/// A git commit
#[derive(Debug, Clone)]
pub struct GitCommit {
    /// Commit hash
    pub hash: String,
    /// Commit timestamp
    pub timestamp: DateTime<Utc>,
    /// Author
    pub author: String,
    /// Subject
    pub subject: String,
}

impl Default for GitCommit {
    fn default() -> Self {
        Self {
            hash: Default::default(),
            timestamp: Utc::now(),
            author: Default::default(),
            subject: Default::default(),
        }
    }
}

impl Display for GitCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "hash:{}", self.hash)?;
        writeln!(f, "ts:{}", self.timestamp.to_rfc3339())?;
        writeln!(f, "author:{}", self.author)?;
        writeln!(f, "subject:{}", self.subject)?;
        Ok(())
    }
}

/// Wrapper for `git commit`
pub fn git_commit() -> Result<()> {
    Ok(())
}

/// Wrapper for `git commit`
pub fn get_commits() -> Result<Vec<GitCommit>> {
    // git log --format=----------%nhash:%H%nts:%ad%nauthor:%an%nsubject:%s --date=unix
    let output = Command::new("git")
        .args([
            "log",
            "--format=----------%nhash:%H%nts:%ad%nauthor:%an%nsubject:%s",
            "--date=iso-strict",
        ])
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        return Err(Error::InternalError("Failed to get git logs".to_string()));
    }

    let mut commits: Vec<GitCommit> = Vec::new();
    let mut commit = GitCommit::default();
    for (i, line) in String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .enumerate()
    {
        // println!("{}", line);

        // > new commit log
        if line == "----------" {
            if i > 0 {
                commits.push(commit);
            }
            commit = GitCommit::default();
            continue;
        }
        // process logs
        let parts: Vec<_> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            panic!("Invalid log line: {line}");
        }
        let field = parts[0];
        let value = parts[1];
        match field {
            "hash" => commit.hash = value.to_string(),
            "ts" => {
                let d =
                    DateTime::<FixedOffset>::parse_from_rfc3339(value).expect("Invalid timestamp");
                commit.timestamp = d.with_timezone(&Utc);
            }
            "author" => commit.author = value.to_string(),
            "subject" => commit.subject = value.to_string(),
            _ => panic!("Invalid log line field: {field}"),
        }
    }

    Ok(commits)
}
