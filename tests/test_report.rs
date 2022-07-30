use std::process::Command;

use assert_cmd::assert::Assert;
use assert_cmd::prelude::*;
use pretty_assertions::assert_eq;

fn find_runner() -> Option<String> {
    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_TARGET_") && key.ends_with("_RUNNER") && !value.is_empty() {
            return Some(value);
        }
    }
    None
}

fn get_base_command() -> Command {
    let mut cmd;
    let path = assert_cmd::cargo::cargo_bin("heliocron");
    if let Some(runner) = find_runner() {
        let mut runner = runner.split_whitespace();
        cmd = Command::new(runner.next().unwrap());
        for arg in runner {
            cmd.arg(arg);
        }
        cmd.arg(path);
    } else {
        cmd = Command::new(path);
    }
    cmd
}

#[test]
fn test_plain_bin() {
    // assert that running the binary with no flags doesn't simply fail
    let mut cmd = get_base_command();
    cmd.assert().code(2);
}

#[test]
fn test_help() {
    // assert that a useful help message is given with the --help flag
    let mut cmd = get_base_command();
    let help = cmd.arg("--help").assert();

    help.success()
        .stdout(predicates::str::contains("heliocron"))
        .stdout(predicates::str::contains("A utility"))
        .stdout(predicates::str::contains("USAGE"))
        .stdout(predicates::str::contains("OPTIONS"));
}

#[test]
fn test_report_default_args() {
    // assert that a report is successfully generated when no options are set
    let mut cmd = get_base_command();
    let report = cmd.arg("report").assert();

    assert_report(report);
}

#[test]
fn test_report_custom_location() {
    // assert that a report is successfully generated when an arbitrary location is given
    let mut cmd = get_base_command();
    let report_long = cmd
        .args(&["--latitude", "51.0", "--longitude", "4.36", "report"])
        .assert();

    assert_report(report_long);

    let mut cmd = get_base_command();
    let report_short = cmd.args(&["-l", "51.0", "-o", "4.36", "report"]).assert();

    assert_report(report_short)
}

#[test]
fn test_report_custom_timezone() {
    // assert that a report is successfully generated when an arbitrary time zone is given
    let mut cmd = get_base_command();
    let report_long = cmd
        .args(&[
            "--latitude",
            "51.0",
            "--longitude",
            "4.36",
            "--time-zone",
            "+01:00",
            "report",
        ])
        .assert();

    assert_report(report_long);

    let mut cmd = get_base_command();
    let report_short = cmd
        .args(&[
            "--latitude",
            "51.0",
            "--longitude",
            "4.36",
            "-t",
            "-05:00",
            "report",
        ])
        .assert();

    assert_report(report_short);
}

#[test]
fn test_report_json_output() {
    let mut cmd = get_base_command();

    // parse the output into a Json Value
    let json: serde_json::Value = serde_json::from_slice(
        &cmd.args(&[
            "--date",
            "2022-06-11",
            "--time-zone",
            "+01:00",
            "--latitude",
            "51.4",
            "--longitude",
            "-5.4670",
            "report",
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone(),
    )
    .unwrap();

    let expected = serde_json::json!({
        "location": {"latitude": 51.4000, "longitude": -5.4670},
        "date": "2022-06-11T12:00:00+01:00",
        "day_length": 59534,
        "solar_noon": "2022-06-11T13:21:31+01:00",
        "sunrise": "2022-06-11T05:05:24+01:00",
        "sunset": "2022-06-11T21:37:38+01:00",
        "dawn": {"civil": "2022-06-11T04:18:29+01:00", "nautical": "2022-06-11T03:06:40+01:00", "astronomical": null},
        "dusk": {"civil": "2022-06-11T22:24:34+01:00", "nautical": "2022-06-11T23:36:23+01:00", "astronomical": null},
    });

    assert_eq!(json, expected);
}

#[test]
fn test_correct_output_small_offset() {
    let output = get_base_command()
        .args(&[
            "--date",
            "2022-07-29",
            "--time-zone",
            "+01:00",
            "--latitude",
            "56.8197",
            "--longitude",
            "-5.1047",
            "report",
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();

    let expected = serde_json::json!({
        "location": {"latitude": 56.8197, "longitude": -5.1047},
        "date": "2022-07-29T12:00:00+01:00",
        "day_length": 59066,
        "solar_noon": "2022-07-29T13:26:55+01:00",
        "sunrise": "2022-07-29T05:14:42+01:00",
        "sunset": "2022-07-29T21:39:08+01:00",
        "dawn": {"civil": "2022-07-29T04:23:01+01:00", "nautical": "2022-07-29T03:00:07+01:00", "astronomical": null},
        "dusk": {"civil": "2022-07-29T22:30:48+01:00", "nautical": "2022-07-29T23:53:43+01:00", "astronomical": null},
    });

    assert_eq!(json, expected);
}

#[test]
fn test_correct_output_large_pos_offset() {
    let output = get_base_command()
        .args(&[
            "--date",
            "2022-07-29",
            "--time-zone",
            "+11:00",
            "--latitude",
            "-37.0321",
            "--longitude",
            "175.1220",
            "report",
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();

    let expected = serde_json::json!({
        "location": {"latitude": -37.0321, "longitude": 175.122},
        "date": "2022-07-29T12:00:00+11:00",
        "day_length": 36606,
        "solar_noon": "2022-07-29T11:26:01+11:00",
        "sunrise": "2022-07-29T06:20:58+11:00",
        "sunset": "2022-07-29T16:31:04+11:00",
        "dawn": {"civil": "2022-07-29T05:53:13+11:00", "nautical": "2022-07-29T05:21:48+11:00", "astronomical": "2022-07-29T04:51:00+11:00"},
        "dusk": {"civil": "2022-07-29T16:58:49+11:00", "nautical": "2022-07-29T17:30:14+11:00", "astronomical": "2022-07-29T18:01:02+11:00"},
    });

    assert_eq!(json, expected);
}

fn assert_report(report: Assert) {
    report
        .success()
        .stdout(predicates::str::contains("LOCATION"))
        .stdout(predicates::str::contains("DATE"))
        .stdout(predicates::str::contains("Sunrise"))
        .stdout(predicates::str::contains("Solar noon"))
        .stdout(predicates::str::contains("Sunset"));
}
