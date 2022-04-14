## Overview

Tools and extensions for git.

## Commands summary

- `gitt init`: Initialize the current repository.
- `gitt lint`: Checks that the commit message adheres to the conventional commit format.
- `gitt commit`: adds all changes and opens a form to submit a conventional commit.
- `gitt amend`: Amends an existing commit.
- `gitt bump`: Bumps the version based on conventional commits.
- `gitt changelog`: Generates the changelog from git history.
- `gitt release`: Performs a release by generating the changelog, creating a commit, tagging the commit, pushing the tag, and optionally pushing to origin.

## `gitt` commands

### `gitt init`

Initializes the current repository, and creates a `.gitt/config.toml` configuraton file.

### `gitt lint`

Lints the commit message.

### `gitt commit`

Adds all commits, opens a form to submit a conventional commit, and pushes the commit.

### `gitt amend`

Amends the previous commit.

### `gitt bump`

Bumps the version based on the conventional commit.

### `gitt changelog`

Generates the changelog, based on the commit range.

### `gitt release`

Performs a release.

1. Check if the repo has uncommitted changes.
2. Generate the changelog.
3. Bump the package(s) version.
4. Commit the changes.
5. Tag the commit as the next version.
6. Optionally, push the tag and the commit to origin.
