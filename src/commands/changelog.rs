//! changelog command

use std::{
    env::{current_dir, set_current_dir},
    fs,
    process::exit,
};

use clap::Parser;
use console::{style, Term};

use log::debug;

use crate::{changelog::ChangeLog, config::Config};

/// changelog command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// If set, the changelog will be generated in the current directory
    #[clap(long, short)]
    pub output: Option<String>,
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

    // init the changelog
    let changelog = match ChangeLog::init(&config) {
        Ok(cl) => cl,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // generate the change log file
    let changelog_str = match changelog.generate() {
        Ok(s) => s,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // save changelog to file if output argument is set
    if let Some(o) = &args.output {
        match fs::write(cwd.join(o), &changelog_str) {
            Ok(_) => {}
            Err(err) => {
                term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                    .unwrap();
                exit(1);
            }
        }
    }

    print!("{changelog_str}");
}
