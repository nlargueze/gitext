//! gitt executable

use clap::{Parser, Subcommand};

use gitext::commands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bump a version
    Bump(commands::bump::Args),
    /// Creates a changelog
    Changelog(commands::changelog::Args),
    /// Creates a release
    Release(commands::release::Args),
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Bump(args) => {
            commands::bump::run(args);
        }
        Commands::Changelog(args) => {
            commands::changelog::run(args);
        }
        Commands::Release(args) => {
            commands::release::run(args);
        }
    }
}
