# Jira Issue Commit Hook

This script can be used as a [Git Hook](https://git-scm.com/docs/githooks) to automatically prefix commit messages with the branches associated Jira issue. 

## Examples

| **Branch Name**            | **Commit Message** | **Result**       | **Description**                                                            |
|----------------------------|--------------------|------------------|----------------------------------------------------------------------------|
| feature/ABC-123-branchname | Message            | ABC-123: Message | Prefix the commit message with the Jira issue key based on the branch name |
| feature/ABC-123/branchname | ABC-234: Message   | ABC-234: Message | Commit message will not be prefixed if a issue key is found                |
| _@1c50bb7f rebase-i 2/3_   | Message            | Message          | No action will be taken when not on a valid branch (i.e. while rebasing)   |
| branchname                 | Message            | Message          | No action will be taken when no Jira issue key is found                    |


## Usage
Place the `commit-msg` script into the `.git/hooks/` directory for it to be picked up by Git. 

Make sure the script is executable and the it points to a valid python3 installation.
