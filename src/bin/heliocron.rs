use std::thread;

use chrono::Duration;
use structopt::StructOpt;

use heliocron::parsers;
use heliocron::report;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    sub_cmd: SubCommand,

    #[structopt(flatten)]
    date_args: DateArgs,

    #[structopt(short = "l", long = "latitude", default_value = "51.0782N", parse(from_str=parsers::parse_latlon))]
    latitude: f64,

    #[structopt(short = "o", long = "longitude", default_value = "4.0583W", parse(from_str=parsers::parse_latlon))]
    longitude: f64,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    Report {},

    Wait {
        #[structopt(
            short = "o",
            long = "offset",
            default_value = "00:00:00",
            parse(from_str=parsers::parse_offset)
        )]
        offset: Duration,

        // should be one of [sunrise | sunset]
        #[structopt(short = "e", long = "event", parse(from_str=parsers::parse_event))]
        event: String,
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

fn wait(offset: Duration, report: report::SolarReport, event: String) {
    println!("{}", report);
    println!(
        "We need to wait for this long: {}s from {}",
        offset.num_seconds(),
        event
    );
    thread::sleep(offset.to_std().unwrap());
}

fn main() {
    let args = Cli::from_args();

    let date = parsers::parse_date(
        args.date_args.date.as_deref(),
        &args.date_args.date_fmt,
        args.date_args.time_zone.as_deref(),
    );

    let latitude: f64 = args.latitude;
    let longitude: f64 = args.longitude;

    let report = report::SolarReport::new(date, latitude, longitude);

    match Cli::from_args().sub_cmd {
        SubCommand::Report {} => println!("{}", report),
        SubCommand::Wait { offset, event } => wait(offset, report, event),
    }
}
