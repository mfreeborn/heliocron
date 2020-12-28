use std::process::Command;

use assert_cmd::prelude::*;

// run these tests with `cargo test --test test_wait --features integration-test` in order to
// override the default sleep function

#[test]
fn test_wait_non_custom_events() {
    // assert that the following events are all supported (excluding custom events)
    let non_custom_events = &[
        "sunrise",
        "sunset",
        "civil_dawn",
        "civil_dusk",
        "nautical_dawn",
        "nautical_dusk",
        "astronomical_dawn",
        "astronomical_dusk",
        "solar_noon",
    ];

    for event in non_custom_events.iter() {
        let mut cmd = Command::cargo_bin("heliocron").unwrap();
        cmd.args(&["-d", "2099-12-30", "wait", "--event", event])
            .assert()
            .success()
            .stdout(predicates::str::contains("going to sleep for"));
    }
}

#[test]
fn test_wait_custom_events() {
    // assert that the following custome events are supported
    let custom_events = &["custom_am", "custom_pm"];

    for event in custom_events.iter() {
        let mut cmd = Command::cargo_bin("heliocron").unwrap();
        cmd.args(&[
            "-d",
            "2099-12-30",
            "wait",
            "--event",
            event,
            "--altitude",
            "8.0",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("going to sleep for"));
    }
}

#[test]
fn test_wait_requires_an_event() {
    // assert that the command fails when no event is provided
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.args(&["wait"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("required"))
        .stderr(predicates::str::contains("--event"));
}

#[test]
fn test_wait_custom_altitude_positive_or_negative() {
    // assert that --altitude can be positve or negative
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.args(&["-d", "2099-12-30", "wait", "-e", "custom_am", "-a", "-6.3"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.args(&["-d", "2099-12-30", "wait", "-e", "custom_am", "-a", "6.3"])
        .assert()
        .success();
}

#[test]
fn test_wait_custom_altitude_range() {
    // assert that --altitude must be <= 90 and >= -90
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.args(&["-d", "2099-12-30", "wait", "-e", "custom_am", "-a", "-90.0"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "The chosen event does not occur on this day",
        ));

    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.args(&["-d", "2099-12-30", "wait", "-e", "custom_am", "-a", "-90.1"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "Must be a number which is <= 90.0 and >= -90.0",
        ));

    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.args(&["-d", "2099-12-30", "wait", "-e", "custom_am", "-a", "90.0"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "The chosen event does not occur on this day",
        ));

    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.args(&["-d", "2099-12-30", "wait", "-e", "custom_am", "-a", "90.1"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "Must be a number which is <= 90.0 and >= -90.0",
        ));
}

#[test]
fn test_custom_altitude_must_be_float() {
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.args(&[
        "-d",
        "2099-12-30",
        "wait",
        "-e",
        "custom_am",
        "-a",
        "not-a-float",
    ])
    .assert()
    .failure()
    .stderr(predicates::str::contains(
        "Must be a number which is <= 90.0 and >= -90.0",
    ));
}

#[test]
fn test_altitude_ignored_for_non_custom_events() {
    // assert that --altitude is ignored for non-custom events
    let mut cmd = Command::cargo_bin("heliocron").unwrap();

    cmd.args(&[
        "-l",
        "51.4769N",
        "-o",
        "0.0005W",
        "--date",
        "2099-12-30",
        "wait",
        "--event",
        "sunrise",
        "-a",
        "5",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("going to sleep for"))
    .stdout(predicates::str::contains("until 2099-12-30 08:05:21"));
}

#[test]
fn test_custom_am_event_correctness() {
    // assert that --altitude is ignored for non-custom events
    let mut cmd = Command::cargo_bin("heliocron").unwrap();

    cmd.args(&[
        "-l",
        "51.4769N",
        "-o",
        "0.0005W",
        "--date",
        "2099-12-30",
        "wait",
        "--event",
        "custom_am",
        "-a",
        "8.5",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("going to sleep for"))
    .stdout(predicates::str::contains("until 2099-12-30 07:07:05"));
}

#[test]
fn test_custom_pm_event_correctness() {
    // assert that --altitude is ignored for non-custom events
    let mut cmd = Command::cargo_bin("heliocron").unwrap();

    cmd.args(&[
        "-l",
        "51.4769N",
        "-o",
        "0.0005W",
        "--date",
        "2099-12-30",
        "wait",
        "--event",
        "custom_pm",
        "-a",
        "8.5",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("going to sleep for"))
    .stdout(predicates::str::contains("until 2099-12-30 16:57:51"));
}

#[test]
fn test_wait_custom_event_requires_custom_altitude() {
    // assert that --altitude is a required argument when using custom events
    let mut cmd = Command::cargo_bin("heliocron").unwrap();

    let wait = cmd.args(&["wait", "--event", "custom_pm"]).assert();

    wait.failure()
        .stderr(predicates::str::contains(
            "The following required arguments were not provided:",
        ))
        .stderr(predicates::str::contains("--altitude"));
}

#[test]
fn test_wait_errors_with_event_non_occurrence() {
    // assert that the correct error is displayed when a given event doees not occur
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.args(&[
        "-d",
        "2099-06-21",
        "-t",
        "+00:00",
        "wait",
        "--event",
        "astronomical_dusk",
    ])
    .assert()
    .failure()
    .stderr(predicates::str::contains(
        "The chosen event does not occur on this day",
    ));
}

#[test]
fn test_wait_no_offset() {
    // assert that the heliocron will put the thread to sleep
    let mut cmd = Command::cargo_bin("heliocron").unwrap();

    let wait_long = cmd
        .args(&[
            "-d",
            "2099-12-31",
            "-t",
            "+00:00",
            "wait",
            "--event",
            "sunset",
        ])
        .assert();

    wait_long
        .success()
        .stdout(predicates::str::contains("going to sleep for"))
        .stdout(predicates::str::contains("2099-12-31 16:00:33 +00:00"));

    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let wait_short = cmd
        .args(&["-d", "2091-10-05", "-t", "+00:00", "wait", "-e", "sunrise"])
        .assert();

    wait_short
        .success()
        .stdout(predicates::str::contains("going to sleep for"))
        .stdout(predicates::str::contains("2091-10-05 06:07:54 +00:00"));
}

#[test]
fn test_wait_with_offset() {
    // assert that the heliocron will put the thread to sleep
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let wait_long = cmd
        .args(&[
            "-d",
            "2099-12-31",
            "-t",
            "+00:00",
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
            "-t",
            "+00:00",
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
        .stdout(predicates::str::contains("2091-10-04 17:37:02 +00:00"));
}

#[test]
fn test_offset_parse_error() {
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.args(&[
        "-d",
        "2091-10-05",
        "-t",
        "+00:00",
        "wait",
        "-e",
        "sunrise",
        "-o",
        "-12:30:520",
    ])
    .assert()
    .failure()
    .stderr(predicates::str::contains("in the format HH:MM:SS or HH:MM"));
}

#[test]
fn test_tag_is_allowed() {
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let wait_short = cmd
        .args(&[
            "--tag",
            "description of this process",
            "-d",
            "2091-10-05",
            "-t",
            "+00:00",
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
        .stdout(predicates::str::contains("2091-10-04 17:37:02 +00:00"));
}
