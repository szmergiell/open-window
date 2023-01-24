use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn open_window_test() {
    let mut cmd = Command::cargo_bin("owcli").unwrap();

    let assert = cmd.arg("20").arg("50").arg("0").arg("95").assert();

    assert
        .success()
        .code(0)
        .stdout(predicate::str::contains("Open window!"));
}

#[test]
fn close_window_test() {
    let mut cmd = Command::cargo_bin("owcli").unwrap();

    let assert = cmd.arg("25").arg("50").arg("30").arg("40").assert();

    assert
        .success()
        .code(0)
        .stdout(predicate::str::contains("Close window!"));
}
