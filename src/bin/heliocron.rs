use chrono::{Duration, FixedOffset, Local, TimeZone};
use structopt::clap::AppSettings;
use structopt::StructOpt;

use heliocron::{enums, parsers, report, structs, utils};

#[derive(Debug, StructOpt)]
#[structopt(
    about = "A simple utility for finding out what time sunrise/sunset is, and executing programs relative to these events."
)]
#[structopt(settings = &[AppSettings::AllowLeadingHyphen])]
struct Cli {
    #[structopt(subcommand)]
    sub_cmd: SubCommand,

    #[structopt(flatten)]
    date_args: DateArgs,

    #[structopt(short = "l", long = "latitude", default_value = "51.0782N")]
    latitude: String,

    #[structopt(short = "o", long = "longitude", default_value = "4.0583W")]
    longitude: String,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    Report {},

    Wait {
        #[structopt(
            help = "Choose a delay from your chosen event (see --event) in one of the following formats: {HH:MM:SS | HH:MM}. You may prepend the delay with '-' to make it negative. A negative offset will set the delay to be before the event, whilst a positive offset will set the delay to be after the event.",
            short = "o",
            long = "offset",
            default_value = "00:00:00",
            parse(from_str=parsers::parse_offset),
        )]
        offset: Duration,

        // should be one of [sunrise | sunset]
        #[structopt(
            help = "Choose one of {sunrise | sunset} from which to base your delay.", 
            short = "e", 
            long = "event", 
            parse(from_str=parsers::parse_event)
        )]
        event: enums::Event,
    },
}

#[derive(Debug, StructOpt)]
struct DateArgs {
    #[structopt(short = "d", long = "date")]
    date: Option<String>,

    #[structopt(short = "f", long = "date-format", default_value = "%Y-%m-%d")]
    date_fmt: String,

    #[structopt(short = "t", long = "time-zone")]
    time_zone: Option<String>,
}

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
    let args = Cli::from_args();

    let date = parsers::parse_date(
        args.date_args.date.as_deref(),
        &args.date_args.date_fmt,
        args.date_args.time_zone.as_deref(),
    );

    let coordinates = structs::Coordinates::from_decimal_degrees(&args.latitude, &args.longitude);

    let report = report::SolarReport::new(date, coordinates);

    match Cli::from_args().sub_cmd {
        SubCommand::Report {} => println!("{}", report),
        SubCommand::Wait { offset, event } => wait(offset, report, event),
    }
}
