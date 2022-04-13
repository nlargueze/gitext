//! Wrapper for git log

use std::process::Command;

use chrono::{DateTime, FixedOffset, Utc};

use crate::{
    error::{Error, Result},
    git::commit::GitCommit,
};

/// Runs `git log` and returns the commits
///
/// ## Notes
///
/// `git log id1..`: get all logs from ref `id1` (exclusive) to HEAD
///
/// `git log id1..id2`: get all logs from ref `id1` (exclusive) to the ref `id2` (inclusive)
pub fn git_log(log_range: &str) -> Result<Vec<GitCommit>> {
    let mut cmd = Command::new("git");
    cmd.args([
        "log",
        "--format=hash:%H%nts:%ad%nauthor:%an%nmessage:%B----------",
        "--date=iso-strict",
    ]);
    if !log_range.is_empty() {
        cmd.arg(log_range);
    }
    let output = cmd.output().expect("Failed to execute command");
    if !output.status.success() {
        return Err(Error::InternalError("Failed to get git logs".to_string()));
    }

    let mut commits: Vec<GitCommit> = Vec::new();
    let mut commit = GitCommit::default();
    for (_i, line) in String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .enumerate()
    {
        // eprintln!("|> {}", line);

        if line.starts_with("hash:") {
            let id = line.strip_prefix("hash:").unwrap();
            commit.id = id.to_string();
        } else if line.starts_with("ts:") {
            let ts = line.strip_prefix("ts:").unwrap();
            let d = DateTime::<FixedOffset>::parse_from_rfc3339(ts).expect("Invalid timestamp");
            commit.timestamp = d.with_timezone(&Utc);
        } else if line.starts_with("author:") {
            let author = line.strip_prefix("author:").unwrap();
            commit.author = author.to_string();
        } else if line.starts_with("message:") {
            let msg = line.strip_prefix("message:").unwrap();
            commit.message.push_str(msg);
        } else if line.starts_with("----------") {
            commits.push(commit);
            commit = GitCommit::default();
        } else {
            commit.message.push_str(line);
        }
    }

    Ok(commits)
}
