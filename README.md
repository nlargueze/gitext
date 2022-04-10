## Overview

Tools for git

## API summary

- `gitt init`: Initialize the current repository.
- `gitt commit`: adds all changes and opens a form to submit conventional commits.
- `gitt lint`: Checks that the commit message adheres to the conventional commit format.
- `gitt changelog`: Generates the changelog from git history.
- `gitt bump`: Bumps the version based on conventional commits.
- `gitt release`: Performs a release by generating the changelog, creating a commit, tagging the commit, pushing the tag, and optionally pushing to origin.

## `gitt` commands

### `gitt init`

Initializes the current repository, and creates a `gitt.toml` configuraton file.

### `gitt commit`

Adds all commits, opens a form to submit conventional commits, and pushes the commit.

Options:

- `--no-push`: do not push the commit.

### `gitt lint`

Lints the commit message.

### `gitt changelog`

Generates the changelog, based on the commit range.

### `gitt bump`

Bumps the version based on the conventional commit.

### `gitt release`

Performs a release by generating the changelog, creating a commit, tagging the commit, pushing the tag, and optionally pushing to origin.

1. Commits the current changes (if any).
2. Generates the changelog.
3. Creates a new commit with the changelog.
4. Tags the commit.
5. Optionally, pushes the commit/tag to the remote.
6. Optionally, push to another branch.