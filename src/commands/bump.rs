//! Changelog command

use std::{
    env::{current_dir, set_current_dir},
    process::exit,
};

use clap::Parser;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm};
use log::debug;

use crate::{
    config::Config,
    git::{git_set_tag, git_status_porcelain},
    version::bump_repo_version,
};

/// bump command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// Allows uncommitted changes when setting the tag
    #[clap(long)]
    pub allow_uncommitted: bool,
    /// If set, the repo is tagged with the new version
    #[clap(long)]
    pub set: bool,
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

    // bump
    let (next_version, curr_version) = match bump_repo_version(&config) {
        Ok(commits) => commits,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };
    term.write_line(
        format!(
            "{} {}",
            style("i").yellow(),
            style(format!(
                "{} --> {}",
                curr_version
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "<none>".to_string()),
                next_version
            ))
            .bold(),
        )
        .as_str(),
    )
    .unwrap();
    let next_version_str = next_version.to_string();

    // dry run
    if !args.set {
        debug!("Dry run: tagging skipped");
        print!("{}", next_version_str);
        exit(0);
    }

    // Tag the repo with the new version

    match git_set_tag(
        &next_version_str,
        format!("Version v{next_version_str}").as_str(),
    ) {
        Ok(_) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style(format!("Repo tagged as {next_version_str}")).bold()
                )
                .as_str(),
            )
            .unwrap();
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    print!("{}", next_version_str);
}
