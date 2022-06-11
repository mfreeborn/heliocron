use std::process::Command;

use assert_cmd::assert::Assert;
use assert_cmd::prelude::*;

#[test]
fn test_plain_bin() {
    // assert that running the binary with no flags doesn't simply fail
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    cmd.assert().code(1);
}

#[test]
fn test_help() {
    // assert that a useful help message is given with the --help flag
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let help = cmd.arg("--help").assert();

    help.success()
        .stdout(predicates::str::contains("heliocron"))
        .stdout(predicates::str::contains("A simple utility for"))
        .stdout(predicates::str::contains("USAGE"))
        .stdout(predicates::str::contains("FLAGS"))
        .stdout(predicates::str::contains("OPTIONS"));
}

#[test]
fn test_report_default_args() {
    // assert that a report is successfully generated when no options are set
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let report = cmd.arg("report").assert();

    assert_report(report);
}

#[test]
fn test_report_custom_location() {
    // assert that a report is successfully generated when an arbitrary location is given
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let report_long = cmd
        .args(&["--latitude", "51.0N", "--longitude", "4.36E", "report"])
        .assert();

    assert_report(report_long);

    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let report_short = cmd.args(&["-l", "51.0N", "-o", "4.36E", "report"]).assert();

    assert_report(report_short)
}

#[test]
fn test_report_custom_date() {
    // assert that a report is successfully generated when an arbitrary date is given
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let report_long = cmd
        .args(&[
            "--date",
            "2020-03-15",
            "--date-format",
            "%Y-%m-%d",
            "report",
        ])
        .assert();

    assert_report(report_long);

    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let report_short = cmd
        .args(&["-d", "2020-25-01", "-f", "%Y-%d-%m", "report"])
        .assert();

    assert_report(report_short);
}

#[test]
fn test_report_custom_timezone() {
    // assert that a report is successfully generated when an arbitrary time zone is given
    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let report_long = cmd.args(&["--time-zone", "+01:00", "report"]).assert();

    assert_report(report_long);

    let mut cmd = Command::cargo_bin("heliocron").unwrap();
    let report_short = cmd.args(&["-t", "-05:00", "report"]).assert();

    assert_report(report_short);
}

#[test]
fn test_report_json_output() {
    let mut cmd = Command::cargo_bin("heliocron").unwrap();

    // parse the output into a Json Value
    let json_output: serde_json::Value = serde_json::from_str(
        &String::from_utf8(
            cmd.args(&[
                "--date",
                "2022-06-11",
                "--time-zone",
                "+01:00",
                "--latitude",
                "51.4N",
                "--longitude",
                "5.4670W",
                "report",
                "--json",
            ])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone(),
        )
        .unwrap(),
    )
    .unwrap();

    let expected = serde_json::json!({
        "location": {"latitude": 51.4, "longitude": -5.467},
        "date": "2022-06-11T12:00:00+01:00",
        "day_length": 59534,
        "solar_noon": "2022-06-11T13:21:31+01:00",
        "sunrise": "2022-06-11T05:05:24+01:00",
        "sunset": "2022-06-11T21:37:38+01:00",
        "dawn": {"civil": "2022-06-11T04:18:29+01:00", "nautical": "2022-06-11T03:06:40+01:00", "astronomical": null},
        "dusk": {"civil": "2022-06-11T22:24:34+01:00", "nautical": "2022-06-11T23:36:23+01:00", "astronomical": null},
    });

    assert_eq!(json_output, expected);
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
