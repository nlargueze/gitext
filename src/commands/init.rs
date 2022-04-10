//! Init command

use std::env::current_dir;

use clap::Parser;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::config::{Configuration, CONFIG_FILE};

/// init command arguments
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
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
            return;
        }
    };

    let cwd = if let Some(arg_cwd) = &args.cwd {
        cwd.join(arg_cwd)
    } else {
        cwd
    };

    let cfg_file = cwd.join(CONFIG_FILE);

    if !Configuration::is_initialized(&cwd) {
        match Configuration::default().to_file(&cfg_file) {
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
                return;
            }
        };
    } else {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Repo already initialized, reset ?")
            .interact()
            .unwrap()
        {
            match Configuration::default().to_file(&cfg_file) {
                Ok(_) => {
                    term.write_line(
                        format!("{} Regenerated config file", style("✓").green()).as_str(),
                    )
                    .unwrap();
                }
                Err(err) => {
                    term.write_line(
                        style(format!("✗ Failed to recreate config file: {}", err))
                            .red()
                            .bold()
                            .to_string()
                            .as_str(),
                    )
                    .unwrap();
                    return;
                }
            };
        } else {
            return;
        }
    }
}
