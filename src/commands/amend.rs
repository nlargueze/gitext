//! Commit command

use std::{
    env::{current_dir, set_current_dir},
    process::{exit, Command, Stdio},
    string::ToString,
};

use clap::Parser;
use console::{style, Term};

/// amend command arguments
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
        exit(1);
    } else {
        term.write_line(
            format!("{} {}", style("✔").green(), style("Changes staged").bold()).as_str(),
        )
        .unwrap();
    }

    // git commit (amend)
    term.write_line("Amending commit …").unwrap();
    let mut cmd = Command::new("git");
    cmd.args(["commit", "--amend", "--no-edit"]);
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
                style("Committed changes (amend)").bold()
            )
            .as_str(),
        )
        .unwrap();
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
