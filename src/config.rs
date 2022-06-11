use std::{fs, path::Path, result};

use chrono::{DateTime, Duration, FixedOffset, Local, TimeZone};
use dirs;
use serde::Deserialize;
use structopt::StructOpt;

use super::{
    enums,
    errors::{ConfigErrorKind, HeliocronError},
    parsers, structs,
};

type Result<T> = result::Result<T, HeliocronError>;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "A simple utility for finding out what time various solar events occur, such as sunrise and \
             sunset, at a given location on a given date. It can be integrated into cron commands to \
             trigger program execution relative to these events.\n\n\
             For example, to execute a script 'turn-on-lights.sh' at sunrise, make a Crontab entry to trigger \
             at a time that will always be before the chosen event (say, 2am) and use heliocron to calculate \
             and perform the appropriate delay:\n\n\
             \t0 2 * * * heliocron --latitude 51.47N --longitude 3.1W wait --event sunrise && turn-on-lights.sh"
)]
struct Cli {
    #[structopt(subcommand)]
    subcommand: Subcommand,

    #[structopt(flatten)]
    date_args: DateArgs,

    // the default values for latitude and longitude are handled differently to enable the user to set the values
    // either on the command line, in a config file or have a default provided by the program
    #[structopt(
        short = "l",
        long = "latitude",
        help = "Set the latitude in decimal degrees. Can also be set in ~/.config/heliocron.toml. [default: 51.4769N]",
        requires = "longitude"
    )]
    latitude: Option<String>,

    #[structopt(
        short = "o",
        long = "longitude",
        help = "Set the longitude in decimal degrees. Can also be set in ~/.config/heliocron.toml. [default: 0.0005W]",
        requires = "latitude"
    )]
    longitude: Option<String>,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    Report {
        #[structopt(
            help = "Set the output format to machine-readable JSON. If this flag is not present, the report will be displayed in the terminal as a block of human-readable text.",
            long = "json"
        )]
        json: bool,
    },

    Wait {
        #[structopt(
            help = "Choose a delay from your chosen event (see --event) in one of the following formats: {HH:MM:SS | HH:MM}. You may prepend the delay with '-' to make it negative. A negative offset will set the delay to be before the event, whilst a positive offset will set the delay to be after the event.",
            short = "o",
            long = "offset",
            default_value = "00:00:00",
            parse(from_str=parsers::parse_offset),
            allow_hyphen_values = true,
        )]
        offset: Result<Duration>,

        #[structopt(
            help = "Choose an event from which to base your delay.", 
            short = "e", 
            long = "event", 
            possible_values = &["sunrise", "sunset", "civil_dawn", "civil_dusk", "nautical_dawn", "nautical_dusk", "astronomical_dawn", "astronomical_dusk", "custom_am", "custom_pm", "solar_noon"]
        )]
        event_name: String,

        #[structopt(
            help = "Set the elevation of the centre of the Sun relative to the horizon. Positive values mean that the centre of the Sun is below the horizon, whilst negative values mean that the centre of the sun is above the horizon. This argument is ignored if not specifying a custom event.",
            short = "a",
            long = "altitude",
            allow_hyphen_values = true,
            required_ifs = &[("event-name", "custom_am"), ("event-name", "custom_pm")],
        )]
        custom_altitude: Option<String>, // we'll validate it to a float later

        #[structopt(
            long = "tag",
            help = "Add a short description to help identify the process e.g. when using htop. This parameter has no other effect on the running of the program."
        )]
        tag: Option<String>,

        #[structopt(
            help = "Define whether the task should still be run even if the event has been missed. A tolerance of 30 seconds after the event is allowed before a task would be skipped. Setting this flag will cause the task to run regardless of how overdue it is.",
            long = "run-missed-event"
        )]
        run_missed_task: bool,
    },
}

#[derive(Debug, StructOpt, Clone)]
struct DateArgs {
    #[structopt(short = "d", long = "date")]
    date: Option<String>,

    #[structopt(short = "f", long = "date-format", default_value = "%Y-%m-%d")]
    date_format: String,

    #[structopt(short = "t", long = "time-zone", allow_hyphen_values = true)]
    time_zone: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    latitude: Option<String>,
    longitude: Option<String>,
}

impl TomlConfig {
    fn new() -> TomlConfig {
        TomlConfig {
            latitude: None,
            longitude: None,
        }
    }

    fn from_toml(config: result::Result<TomlConfig, toml::de::Error>) -> TomlConfig {
        match config {
            Ok(conf) => conf,
            _ => TomlConfig::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Report {
        json: bool,
    },
    Wait {
        event: enums::Event,
        offset: Duration,
        run_missed_task: bool,
    },
}

#[derive(Debug, Clone)]
pub struct Config {
    // this master config struct contains all information parsed from the TOML file and CLI args
    // required to run any command
    pub coordinates: structs::Coordinates,
    pub date: DateTime<FixedOffset>,
    pub action: Action,
}

impl Config {
    fn merge_toml(mut self, toml_config: TomlConfig) -> Result<Config> {
        if let (Some(latitude), Some(longitude)) = (toml_config.latitude, toml_config.longitude) {
            self.coordinates = structs::Coordinates::from_decimal_degrees(&latitude, &longitude)?
        }
        Ok(self)
    }

    fn merge_cli_args(mut self, cli_args: Cli) -> Result<Config> {
        // merge in location if set. Structopt requires either both or neither of lat and long to be set
        if let (Some(latitude), Some(longitude)) = (cli_args.latitude, cli_args.longitude) {
            self.coordinates = structs::Coordinates::from_decimal_degrees(&latitude, &longitude)?
        }

        // set the date
        let date_args = cli_args.date_args;

        if let Some(date) = date_args.date {
            self.date = parsers::parse_date(
                &date,
                &date_args.date_format,
                date_args.time_zone.as_deref(),
            )?;
        }

        self.action = match cli_args.subcommand {
            Subcommand::Wait {
                offset,
                custom_altitude,
                event_name,
                run_missed_task,
                ..
            } => {
                // do some gymnastics here. Structopt already validates that altitude is provided
                let altitude = custom_altitude.map(parsers::parse_altitude).transpose()?;
                let event = enums::Event::new(event_name.as_str(), altitude)?;
                Action::Wait {
                    offset: offset?,
                    event,
                    run_missed_task,
                }
            }
            Subcommand::Report { json } => Action::Report { json },
        };

        Ok(self)
    }
}

pub fn parse_config() -> Result<Config> {
    // master function for collecting all config variables and returning a single runtime configuration

    // 0. Set up default config
    let default_config = Config {
        coordinates: structs::Coordinates::from_decimal_degrees("51.4769N", "0.0005W")?,
        date: Local::today()
            .and_hms(12, 0, 0)
            .with_timezone(&FixedOffset::from_offset(Local::today().offset())),
        // action will always get overwritten by the action provided later on from the CLI args
        action: Action::Report { json: false },
    };

    // 1. Overwrite defaults with config from ~/.config/heliocron.toml if present
    let config: Config = if cfg!(feature = "integration-test") {
        // if we are running integration tests, we actually just want to use the default config
        default_config
    } else {
        let path = dirs::config_dir()
            .unwrap() // this shouldn't ever really be None?
            .join(Path::new("heliocron.toml"));

        let file = fs::read_to_string(path);

        let config: Config = match file {
            Ok(f) => match default_config.merge_toml(TomlConfig::from_toml(toml::from_str(&f))) {
                Ok(merged_config) => Ok(merged_config),
                // any errors parsing the .toml raise an error
                Err(_) => Err(HeliocronError::Config(ConfigErrorKind::InvalidTomlFile)),
            },
            // any problems opening the .toml file and we just continue on with the default configuration
            Err(_) => Ok(default_config),
        }?;

        config
    };

    // 2. Overwrite any currently set config with CLI arguments
    let cli_args = Cli::from_args();

    let config = config.merge_cli_args(cli_args)?;

    Ok(config)
}
