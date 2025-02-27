use cargo_test_support::compare::assert_ui;
use cargo_test_support::current_dir;
use cargo_test_support::file;
use cargo_test_support::prelude::*;
use cargo_test_support::str;
use cargo_test_support::Project;

#[cargo_test]
fn case() {
    let project = Project::from_template(current_dir!().join("in"));
    let project_root = project.root();
    let cwd = &project_root;

    snapbox::cmd::Command::cargo_ui()
        .arg("add")
        .arg_line("+nightly")
        .current_dir(cwd)
        .assert()
        .failure()
        .stdout_matches(str![""])
        .stderr_matches(file!["stderr.log"]);

    assert_ui().subset_matches(current_dir!().join("out"), &project_root);
}
