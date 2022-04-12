//! Commit command

use std::{
    env::{current_dir, set_current_dir},
    num::ParseIntError,
    process::exit,
    string::ToString,
};

use clap::Parser;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Input, Select};

use crate::{
    config::Config,
    conventional::ConventionalCommit,
    git::{add::git_add, git_commit, git_push},
};

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

    // git commit
    // > type
    let r#type = {
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
    };

    // > scope
    let scope = {
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
    };

    // > subject
    let subject = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit subject")
        .report(true)
        .interact_text()
        .unwrap();

    // > body
    let body = {
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Commit message body ?")
            .report(true)
            .default(false)
            .interact()
            .unwrap()
        {
            false => None,
            true => {
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
        }
    };

    // > breaking changes
    let breaking_change = {
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Breaking change ?")
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
    };

    // > closed issues
    let closed_issues = {
        let issues_str = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Closed issues (comma separated) ?".to_string())
            .report(true)
            .allow_empty(true)
            .interact_text()
            .unwrap();
        match issues_str.as_str() {
            "" => None,
            s => {
                let issues: Result<Vec<_>, ParseIntError> = s
                    .split(',')
                    .enumerate()
                    .map(|(_i, p)| p.trim().parse::<u32>())
                    .collect();
                match issues {
                    Ok(ids) => Some(ids),
                    Err(err) => {
                        term.write_line(
                            style(format!("✗ Invalid issue id: {err}"))
                                .red()
                                .to_string()
                                .as_str(),
                        )
                        .unwrap();
                        exit(1);
                    }
                }
            }
        }
    };

    // write the commit message
    let commit = ConventionalCommit {
        r#type,
        scope,
        subject,
        body,
        breaking_change,
        closed_issues,
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
        term.write_line("--- COMMIT MESSAGE ---").unwrap();
        term.write_line(&commit_msg).unwrap();
        exit(0);
    }

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

    // submit the commit
    term.write_line("Committing …").unwrap();
    match git_commit(&commit_msg) {
        Ok(_) => {
            term.clear_last_lines(1).unwrap();
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
        Err(err) => {
            term.clear_last_lines(1).unwrap();
            term.write_line(
                style(format!("✗ Failed to commit: {err}"))
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            exit(1);
        }
    }

    // git push
    if args.push {
        term.write_line("Pushing …").unwrap();
        match git_push() {
            Ok(_) => {
                term.clear_last_lines(1).unwrap();
                term.write_line(
                    format!("{} {}", style("✔").green(), style("Pushed commit").bold()).as_str(),
                )
                .unwrap();
            }
            Err(err) => {
                term.clear_last_lines(1).unwrap();
                term.write_line(
                    style(format!("✗ Failed to push: {err}"))
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(1);
            }
        };
    }
}
