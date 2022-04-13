//! Wrapper for git log

use std::process::Command;

use chrono::{DateTime, FixedOffset, Utc};

use crate::{
    error::{Error, Result},
    git::commit::GitCommit,
};

/// Wrapper for `git_log`
pub fn git_log() -> Result<Vec<GitCommit>> {
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
            "hash" => commit.id = value.to_string(),
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
