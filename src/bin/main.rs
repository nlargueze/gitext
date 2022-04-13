//! gitt executable

use clap::{Parser, Subcommand};

use gitt::commands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes the repo
    Init(commands::init::Args),
    /// Commits the current changes
    Commit(commands::commit::Args),
    /// Amends the last commit
    Amend(commands::amend::Args),
    /// Lints a commit message
    Lint(commands::lint::Args),
    /// Bump a version
    Bump(commands::bump::Args),
    /// Creates a changelog
    Changelog(commands::changelog::Args),
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Init(args) => {
            commands::init::run(args);
        }
        Commands::Commit(args) => {
            commands::commit::run(args);
        }
        Commands::Amend(args) => {
            commands::amend::run(args);
        }
        Commands::Lint(args) => {
            commands::lint::run(args);
        }
        Commands::Bump(args) => {
            commands::bump::run(args);
        }
        Commands::Changelog(args) => {
            commands::changelog::run(args);
        }
    }
}
