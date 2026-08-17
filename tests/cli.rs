// End to end tests: they run the built binary the way git actually invokes a
// commit-msg hook, against a real git repository, and check what lands in the
// commit message file - the one thing the unit tests never touch.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Repo {
    _directory: tempfile::TempDir,
    dir: PathBuf,
}

impl Repo {
    // A single initial commit, so that HEAD exists and branches can be cut
    // from it the way they would be in a real checkout.
    fn new() -> Repo {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let dir = directory.path().to_path_buf();
        run_ok(&dir, "git", &["init", "-q", "-b", "master"]);
        run_ok(&dir, "git", &["config", "user.email", "test@test.com"]);
        run_ok(&dir, "git", &["config", "user.name", "test"]);
        fs::write(dir.join("file.txt"), "content").expect("a tracked file");
        run_ok(&dir, "git", &["add", "."]);
        run_ok(&dir, "git", &["commit", "-q", "-m", "init"]);
        Repo {
            _directory: directory,
            dir,
        }
    }

    fn checkout(&self, branch: &str) {
        run_ok(&self.dir, "git", &["checkout", "-q", "-b", branch]);
    }

    fn set_config(&self, key: &str, value: &str) {
        run_ok(&self.dir, "git", &["config", key, value]);
    }

    fn commit_msg_file(&self, name: &str, contents: &str) -> PathBuf {
        let path = self.dir.join(name);
        fs::write(&path, contents).expect("a commit message file");
        path
    }

    fn read(&self, path: &Path) -> String {
        fs::read_to_string(path).expect("the commit message file")
    }

    // The hook never guesses a prefix from whoever runs the tests: any
    // TICKET_PREFIX in the host environment is dropped, and a test that wants
    // one sets it explicitly.
    fn hook(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_ticket-commit-msg"));
        command.current_dir(&self.dir).env_remove("TICKET_PREFIX");
        command
    }
}

fn run_ok(dir: &Path, program: &str, args: &[&str]) {
    let output = Command::new(program)
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap_or_else(|e| panic!("failed to start {program}: {e}"));
    assert!(
        output.status.success(),
        "{program} {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

#[test]
fn a_ticket_number_from_the_branch_is_appended_to_the_commit_message() {
    let repo = Repo::new();
    repo.checkout("QUEUE-123");
    let msg_file = repo.commit_msg_file("COMMIT_EDITMSG", "Test commit");

    let output = repo
        .hook()
        .arg(&msg_file)
        .output()
        .expect("the hook starts");

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!("Test commit\n\nQUEUE-123", repo.read(&msg_file));
}

#[test]
fn a_branch_without_a_ticket_number_leaves_the_commit_message_untouched() {
    let repo = Repo::new();
    repo.checkout("chore/cleanup");
    let msg_file = repo.commit_msg_file("COMMIT_EDITMSG", "Test commit");

    let output = repo
        .hook()
        .arg(&msg_file)
        .output()
        .expect("the hook starts");

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!("Test commit", repo.read(&msg_file));
}

#[test]
fn a_git_config_prefix_is_applied() {
    let repo = Repo::new();
    repo.checkout("QUEUE-123");
    repo.set_config("custom.ticketnumberprefix", "JIRA: ");
    let msg_file = repo.commit_msg_file("COMMIT_EDITMSG", "Test commit");

    let output = repo
        .hook()
        .arg(&msg_file)
        .output()
        .expect("the hook starts");

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!("Test commit\n\nJIRA: QUEUE-123", repo.read(&msg_file));
}

#[test]
fn an_env_var_prefix_is_applied_when_no_git_config_is_set() {
    let repo = Repo::new();
    repo.checkout("QUEUE-123");
    let msg_file = repo.commit_msg_file("COMMIT_EDITMSG", "Test commit");

    let output = repo
        .hook()
        .arg(&msg_file)
        .env("TICKET_PREFIX", "Issue: ")
        .output()
        .expect("the hook starts");

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!("Test commit\n\nIssue: QUEUE-123", repo.read(&msg_file));
}

#[test]
fn git_config_prefix_wins_over_the_env_var() {
    let repo = Repo::new();
    repo.checkout("QUEUE-123");
    repo.set_config("custom.ticketnumberprefix", "JIRA: ");
    let msg_file = repo.commit_msg_file("COMMIT_EDITMSG", "Test commit");

    let output = repo
        .hook()
        .arg(&msg_file)
        .env("TICKET_PREFIX", "Issue: ")
        .output()
        .expect("the hook starts");

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!("Test commit\n\nJIRA: QUEUE-123", repo.read(&msg_file));
}

#[test]
fn an_already_present_ticket_number_is_not_duplicated() {
    let repo = Repo::new();
    repo.checkout("QUEUE-123");
    let msg_file = repo.commit_msg_file("COMMIT_EDITMSG", "Test commit\n\nQUEUE-123");

    let output = repo
        .hook()
        .arg(&msg_file)
        .output()
        .expect("the hook starts");

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!("Test commit\n\nQUEUE-123", repo.read(&msg_file));
}

#[test]
fn help_flag_prints_usage_and_exits_successfully() {
    let repo = Repo::new();

    let output = repo.hook().arg("--help").output().expect("the hook starts");

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("Usage:"), "{}", stdout(&output));
}

#[test]
fn no_arguments_prints_usage_and_exits_successfully() {
    let repo = Repo::new();

    let output = repo.hook().output().expect("the hook starts");

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("Usage:"), "{}", stdout(&output));
}

// Regression test for environment::system_environment: a real commit message
// file that happens to be named exactly '-h' must still be treated as the
// commit message, not mistaken for the help flag. This specifically has to be
// '-h' rather than '--help': the bug it guards against checked for a file
// named '--help' regardless of which flag spelling was actually passed, so a
// '--help' named file passed as '--help' looked fine even while buggy.
#[test]
fn a_commit_message_file_literally_named_h_flag_is_not_mistaken_for_the_flag() {
    let repo = Repo::new();
    repo.checkout("QUEUE-123");
    let msg_file = repo.commit_msg_file("-h", "Test commit");

    let output = repo.hook().arg("-h").output().expect("the hook starts");

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!("Test commit\n\nQUEUE-123", repo.read(&msg_file));
}

#[test]
fn a_missing_commit_message_file_is_reported_as_an_error() {
    let repo = Repo::new();
    repo.checkout("QUEUE-123");

    let output = repo
        .hook()
        .arg("does-not-exist")
        .output()
        .expect("the hook starts");

    assert_eq!(Some(1), output.status.code());
    assert!(
        stderr(&output).contains("Cannot resolve commit message file path"),
        "{}",
        stderr(&output)
    );
}
