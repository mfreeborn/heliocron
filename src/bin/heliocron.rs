use std::process;

use chrono::{Duration, FixedOffset, Local, TimeZone};

use heliocron::{config, enums, errors, report, utils};

fn wait(offset: Duration, report: report::SolarReport, event: enums::Event) {
    let event_time = match event {
        enums::Event::Sunrise => report.sunrise,
        enums::Event::Sunset => report.sunset,
    };

    let sleep_until = event_time + offset;

    let local_time = Local::now();
    let local_time = local_time.with_timezone(&FixedOffset::from_offset(local_time.offset()));

    let duration_to_sleep = sleep_until - local_time;

    utils::sleep(duration_to_sleep, sleep_until);
}

fn run_heliocron() -> Result<(), errors::HeliocronError> {
    let config = config::get_config()?;

    let report = report::SolarReport::new(config.date, config.coordinates);

    match config.subcommand {
        Some(config::Subcommand::Report {}) => println!("{}", report),
        Some(config::Subcommand::Wait { offset, event }) => wait(offset, report, event),
        // will never match None as this is caught earlier by StructOpt
        None => println!("No subcommand provided!"),
    }
    Ok(())
}

fn main() {
    process::exit(match run_heliocron() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
