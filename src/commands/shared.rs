//! Shared utilities for commands

use std::{
    env::{current_dir, set_current_dir},
    path::{Path, PathBuf},
    process::exit,
};

use console::{style, Term};
use log::debug;

use crate::config::Config;

/// Sets the current directory from an argument
pub fn set_current_dir_from_arg(cwd_input: &Option<String>) -> PathBuf {
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

    let cwd = if let Some(arg_cwd) = &cwd_input {
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

    cwd
}

/// Loads the configuration
pub fn load_config(repo_path: &Path, use_default_if_uninit: bool) -> Config {
    let term = Term::stderr();

    let config = if Config::is_initialized(repo_path) {
        match Config::load(repo_path) {
            Ok(cfg) => cfg,
            Err(err) => {
                term.write_line(
                    style(format!("✗ Invalid config : {err}"))
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(1);
            }
        }
    } else if use_default_if_uninit {
        // >> use default config
        let default_cfg = Config::default();
        match default_cfg.save(repo_path) {
            Ok(_) => {
                term.write_line(
                    format!(
                        "{} {}",
                        style("✔").green(),
                        style("Generated config file").bold()
                    )
                    .as_str(),
                )
                .unwrap();
                default_cfg
            }
            Err(err) => {
                term.write_line(
                    style(format!("✗ Failed to create config file: {}", err))
                        .red()
                        .bold()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(1);
            }
        }
    } else {
        term.write_line(
            style("✗ Missing repo config".to_string())
                .red()
                .to_string()
                .as_str(),
        )
        .unwrap();
        exit(1);
    };

    config
}
