//! Commit command

use std::{
    env::{current_dir, set_current_dir},
    process::{Command, Stdio},
};

use clap::Parser;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Input, Select};
use indoc::formatdoc;

/// commit command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// If set, the commit will be pushed to the remote
    #[clap(long, short)]
    pub push: bool,
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
    match set_current_dir(cwd) {
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
    let conv_types = vec![
        "feat: A new feature",
        "fix: A bug fix",
        "docs: Documentation",
        "style: Code styling",
        "refactor: Refactoring code",
        "perf: Performance Improvements",
        "test: Tests",
        "build: Build system",
        "ci: Continuous Integration",
        "cd: Continuous Delivery",
        "chore: Other changes",
    ];
    let select_type = Select::with_theme(&ColorfulTheme::default())
        .items(&conv_types)
        .clear(true)
        .default(0)
        .report(true)
        .with_prompt("Commit type")
        .interact_on_opt(&Term::stderr())
        .unwrap();
    let commit_type = match select_type {
        Some(i) => match i {
            0 => "feat",
            1 => "fix",
            2 => "docs",
            3 => "style",
            4 => "refactor",
            5 => "perf",
            6 => "test",
            7 => "build",
            8 => "ci",
            9 => "cd",
            10 => "chore",
            _ => unreachable!(),
        },
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
    let commit_msg = formatdoc!(
        "
        {}{}: {}
        {}
        {}
        ",
        commit_type,
        scope.map(|s| format!("({})", s)).unwrap_or_default(),
        description,
        body.map(|s| format!("\n{}", s)).unwrap_or_default(),
        breaking_change
            .map(|s| format!("\nBREAKING CHANGE: {}", s))
            .unwrap_or_default(),
    );

    // submit the commit
    term.write_line("Committing …").unwrap();
    let output = Command::new("git")
        .args(["commit", "-m", &commit_msg])
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
        if !output.stdout.is_empty() {
            term.write_line(&String::from_utf8_lossy(&output.stdout))
                .unwrap();
        }
        if !output.stderr.is_empty() {
            term.write_line(&String::from_utf8_lossy(&output.stderr))
                .unwrap();
        }
        if !output.status.success() {
            term.write_line(style("✗ Failed to push").red().to_string().as_str())
                .unwrap();
            return;
        } else {
            term.write_line(
                format!("{} {}", style("✔").green(), style("Pushed commit").bold()).as_str(),
            )
            .unwrap();
            term.write_line(&commit_msg).unwrap();
        }
    }
}
