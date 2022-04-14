//! release command

use std::{
    env::{current_dir, set_current_dir},
    fs,
    process::exit,
};

use clap::Parser;
use console::{style, Term};
use log::debug;

use crate::{
    changelog::ChangeLog,
    config::Config,
    git::{git_commit, git_push, git_set_tag, git_status_porcelain},
    version::{bump_repo_version, exec_bump_commands},
};

/// release command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// If set, commits and tags are pushed
    #[clap(long, short)]
    pub push: bool,
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

    // 1. check for uncommitted changes
    match git_status_porcelain() {
        Ok(status) => {
            if let Some(files_list) = status {
                term.write_line(
                    style("✗ Repo has uncommited changes:")
                        .red()
                        .bold()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                term.write_line(style(&files_list).red().to_string().as_str())
                    .unwrap();
                exit(1);
            }
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // 2. generate the changelog
    let changelog = match ChangeLog::init() {
        Ok(cl) => cl,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    let next_version = match bump_repo_version(&config) {
        Ok((v, _)) => v.to_string(),
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    let changelog_str = match changelog.generate(&config, &next_version) {
        Ok(s) => s,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    match fs::write(cwd.join("CHANGELOG.md"), changelog_str) {
        Ok(_) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style("Generated changelog").bold()
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

    // 3. Bump the package versions
    match exec_bump_commands(&config, &next_version) {
        Ok(exec_commands) => {
            for cmd in exec_commands {
                term.write_line(
                    format!(
                        "{} {}",
                        style("✔").green(),
                        style(format!("Executed bump command: {cmd} ")).bold()
                    )
                    .as_str(),
                )
                .unwrap();
            }
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    // 4. Commit the changes
    let commit_msg = format!("chore(release): created release {}", next_version);
    match git_commit(&commit_msg) {
        Ok(_) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style(format!("Created commit for release {next_version} ")).bold()
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

    // 5. Tag the commit
    let tag_msg = format!("Release {next_version}");
    match git_set_tag(&next_version, &tag_msg) {
        Ok(_) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style(format!("Tagged commit as {next_version} ")).bold()
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

    // 6.push the commit and tag
    if args.push {
        match git_push() {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
