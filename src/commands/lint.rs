//! Commit command

use std::{
    env::{current_dir, set_current_dir},
    io,
    process::exit,
    string::ToString,
};

use clap::Parser;
use console::{style, Term};

use crate::{config::Config, conventional::ConventionalCommitMessage};

/// lint command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// Commit message (if ommitted, the message will be read from stdin)
    #[clap(short, long)]
    pub msg: Option<String>,
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

    // get the commig message
    let commit = match &args.msg {
        Some(c) => c.to_string(),
        None => {
            // input is piped
            if atty::is(atty::Stream::Stdin) {
                term.write_line(
                    style("✗ pass a commit message as an option or pipe input")
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(1);
            }
            let mut stdin = String::new();
            io::stdin()
                .read_line(&mut stdin)
                .expect("Cannot read stdin");
            stdin
        }
    };

    // validate the commit message
    let _commit = match ConventionalCommitMessage::parse(&commit, &config.valid_types()) {
        Ok(c) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style("Conventional commit OK").bold()
                )
                .as_str(),
            )
            .unwrap();
            c
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };
}
