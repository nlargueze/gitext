//! Install custom hooks

use std::{
    env::{current_dir, set_current_dir},
    fs,
    os::unix::prelude::PermissionsExt,
    process::exit,
};

use clap::Parser;
use console::{style, Term};
use log::debug;

use crate::{config::Config, git::get_config_install_hooks, hooks::create_git_hooks_scripts};

/// install-hooks command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
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

    // create the hooks dir
    let hooks_dir = config.hooks_folder(&cwd);
    if hooks_dir.exists() {
        match fs::remove_dir_all(&hooks_dir) {
            Ok(_) => {
                debug!("Removed existing hooks dir: {}", hooks_dir.display());
            }
            Err(err) => {
                term.write_line(
                    style(format!("✗ Failed to delete hooks dir : {err}"))
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(1);
            }
        }
    }
    match fs::create_dir(&hooks_dir) {
        Ok(_) => {
            debug!("Created hooks dir: {}", hooks_dir.display());
        }
        Err(err) => {
            term.write_line(
                style(format!("✗ Failed to create hooks dir : {err}"))
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            exit(1);
        }
    }

    // create the hooks scripts
    let scripts = match create_git_hooks_scripts(&config) {
        Ok(s) => s,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // generate the files
    for (key, script) in scripts {
        let mut script_file = hooks_dir.join(&key);
        script_file.set_extension("sh");
        match fs::write(&script_file, script) {
            Ok(_) => {
                fs::set_permissions(&script_file, fs::Permissions::from_mode(0o755)).unwrap();
                term.write_line(
                    format!(
                        "{} {}",
                        style("✔").green(),
                        style(format!("Generated hook script for {key}")).bold()
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
    }

    // add to git config
    match get_config_install_hooks(&hooks_dir) {
        Ok(_) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style("Added .gitt/hooks folder to git config core.hooksPath").bold()
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
}
