use cargo_test_support::file;
use cargo_test_support::prelude::*;
use cargo_test_support::str;

#[cargo_test]
fn case() {
    snapbox::cmd::Command::cargo_ui()
        .arg("yank")
        .arg("--help")
        .assert()
        .success()
        .stdout_matches(file!["stdout.log"])
        .stderr_matches(str![""]);
}
