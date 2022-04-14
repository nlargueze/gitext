//! changelog command

use std::{
    env::{current_dir, set_current_dir},
    process::exit,
};

use clap::Parser;
use console::{style, Term};

use dialoguer::{theme::ColorfulTheme, Confirm};
use log::debug;

use crate::{
    changelog::ChangeLog, config::Config, git::git_status_porcelain, version::bump_repo_version,
};

/// changelog command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// Allows uncommitted changes when setting the tag
    #[clap(long)]
    pub allow_uncommitted: bool,
}

/// Runs the command
pub fn run(args: &Args) {
    env_logger::init();
    let term = Term::stderr();

    let cwd = match current_dir() {
        Ok(path) => path,
        Err(err) => {
            term.write_line(
                style(format!("✗ Internal error: {err}"))
                    .red()
                    .bold()
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

    // set the current directory
    match set_current_dir(&cwd) {
        Ok(_) => {
            debug!("Current directory set to {}", cwd.display());
        }
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

    // check if the repo is pristine
    if !args.allow_uncommitted {
        let commit_status = match git_status_porcelain() {
            Ok(status) => status,
            Err(err) => {
                term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                    .unwrap();
                exit(1);
            }
        };

        if let Some(files_list) = commit_status {
            term.write_line(
                style("> Repo has uncommited changes:")
                    .bold()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            term.write_line(&files_list).unwrap();
            let ok = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Tag despite uncommited changes ?")
                .report(true)
                .default(false)
                .interact()
                .unwrap();
            if !ok {
                term.write_line(
                    style("✗ Uncommited changes -> skipped".to_string())
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(0);
            }
        }
    }

    // init the changelog
    let changelog = match ChangeLog::init() {
        Ok(cl) => cl,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // get the latest version based on the commit history
    // NB: can be replaced by Unreleased tag
    let next_version = match bump_repo_version(&config) {
        Ok((v, _)) => v.to_string(),
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // generate the change log file
    let changelog_str = match changelog.generate(&config, &next_version) {
        Ok(s) => s,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // print changelog
    print!("{changelog_str}");
}
