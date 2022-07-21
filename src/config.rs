use std::{fs, path::Path, result};

use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveTime, TimeZone};
use clap::{Parser, Subcommand};
use serde::Deserialize;

use super::{
    enums,
    errors::{ConfigErrorKind, HeliocronError},
    structs,
};

type Result<T, E = HeliocronError> = result::Result<T, E>;

#[derive(Debug, Parser)]
#[clap(
    version,
    about = "\nA utility program for finding out what time various solar events occur, such as sunrise and \
            sunset, at a given location on a given date. It can be integrated into cron commands to \
            trigger program execution relative to these events.\n\n\
            For example, to execute a script 'turn-on-lights.sh' at sunrise, make a Crontab entry to trigger \
            at a time that will always be before the chosen event (say, 2am) and use heliocron to calculate \
            and perform the appropriate delay:\n\n\
            \t0 2 * * * heliocron --latitude 51.47N --longitude 3.1W wait --event sunrise && turn-on-lights.sh"
)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Commands,

    #[clap(flatten)]
    date_args: DateArgs,

    // the default values for latitude and longitude are handled differently to enable the user to set the values
    // either on the command line, in a config file or have a default provided by the program
    #[clap(
        short = 'l',
        long = "latitude",
        help = "Set the latitude in decimal degrees. Can also be set in ~/.config/heliocron.toml. [default: 51.4769N]",
        requires = "longitude"
    )]
    latitude: Option<String>,

    #[clap(
        short = 'o',
        long = "longitude",
        help = "Set the longitude in decimal degrees. Can also be set in ~/.config/heliocron.toml. [default: 0.0005W]",
        requires = "latitude"
    )]
    longitude: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Report {
        #[clap(
            help = "Set the output format to machine-readable JSON. If this flag is not present, the report will be displayed in the terminal as a block of human-readable text.",
            long = "json"
        )]
        json: bool,
    },

    /// Report whether it is day or night right now
    Now {
        /// Report day/night based on civil dawn/dusk rather than sunrise/sunset
        #[clap(short = 'c', long = "civil")]
        civil: bool,
    },

    Wait {
        #[clap(
            help = "Choose an event from which to base your delay.", 
            short = 'e',
            long = "event", 
            possible_values = &["sunrise", "sunset", "civil_dawn", "civil_dusk", "nautical_dawn", "nautical_dusk", "astronomical_dawn", "astronomical_dusk", "custom_am", "custom_pm", "solar_noon"]
        )]
        event_name: String,

        #[clap(
            help = "Choose a delay from your chosen event (see --event) in one of the following formats: {HH:MM:SS | HH:MM}. The value may be prepended with '-' to make it negative. A negative offset will set the delay to be before the event, whilst a positive offset will set the delay to be after the event.",
            short = 'o',
            long = "offset",
            default_value = "00:00:00",
            parse(try_from_str=parse_offset),
            allow_hyphen_values = true,
        )]
        offset: Duration,

        #[clap(
            help = "Set the elevation of the centre of the Sun relative to the horizon. Positive values mean that the centre of the Sun is below the horizon, whilst negative values mean that the centre of the sun is above the horizon. This argument is ignored if not specifying a custom event.",
            short = 'a',
            long = "altitude",
            allow_hyphen_values = true,
            parse(try_from_str=parse_altitude),
            required_if_eq_any = &[("event-name", "custom_am"), ("event-name", "custom_pm")]
        )]
        custom_altitude: Option<f64>,

        #[clap(
            long = "tag",
            help = "Add a short description to help identify the process e.g. when using htop. This parameter has no other effect on the running of the program."
        )]
        tag: Option<String>,

        #[clap(
            help = "Define whether the task should still be run even if the event has been missed. A tolerance of 30 seconds after the event is allowed before a task would be skipped. Setting this flag will cause the task to run regardless of how overdue it is.",
            long = "run-missed-event"
        )]
        run_missed_task: bool,
    },
}

fn parse_offset(offset: &str) -> Result<Duration, String> {
    // offset should either be %H:%M:%S or %H:%M +/- a "-" if negative
    let (positive, offset): (bool, &str) = match offset.chars().next() {
        Some('-') => (false, &offset[1..]),
        _ => (true, offset),
    };

    let pattern = if offset.len() == 5 {
        "%H:%M"
    } else {
        "%H:%M:%S"
    };
    let offset = NaiveTime::parse_from_str(offset, pattern)
        .map_err(|_e| "Expected an offset in the format '[-]HH:MM' or '[-]HH:MM:SS'".to_string())?;
    let offset = offset.signed_duration_since(NaiveTime::from_hms(0, 0, 0));

    if positive {
        Ok(offset)
    } else {
        Ok(-offset)
    }
}

fn parse_altitude(altitude: &str) -> Result<f64, String> {
    let altitude = altitude
        .parse::<f64>()
        .map_err(|_e| "Expected a number between -90.0 and 90.0".to_string())?;
    if (-90.0..=90.0).contains(&altitude) {
        Ok(altitude)
    } else {
        Err("Expected a number between -90.0 and 90.0".to_string())
    }
}

#[derive(Debug, Parser, Clone)]
struct DateArgs {
    #[clap(
        short = 'd',
        long = "date",
        help = "Set the date for which the calculations should be run. Expected to be in the format %Y-%m-%d and defaults to today's date in the local timezone if not set.",
        parse(try_from_str=parse_date)
    )]
    date: Option<NaiveDate>,

    #[clap(short = 't', long = "time-zone", allow_hyphen_values = true, parse(try_from_str=parse_tz))]
    time_zone: Option<FixedOffset>,
}

fn parse_date(d: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|e| e.to_string())
}

fn parse_tz(tz: &str) -> Result<chrono::FixedOffset, String> {
    let x = chrono::DateTime::parse_from_str(&format!("2022-01-01T00:00:00{}", tz), "%FT%T%:z")
        .map_err(|_| {
            format!(
                "Invalid timezone - expected [+|-]HH:MM between -23:59 and +23:59, got {:?}",
                tz
            )
        })?;
    Ok(*x.offset())
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
    Now {
        civil: bool,
    },
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
        // merge in location if set. Clap requires either both or neither of lat and long to be set
        if let (Some(latitude), Some(longitude)) = (cli_args.latitude, cli_args.longitude) {
            self.coordinates = structs::Coordinates::from_decimal_degrees(&latitude, &longitude)?
        }

        // set the date
        let date_args = cli_args.date_args;

        let date = date_args
            .date
            .unwrap_or_else(|| Local::today().naive_local());

        let time_zone = date_args
            .time_zone
            .unwrap_or_else(|| *Local::now().offset());

        self.date = time_zone.from_local_date(&date).unwrap().and_hms(12, 0, 0);

        self.action = match cli_args.subcommand {
            Commands::Wait {
                offset,
                custom_altitude,
                event_name,
                run_missed_task,
                ..
            } => {
                // do some gymnastics here. Structopt already validates that altitude is provided
                let event = enums::Event::new(event_name.as_str(), custom_altitude)?;
                Action::Wait {
                    offset,
                    event,
                    run_missed_task,
                }
            }
            Commands::Now { civil } => Action::Now { civil },
            Commands::Report { json } => Action::Report { json },
        };

        Ok(self)
    }
}

pub fn parse_config() -> Result<Config> {
    // master function for collecting all config variables and returning a single runtime configuration

    // 0. Set up default config
    let today = Local::today();
    let default_config = Config {
        coordinates: structs::Coordinates::from_decimal_degrees("51.4769N", "0.0005W")?,
        date: today.and_hms(12, 0, 0).with_timezone(today.offset()),
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
    let cli_args = Cli::parse();

    let config = config.merge_cli_args(cli_args)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_offset() {
        let valid_offsets = &[
            ("12:00:00", Duration::hours(12)),
            ("12:00", Duration::hours(12)),
            ("-12:00:00", -Duration::hours(12)),
            ("23:59:59", Duration::seconds(86399)),
            ("23:59", Duration::seconds(86340)),
            ("00:59", Duration::minutes(59)),
            ("00:00", Duration::minutes(0)),
        ];

        for (input, expected) in valid_offsets.iter() {
            let offset = parse_offset(*input);
            assert_eq!(offset, Ok(*expected));
        }

        let invalid_offsets = &["24:00:00"];

        for input in invalid_offsets.iter() {
            let offset = parse_offset(*input);
            assert!(offset.is_err());
        }
    }
}
