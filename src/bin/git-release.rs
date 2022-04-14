//! Performs a release.

use std::{fs, process::exit};

use clap::Parser;

use console::{style, Term};
use gitext::{
    changelog::ChangeLog,
    commands::shared::{load_config, set_current_dir_from_arg},
    git::{git_add, git_commit, git_push_follow_tags, git_set_tag, git_status_porcelain},
    version::{bump_repo_version, exec_bump_commands},
};

/// Release command
#[derive(Debug, Parser)]
#[clap(author, version, about = "Performs a release")]
pub struct Cli {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// If set, commits and tags are pushed
    #[clap(long, short)]
    pub push: bool,
}

fn main() {
    env_logger::init();

    let term = Term::stderr();

    let args = Cli::parse();

    // set CWD
    let cwd = set_current_dir_from_arg(&args.cwd);

    // load the config
    let config = load_config(&cwd, true);

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
    match git_add() {
        Ok(_) => {
            term.write_line(
                format!("{} {}", style("✔").green(), style("Changed staged").bold()).as_str(),
            )
            .unwrap();
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

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
        match git_push_follow_tags() {
            Ok(_) => {
                term.write_line(
                    format!(
                        "{} {}",
                        style("✔").green(),
                        style("Pushed commit and tag").bold()
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
}
