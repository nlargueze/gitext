## Overview

Tools for git

## API

- ` init`: Initialize a git repository
- `git commit`: adds all commits and opens a form to submit commits
- `git commit --push`: git commit with push
- `git lint`: Checks that commit message adheres to the conventional commit format.
- `git clog`: Generates a changelog from git history.
- `git clog --tests`: Displays the changelog on test mode.
- `git bump`: Bumps the conventional commit.
- `git release`:
  1. Commits the current changes (if any).
  2. Generates the changelog.
  3. Creates a new commit with the changelog.
  4. Pushes the commit to the remote.
  5. Optionally, push to another branch.

```

```
