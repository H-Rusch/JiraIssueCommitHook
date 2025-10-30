Feature: Specifiying a custom format

    Background:

        Given initialized git repo with included commit-msg script
        And setting the custom commit message format as "[{{key}}] {{message}}"

    Scenario: Commit message is written in custom format

        Given the repo is on branch feature/ABC-123-branch
        Then the current branch is feature/ABC-123-branch
        When committing with message "Message"
        Then the commit-message is "[ABC-123] Message"

    Scenario: Detecting if commit is already tagged works with custom format

        Given the repo is on branch feature/ABC-123-branch
        Then the current branch is feature/ABC-123-branch
        When committing with message "[DEF-456] Message"
        Then the commit-message is "[DEF-456] Message"
