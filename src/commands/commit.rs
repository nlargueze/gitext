//! Commit command

use std::{
    env::{current_dir, set_current_dir},
    process::{exit, Command, Stdio},
    string::ToString,
};

use clap::Parser;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Input, Select};

use crate::{config::Config, conventional::ConventionalCommit, git::add::git_add};

/// commit command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// If set, the commit will be pushed to the remote
    #[clap(long, short)]
    pub push: bool,
    /// If set, no commit is made
    #[clap(long)]
    pub dry_run: bool,
    /// Commit type
    #[clap(long, short)]
    pub r#type: Option<String>,
    /// Commit scope
    #[clap(long, short)]
    pub scope: Option<String>,
    /// Commit description
    #[clap(long, short)]
    pub desc: Option<String>,
    /// Commit body
    #[clap(long, short)]
    pub body: Option<String>,
    /// Commit breaking change
    #[clap(long, short)]
    pub breaking_change: Option<String>,
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
            exit(1);
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
            exit(1);
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
            exit(1);
        }
    };

    // git add -A
    term.write_line("Staging changes …").unwrap();
    match git_add() {
        Ok(_) => {
            term.clear_last_lines(1).unwrap();
            term.write_line(
                format!("{} {}", style("✔").green(), style("Changes staged").bold()).as_str(),
            )
            .unwrap();
        }
        Err(err) => {
            term.clear_last_lines(1).unwrap();
            term.write_line(
                style(format!("✗ Failed to stage changes {err}"))
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            exit(1);
        }
    }

    // git commit
    // > type
    let r#type = match &args.r#type {
        Some(r#type) => r#type.to_string(),
        None => {
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
            let r#type = match select_type {
                Some(i) => commit_types_keys[i].clone(),
                None => {
                    term.write_line(
                        style("✗ A commit type must be selected")
                            .red()
                            .to_string()
                            .as_str(),
                    )
                    .unwrap();
                    exit(1);
                }
            };
            r#type
        }
    };

    // > scope
    let scope = match &args.scope {
        Some(s) => Some(s.to_string()),
        None => {
            let scope: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Commit scope")
                .report(true)
                .allow_empty(true)
                .interact_text()
                .unwrap();
            if scope.is_empty() {
                None
            } else {
                Some(scope)
            }
        }
    };

    // > description
    let description = match &args.desc {
        Some(d) => d.to_string(),
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Commit description")
            .report(true)
            .interact_text()
            .unwrap(),
    };

    // > body
    let body = match &args.body {
        Some(b) => Some(b.to_string()),
        None => {
            match Editor::new()
                .executable("micro")
                .require_save(true)
                .trim_newlines(true)
                .edit("")
            {
                Ok(b) => b,
                Err(err) => {
                    term.write_line(
                        style(format!("✗ Failed to edit body: {err}"))
                            .red()
                            .to_string()
                            .as_str(),
                    )
                    .unwrap();
                    exit(1);
                }
            }
        }
    };

    // > breaking changes
    let breaking_change = match &args.breaking_change {
        Some(b) => Some(b.to_string()),
        None => {
            match Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Breaking change")
                .report(true)
                .default(false)
                .interact()
                .unwrap()
            {
                false => None,
                true => Some(
                    Input::<String>::with_theme(&ColorfulTheme::default())
                        .with_prompt("Breaking change description".to_string())
                        .report(true)
                        .allow_empty(true)
                        .interact_text()
                        .unwrap(),
                ),
            }
        }
    };

    // write the commit message
    let commit = ConventionalCommit {
        r#type,
        scope,
        description,
        body,
        breaking_change,
    };

    let commit_msg = commit.to_string();

    // validate the commit message
    match ConventionalCommit::parse(&commit_msg, &config) {
        Ok(_) => {}
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    // dry-run
    if args.dry_run {
        term.write_line("--- COMMIT ---").unwrap();
        term.write_line(&commit_msg).unwrap();
        exit(0);
    }

    // submit the commit
    term.write_line("Committing …").unwrap();
    let mut cmd = Command::new("git");
    cmd.args(["commit", "-m", &commit_msg]);
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
        exit(1);
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
            exit(1);
        } else {
            term.write_line(
                format!("{} {}", style("✔").green(), style("Pushed commit").bold()).as_str(),
            )
            .unwrap();
        }
    }
}
