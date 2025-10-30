use std::{fs::copy, path::Path, process::Output};

use assert_cmd::Command;
use assert_fs::TempDir;
use cucumber::{given, then, when, World};

const FORMAT_ENV: &str = "COMMIT_MESSAGE_FORMAT";

#[derive(Debug, World)]
#[world(init = Self::new)]
struct TestWorld {
    directory: TempDir,
    custom_format: Option<String>,
}

impl TestWorld {
    fn new() -> Self {
        Self {
            directory: TempDir::new().unwrap(),
            custom_format: None,
        }
    }

    fn init_git_repo(&self) {
        let repo_path = self.directory.path();

        self.run("git", &["init"]);
        self.run("git", &["commit", "--allow-empty", "-m", "Root commit"]);

        self.run("git", &["config", "user.email", "test@example.com"]);
        self.run("git", &["config", "user.name", "Test User"]);

        let source_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("commit-msg");
        let hook_path = repo_path.join(".git/hooks/commit-msg");

        copy(&source_path, &hook_path).expect("failed to copy commit-msg hook");

        self.run("chmod", &["+x", hook_path.to_str().unwrap()]);
    }

    fn run(&self, cmd: &str, args: &[&str]) -> Output {
        let mut binding = Command::new(cmd);
        let command = binding.args(args).env_clear().current_dir(&self.directory);
        if let Some(format_string) = &self.custom_format {
            command.env(FORMAT_ENV, format_string);
        }

        let output = command.unwrap();
        assert!(
            output.status.success(),
            "{cmd} failed in {:?}",
            self.directory
        );
        output
    }
}

#[given("initialized git repo with included commit-msg script")]
async fn initialized_git_repo(world: &mut TestWorld) {
    world.init_git_repo();
}

#[given(expr = "the repo is on branch {word}")]
async fn switch_to_branch(world: &mut TestWorld, branch: String) {
    switch_to_new_branch(world, branch);
}

#[then(expr = "the current branch is {word}")]
async fn current_branch_is(world: &mut TestWorld, expected_branch: String) {
    let branch = get_cmd_stdout(world, "git", &["branch", "--show"]);

    assert_eq!(expected_branch, branch, "Current branch is not expected")
}

#[when(regex = "^committing with message \"(.+)\"$")]
async fn committing_with_message(world: &mut TestWorld, message: String) {
    world.run("git", &["commit", "--allow-empty", "-m", &message]);
}

#[then(regex = "^the commit-message is \"(.+)\"$")]
async fn assert_message_written(world: &mut TestWorld, expected_message: String) {
    let message = get_cmd_stdout(world, "git", &["log", "-1", "--pretty=%B"]);

    assert_eq!(
        expected_message, message,
        "Actual message does not match expected"
    )
}

#[given(expr = "a branch {word} with commits exists")]
async fn branch_with_commits_exists(world: &mut TestWorld, branch: String) {
    switch_to_new_branch(world, branch);
    committing_with_message(world, "Some message".to_string()).await;
    world.run("git", &["switch", "-"]);
}

#[then(expr = "rebasing current branch onto {word} works without error")]
async fn rebasing_current_onto(world: &mut TestWorld, branch: String) {
    world.run("git", &["rebase", &branch]);
}

#[given(regex = "^setting the custom commit message format as \"(.+)\"$")]
async fn environment_variable_is_set(world: &mut TestWorld, custom_format: String) {
    world.custom_format = Some(custom_format);
}

fn switch_to_new_branch(world: &TestWorld, branch: String) {
    world.run("git", &["switch", "-c", &branch]);
}

/// Run a command that is expected to finish successfully and extract its output text.
fn get_cmd_stdout(world: &TestWorld, cmd: &str, args: &[&str]) -> String {
    let output = world.run(cmd, args);

    let sdtout =
        String::from_utf8(output.stdout).expect("Could not convert command output to string");
    String::from(sdtout.trim())
}

#[tokio::main]
async fn main() {
    TestWorld::cucumber().run_and_exit("tests/features").await;
}
