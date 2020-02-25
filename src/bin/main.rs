use sunwait::parsers;
use sunwait::report;

use structopt::StructOpt;

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
        #[structopt(short = "t", long = "offset", default_value = "00:00:00")]
        offset: String,
    },
}

#[derive(Debug, StructOpt)]
struct DateArgs {
    #[structopt(short = "d", long = "date", default_value = "2020-03-22")]
    date: String,

    #[structopt(short = "f", long = "date-format", default_value = "%Y-%m-%d")]
    date_fmt: String,

    #[structopt(short = "t", long = "time-zone")]
    time_zone: Option<String>,
}

fn main() {
    let args = Cli::from_args();

    let date = parsers::parse_date(
        &args.date_args.date,
        &args.date_args.date_fmt,
        args.date_args.time_zone.as_deref(),
    );

    let latitude: f64 = args.latitude;
    let longitude: f64 = args.longitude;

    let report = report::SolarReport::new(date, latitude, longitude);

    match Cli::from_args().sub_cmd {
        SubCommand::Report {} => println!("{}", report),
        SubCommand::Wait { offset } => println!("We need to wait for this long: {}", offset),
    }
}
