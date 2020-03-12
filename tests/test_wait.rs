use std::process::Command;

use assert_cmd::prelude::*;

#[test]
#[ignore]
fn test_wait_no_offset() {
    // assert that the heliocron will put the thread to sleep
    // let _guard = guerrilla::patch2(heliocron::utils::sleep, |_duration, _sleep_until| {
    //     println!("Sleeping!");
    // });
    // let mut cmd = Command::cargo_bin("heliocron").unwrap();
    // let wait_long = cmd
    //     .args(&["-d", "2099-12-31", "wait", "--event", "sunset"])
    //     .assert();

    // wait_long.success();

    // let mut cmd = Command::cargo_bin("heliocron").unwrap();
    // let wait_short = cmd
    //     .args(&["-d", "2099-12-31", "wait", "-e", "sunrise"])
    //     .assert();

    // wait_short.success();
    // drop(_guard)
}

#[test]
#[ignore]
fn test_wait_with_offset() {
    // assert that the heliocron will put the thread to sleep
    // let _guard = guerrilla::patch2(heliocron::utils::sleep, |_duration, _sleep_until| {
    //     println!("Sleeping!");
    // });
    // let mut cmd = Command::cargo_bin("heliocron").unwrap();
    // let wait_long = cmd
    //     .args(&[
    //         "-d",
    //         "2099-12-31",
    //         "wait",
    //         "--event",
    //         "sunset",
    //         "--offset",
    //         "01:00",
    //     ])
    //     .assert();

    // wait_long.success();

    // let mut cmd = Command::cargo_bin("heliocron").unwrap();
    // let wait_short = cmd
    //     .args(&[
    //         "-d",
    //         "2099-12-31",
    //         "wait",
    //         "-e",
    //         "sunrise",
    //         "-o",
    //         "-12:30:52",
    //     ])
    //     .assert();

    // wait_short.success();
    // drop(_guard)
}
