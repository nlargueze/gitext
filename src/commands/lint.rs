//! Commit command

use std::{
    env::{current_dir, set_current_dir},
    process::exit,
    string::ToString,
};

use clap::Parser;
use console::{style, Term};

use crate::{config::Config, conventional::ConventionalCommit};

/// lint command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Commit message
    #[clap(short, long)]
    pub msg: String,
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

    // validate the commit message
    let _commit = match ConventionalCommit::parse(&args.msg, &config) {
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
            term.write_line(
                style(format!("✗ Invalid commit message: {err}"))
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            exit(1);
        }
    };
}
