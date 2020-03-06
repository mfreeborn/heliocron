use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_plain_bin() {
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.env("exit", "1").assert().code(1);
}

#[test]
fn test_report_default_args() {
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let report = cmd.arg("report").assert();

    report
        .success()
        .stdout(predicates::str::contains("LOCATION"))
        .stdout(predicates::str::contains("DATE"))
        .stdout(predicates::str::contains("Sunrise"))
        .stdout(predicates::str::contains("Solar noon"))
        .stdout(predicates::str::contains("Sunset"));
}
