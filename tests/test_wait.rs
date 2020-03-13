use std::process::Command;

use assert_cmd::prelude::*;

// run these tests with `cargo test --test test_wait --features integration-test` in order to
// override the default sleep function

#[test]
fn test_wait_no_offset() {
    // assert that the heliocron will put the thread to sleep
    let mut cmd = Command::cargo_bin("heliocron").unwrap();

    let wait_long = cmd
        .args(&["-d", "2099-12-31", "wait", "--event", "sunset"])
        .assert();

    wait_long
        .success()
        .stdout(predicates::str::contains("going to sleep for"))
        .stdout(predicates::str::contains("2099-12-31 16:00:33 +00:00"));

    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let wait_short = cmd
        .args(&["-d", "2091-10-05", "wait", "-e", "sunrise"])
        .assert();

    wait_short
        .success()
        .stdout(predicates::str::contains("going to sleep for"))
        .stdout(predicates::str::contains("2091-10-05 07:07:50 +01:00"));
}

#[test]
fn test_wait_with_offset() {
    // assert that the heliocron will put the thread to sleep
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let wait_long = cmd
        .args(&[
            "-d",
            "2099-12-31",
            "wait",
            "--event",
            "sunset",
            "--offset",
            "01:00",
        ])
        .assert();

    wait_long
        .success()
        .stdout(predicates::str::contains("going to sleep for"))
        .stdout(predicates::str::contains("2099-12-31 17:00:33 +00:00"));

    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let wait_short = cmd
        .args(&[
            "-d",
            "2091-10-05",
            "wait",
            "-e",
            "sunrise",
            "-o",
            "-12:30:52",
        ])
        .assert();

    wait_short
        .success()
        .stdout(predicates::str::contains("going to sleep for"))
        .stdout(predicates::str::contains("2091-10-04 18:36:58 +01:00"));
}
