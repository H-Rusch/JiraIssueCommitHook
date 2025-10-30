use std::{fs::copy, path::Path, process::Output};

use assert_cmd::Command;
use assert_fs::TempDir;
use cucumber::{given, then, when, World};

#[derive(Debug, World)]
#[world(init = Self::new)]
struct TestWorld {
    directory: TempDir,
}

impl TestWorld {
    fn new() -> Self {
        Self {
            directory: TempDir::new().unwrap(),
        }
    }

    fn init_git_repo(&self) {
        let repo_path = self.directory.path();

        run("git", &["init"], repo_path);
        run(
            "git",
            &["commit", "--allow-empty", "-m", "Root commit"],
            repo_path,
        );

        run(
            "git",
            &["config", "user.email", "test@example.com"],
            repo_path,
        );
        run("git", &["config", "user.name", "Test User"], repo_path);

        let source_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("commit-msg");
        let hook_path = repo_path.join(".git/hooks/commit-msg");

        copy(&source_path, &hook_path).expect("failed to copy commit-msg hook");

        run("chmod", &["+x", hook_path.to_str().unwrap()], repo_path);
    }
}

#[given("initialized git repo with included commit-msg script")]
async fn initialized_git_repo(world: &mut TestWorld) {
    world.init_git_repo();
}

#[given(expr = "the repo is on branch {word}")]
async fn switch_to_branch(world: &mut TestWorld, branch: String) {
    switch_to_new_branch(branch, &world.directory);
}

#[then(expr = "the current branch is {word}")]
async fn current_branch_is(world: &mut TestWorld, expected_branch: String) {
    let branch = get_cmd_stdout("git", &["branch", "--show"], &world.directory);

    assert_eq!(expected_branch, branch, "Current branch is not expected")
}

#[when(regex = "^committing with message \"(.+)\"$")]
async fn committing_with_message(world: &mut TestWorld, message: String) {
    run(
        "git",
        &["commit", "--allow-empty", "-m", &message],
        &world.directory,
    );
}

#[then(regex = "^the commit-message is \"(.+)\"$")]
async fn assert_message_written(world: &mut TestWorld, expected_message: String) {
    let message = get_cmd_stdout("git", &["log", "-1", "--pretty=%B"], &world.directory);

    assert_eq!(
        expected_message, message,
        "Actual message does not match expected"
    )
}

#[given(expr = "a branch {word} with commits exists")]
async fn branch_with_commits_exists(world: &mut TestWorld, branch: String) {
    switch_to_new_branch(branch, &world.directory);
    committing_with_message(world, "Some message".to_string()).await;
    run("git", &["switch", "-"], &world.directory);
}

#[then(expr = "rebasing current branch onto {word} works without error")]
async fn rebasing_current_onto(world: &mut TestWorld, branch: String) {
    run("git", &["rebase", &branch], &world.directory);
}

fn switch_to_new_branch(branch: String, path: &Path) {
    run("git", &["switch", "-c", &branch], path);
}

/// Run a command that is expected to finish successfully.
fn run(cmd: &str, args: &[&str], dir: &Path) -> Output {
    let output = Command::new(cmd).args(args).current_dir(dir).unwrap();
    assert!(output.status.success(), "{cmd} failed in {:?}", dir);
    output
}

/// Run a command that is expected to finish successfully and extract its output text.
fn get_cmd_stdout(cmd: &str, args: &[&str], dir: &Path) -> String {
    let output = run(cmd, args, dir);

    let sdtout =
        String::from_utf8(output.stdout).expect("Could not convert command output to string");
    String::from(sdtout.trim())
}

#[tokio::main]
async fn main() {
    TestWorld::cucumber().run_and_exit("tests/features").await;
}
