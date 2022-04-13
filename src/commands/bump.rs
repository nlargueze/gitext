//! Changelog command

use std::{
    env::{current_dir, set_current_dir},
    process::exit,
};

use clap::Parser;
use console::{style, Term};

use crate::{
    config::Config,
    git::{git_log, git_set_tag},
    version::increment_repo_version,
};

/// bump command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// If set, the next version
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

    // get all commits
    let commits = match git_log() {
        Ok(commits) => commits,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // // get the next version
    // let next_version = match increment_repo_version(&commits) {
    //     Ok(res) => res,
    //     Err(err) => {
    //         term.write_line(
    //             style(format!("✗ Failed to get last version: {err}"))
    //                 .red()
    //                 .to_string()
    //                 .as_str(),
    //         )
    //         .unwrap();
    //         exit(1);
    //     }
    // };

    // if args.dry_run {
    //     println!("{}", next_version);
    //     exit(0);
    // }

    // // Tag the repo with the new version
    // let next_tag_str = next_version.to_string();
    // match git_set_tag(&next_version.to_string().as_str()) {
    //     Ok(_) => {
    //         term.write_line(
    //             format!(
    //                 "{} {}",
    //                 style("✔").green(),
    //                 style(format!("Tagged as {next_tag_str}")).bold()
    //             )
    //             .as_str(),
    //         )
    //         .unwrap();
    //     }
    //     Err(err) => {
    //         term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
    //             .unwrap();
    //         exit(1);
    //     }
    // }
}
