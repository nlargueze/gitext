## Overview

Git extensions and extra tooling for repo management.

## Commands

### `git-install-hooks`

Installs custom git hooks.

### `git-lint`

Lints a commit message.

### `git-c`

Adds all commits, opens a form to submit a conventional commit, and pushes the commit.

### `git-amend`

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
