Feature: Running the commit-msg hook

  Background:

    Given initialized git repo with included commit-msg script

  Scenario Outline: Committing on a valid branch prefixes commit message

    Given the repo is on branch <branch>
    Then the current branch is <branch>
    When committing with message <message>
    Then the commit-message is <expected>

    Examples:
      | branch                        | message   | expected            |
      | feature/ABC-123-branch        | "Message" | "ABC-123: Message"  |
      | feature/ABC-123/branch        | "Message" | "ABC-123: Message"  |
      | feature/abc-123-branch        | "Message" | "ABC-123: Message"  |
      | bugfix/GG-12345-branch        | "Message" | "GG-12345: Message" |
      | something-else/ABC-123-branch | "Message" | "ABC-123: Message"  |
      | ABC-123-branch                | "Message" | "ABC-123: Message"  |

  Scenario Outline: Committing on invalid branch does not prefix commit message

    Given the repo is on branch <branch>
    Then the current branch is <branch>
    When committing with message <message>
    Then the commit-message is <message>

    Examples:
      | branch                 | message   |
      | branch-name            | "Message" |
      | feauture/ABC-123branch | "Message" |
      | feauture/ABC123-branch | "Message" |

  Scenario Outline: Manually specifying Jira key overwrites the script from writing it

    Given the repo is on branch feature/ABC-123-branch
    Then the current branch is feature/ABC-123-branch
    When committing with message "DEF-456: Message"
    Then the commit-message is "DEF-456: Message"

    Examples:
      | message                      |
      | "DEF-456: Message"           |
      | "ABC-123 & DEF-456: Message" |
      | "No-Ticket: Message"         |

  Scenario: The script does not prefix when not currently on a branch

    During rebase git is not on a branch. In order to prevent errors the script is able to handle this situation.

    Given the repo is on branch feature/ABC-123-branch
    Then the current branch is feature/ABC-123-branch
    When committing with message "Message"
    Given a branch branch-name with commits exists
    Then rebasing current branch onto branch-name works without error
