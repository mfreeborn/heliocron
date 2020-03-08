use chrono::{Duration, FixedOffset, Local, TimeZone};

use heliocron::{config, enums, report, utils};

fn wait(offset: Duration, report: report::SolarReport, event: enums::Event) {
    let event_time = match event {
        enums::Event::Sunrise => report.get_sunrise(),
        enums::Event::Sunset => report.get_sunset(),
    };

    let sleep_until = event_time + offset;

    let local_time = Local::now();
    let local_time = local_time.with_timezone(&FixedOffset::from_offset(local_time.offset()));

    let duration_to_sleep = sleep_until - local_time;

    utils::sleep(duration_to_sleep, sleep_until);
}

fn main() {
    let config = config::get_config();

    // let args = Cli::from_args();

    // let date = parsers::parse_date(
    //     args.date_args.date.as_deref(),
    //     &args.date_args.date_fmt,
    //     args.date_args.time_zone.as_deref(),
    // );

    // let coordinates = structs::Coordinates::from_decimal_degrees(&args.latitude, &args.longitude);

    let report = report::SolarReport::new(config.date, config.coordinates);

    match config.subcommand {
        Some(config::Subcommand::Report {}) => println!("{}", report),
        Some(config::Subcommand::Wait { offset, event }) => wait(offset, report, event),
        // will never match None as this is caught earlier by StructOpt
        None => println!("No subcommand provided!"),
    }
}
