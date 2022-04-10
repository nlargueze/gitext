//! Commit command

use std::{
    env::{current_dir, set_current_dir},
    process::{Command, Stdio},
    string::ToString,
};

use clap::Parser;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Input, Select};

use crate::{commit::Commit, config::Config};

/// commit command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// If set, the commit will be pushed to the remote
    #[clap(long, short)]
    pub push: bool,
    /// If set, the commit is amended
    #[clap(long, short)]
    pub amend: bool,
}

/// Runs the command
pub fn run(args: &Args) {
    let term = Term::stderr();

    let cwd = match current_dir() {
        Ok(path) => path,
        Err(err) => {
            term.write_line(
                style(format!("✗ Internal error: {err}"))
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            return;
        }
    };

    let cwd = if let Some(arg_cwd) = &args.cwd {
        cwd.join(arg_cwd)
    } else {
        cwd
    };
    match set_current_dir(&cwd) {
        Ok(_) => {}
        Err(err) => {
            term.write_line(
                style(format!("✗ Failed to set current directory: {err}"))
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            return;
        }
    };

    // load the config
    let config = match Config::load(&cwd) {
        Ok(cfg) => cfg,
        Err(err) => {
            term.write_line(
                style(format!("✗ Missing or invalid config : {err}"))
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            return;
        }
    };

    // git add -A
    term.write_line("Staging changes …").unwrap();
    let output = Command::new("git")
        .args(["add", "-A"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");
    term.clear_last_lines(1).unwrap();
    if !output.stdout.is_empty() {
        term.write_line(&String::from_utf8_lossy(&output.stdout))
            .unwrap();
    }
    if !output.stderr.is_empty() {
        term.write_line(&String::from_utf8_lossy(&output.stderr))
            .unwrap();
    }
    if !output.status.success() {
        term.write_line(
            style("✗ Failed to stage changes")
                .red()
                .to_string()
                .as_str(),
        )
        .unwrap();
        return;
    } else {
        term.write_line(
            format!("{} {}", style("✔").green(), style("Changes staged").bold()).as_str(),
        )
        .unwrap();
    }

    // git commit
    // > type
    let commit_types: Vec<_> = config
        .commits
        .types
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v.desc))
        .collect();
    let commit_types_keys: Vec<_> = config
        .commits
        .types
        .iter()
        .map(|(k, _v)| format!("{}", k))
        .collect();
    let select_type = Select::with_theme(&ColorfulTheme::default())
        .items(&commit_types)
        .clear(true)
        .default(0)
        .report(true)
        .with_prompt("Commit type")
        .interact_on_opt(&Term::stderr())
        .unwrap();
    let commit_type = match select_type {
        Some(i) => commit_types_keys[i].clone(),
        None => {
            term.write_line(
                style("✗ A commit type must be selected")
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            return;
        }
    };
    // > scope
    let scope: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit scope")
        .report(true)
        .allow_empty(true)
        .interact_text()
        .unwrap();
    let scope = if scope.is_empty() { None } else { Some(scope) };
    // > description
    let description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit description")
        .report(true)
        .interact_text()
        .unwrap();
    // > body
    let body = match Editor::new()
        .executable("micro")
        .require_save(true)
        .trim_newlines(true)
        .edit("")
    {
        Ok(rv) => rv,
        Err(err) => {
            term.write_line(
                style(format!("✗ Failed to edit body: {err}"))
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            return;
        }
    };
    // > breaking changes
    let is_breaking_change = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Breaking change")
        .report(true)
        .default(false)
        .interact()
        .unwrap();
    let breaking_change = if is_breaking_change {
        let desc: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Breaking change description")
            .report(true)
            .allow_empty(true)
            .interact_text()
            .unwrap();
        Some(desc)
    } else {
        None
    };

    // write the commit message
    let commit = Commit {
        r#type: commit_type,
        scope,
        description,
        body,
        breaking_change,
    };

    let commit_msg = commit.to_string();

    // submit the commit
    term.write_line("Committing …").unwrap();
    let mut cmd = Command::new("git");
    cmd.args(["commit", "-m", &commit_msg]);
    if args.amend {
        cmd.arg("--amend");
    }
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");
    term.clear_last_lines(1).unwrap();
    if !output.stdout.is_empty() {
        term.write_line(&String::from_utf8_lossy(&output.stdout))
            .unwrap();
    }
    if !output.stderr.is_empty() {
        term.write_line(&String::from_utf8_lossy(&output.stderr))
            .unwrap();
    }
    if !output.status.success() {
        term.write_line(style("✗ Failed to commit").red().to_string().as_str())
            .unwrap();
        return;
    } else {
        term.write_line(
            format!(
                "{} {}",
                style("✔").green(),
                style("Committed changes").bold()
            )
            .as_str(),
        )
        .unwrap();
        term.write_line(&commit_msg).unwrap();
    }

    // git push
    if args.push {
        term.write_line("Pushing …").unwrap();
        let output = Command::new("git")
            .args(["push"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to execute command");
        term.clear_last_lines(1).unwrap();
        // if !output.stdout.is_empty() {
        //     term.write_line(&String::from_utf8_lossy(&output.stdout))
        //         .unwrap();
        // }
        if !output.status.success() {
            term.write_line(style("✗ Failed to push").red().to_string().as_str())
                .unwrap();
            if !output.stderr.is_empty() {
                term.write_line(
                    style(String::from_utf8_lossy(&output.stderr))
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
            }
            return;
        } else {
            term.write_line(
                format!("{} {}", style("✔").green(), style("Pushed commit").bold()).as_str(),
            )
            .unwrap();
        }
    }
}
