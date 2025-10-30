# Jira Issue Commit Hook

[![Integration Tests](https://github.com/H-Rusch/JiraIssueCommitHook/actions/workflows/integration-tests.yml/badge.svg)](https://github.com/H-Rusch/JiraIssueCommitHook/actions/workflows/integration-tests.yml)

This script can be used as a [Git Hook](https://git-scm.com/docs/githooks) to automatically prefix commit messages with the branches associated Jira issue. 

## Examples

| **Branch Name**            | **Commit Message** | **Result**       | **Description**                                                            |
|----------------------------|--------------------|------------------|----------------------------------------------------------------------------|
| feature/ABC-123-branchname | Message            | ABC-123: Message | Prefix the commit message with the Jira issue key based on the branch name |
| feature/ABC-123/branchname | ABC-234: Message   | ABC-234: Message | Commit message will not be prefixed if a issue key is found                |
| _@1c50bb7f rebase-i 2/3_   | Message            | Message          | No action will be taken when not on a valid branch (i.e. while rebasing)   |
| branchname                 | Message            | Message          | No action will be taken when no Jira issue key is found                    |


## Prerequisites

- **Python 3.x**: Make sure Python 3 is installed on your machine.
- **Git**: You need Git installed and configured.


## Installation

1. **Download the Script**: Place the `commit-msg` script into the `.git/hooks/` directory of your local repository.

   - **Alternatively**, if you want to automatically apply this hook to all newly cloned repositories, you can set up a [Git template directory](https://git-scm.com/docs/git-init#_template_directory) and place the `commit-msg` script in the `hooks` folder of your template. This will ensure the hook is included in all new repositories initialized with `git init`.

2. **Make the Script Executable**:
   ```bash
   chmod +x .git/hooks/commit-msg
   ```

3. **Ensure Correct Python Path**: Verify that the first line of the `commit-msg` script points to a valid Python 3 installation. 

## Usage

Once installed, the hook will automatically run each time you create a commit. The script scans the branch name for a Jira issue key (e.g., ABC-123) and prefixes it to the commit message, if not already present.

## Uninstallation

To uninstall the commit hook:

```bash
rm .git/hooks/commit-msg
```

## Tests

Integration tests are implemented to verify the behavior of this script using [cucumber-rs](https://github.com/cucumber-rs/cucumber). Tests are written inside the `integration-tests` directory.

## License

This project is licensed under the MIT License.