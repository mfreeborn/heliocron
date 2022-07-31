use std::{fs, path::PathBuf, result};

use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, NaiveDate, NaiveTime, TimeZone};
use clap::{Parser, Subcommand};
use serde::Deserialize;

use super::{domain, enums, errors::HeliocronError};

type Result<T, E = HeliocronError> = result::Result<T, E>;

#[derive(Debug, Parser)]
#[clap(version, about, long_about=None)]
struct Cli {
    /// Set the date for which the calculations should be run. If specified, it should be in 'yyyy-mm-dd' format.
    #[clap(
        short = 'd',
        long = "date",
        value_parser=parse_date,
        default_value_t=Local::today().naive_local()
    )]
    date: NaiveDate,

    /// Set the time zone. If specified, it should be in the format '[+/-]HH:MM', otherwise it defaults to the current local time zone.
    #[clap(short = 't', long = "time-zone", allow_hyphen_values = true, value_parser=parse_tz, default_value_t=*Local::today().offset())]
    time_zone: FixedOffset,

    /// Set the latitude in decimal degrees. Positive values to the north; negative values to the south. Defaults to '51.4769' if not
    /// otherwise specified here or in ~.config/heliocron.toml.
    #[clap(short = 'l', long = "latitude", requires = "longitude", allow_hyphen_values = true, value_parser = domain::Latitude::parse)]
    latitude: Option<domain::Latitude>,

    /// Set the longitude in decimal degrees. Positive values to the east; negative values to the west. Defaults to -0.0005 if not
    /// otherwise specified here or in ~/.config/heliocron.toml.
    #[clap(short = 'o', long = "longitude", requires = "latitude", allow_hyphen_values = true, value_parser = domain::Longitude::parse)]
    longitude: Option<domain::Longitude>,

    #[clap(subcommand)]
    subcommand: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Report {
        #[clap(
            help = "Set the output format to machine-readable JSON. If this flag is not present, the report will be displayed in the terminal as a block of human-readable text.",
            long = "json"
        )]
        json: bool,
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

fn parse_date(date: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| format!("Invalid date - must be in the format 'yyyy-mm-dd'. Found '{date}'"))
}

fn parse_tz(tz: &str) -> Result<chrono::FixedOffset, String> {
    // Use chrono's own parsing function to validate the provided time zone.
    let date = chrono::DateTime::parse_from_str(&format!("2022-01-01T00:00:00{}", tz), "%FT%T%:z")
        .map_err(|_| {
            format!(
                "Invalid time zone - expected the format '[+|-]HH:MM' between '-23:59' and '+23:59'. Found '{tz}'"
            )
        })?;
    Ok(*date.offset())
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    latitude: Option<f64>,
    longitude: Option<f64>,
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

/// Container for all necessary runtime configuration.
#[derive(Debug, Clone)]
pub struct Config {
    pub coordinates: domain::Coordinates,
    pub date: DateTime<FixedOffset>,
    pub action: Action,
}

/// Parse all configuration streams into one valid runtime configuration. Where supported, arguments passed over the
/// command line take precedence over values found in configuration files, which, in turn, takes precedence over
/// any hard coded default values.
pub fn parse_config() -> Result<Config, HeliocronError> {
    let cli_args = Cli::parse();

    let coordinates = {
        // First try the command line arguments...
        if let (Some(lat), Some(lon)) = (cli_args.latitude, cli_args.longitude) {
            domain::Coordinates::new(lat, lon)
        } else {
            // ...failing that, check if the coordinates are set in a config file...
            dirs::config_dir()
                .map(|path| path.join("heliocron.toml"))
                .filter(|path| path.exists())
                .map(|path| parse_local_config(&path))
                .and_then(|res| {
                    match res {
                        Ok(coords) => Some(coords),
                        Err(e) => {
                            eprintln!("Warning - couldn't parse configuration file due to the following reason: {}\n. Proceeding with default coordinates.", e);
                            None
                        }
                        }
                })
                .unwrap_or_else(|| {
                    // ...otherwise default to some hardcoded values. Safe to unwrap because we know these values are valid.
                    domain::Coordinates::new(
                        domain::Latitude::new(51.4769).unwrap(),
                        domain::Longitude::new(-0.0005).unwrap(),
                    )
                })
        }
    };

    let date = cli_args
        .time_zone
        .ymd(
            cli_args.date.year(),
            cli_args.date.month(),
            cli_args.date.day(),
        )
        .and_hms(12, 0, 0);

    let action = match cli_args.subcommand {
        Command::Report { json } => Action::Report { json },
        Command::Wait {
            event_name,
            offset,
            run_missed_task,
            custom_altitude,
            ..
        } => Action::Wait {
            event: enums::Event::new(event_name.as_str(), custom_altitude).unwrap(),
            offset,
            run_missed_task,
        },
    };

    Ok(Config {
        coordinates,
        date,
        action,
    })
}

fn parse_local_config(path: &PathBuf) -> Result<domain::Coordinates, String> {
    let config_file = fs::read(path).map_err(|_| "Failed to read config file path".to_string())?;
    let toml_config = toml::from_slice::<TomlConfig>(&config_file).map_err(
        |e| e.to_string(), // "Failed to parse TOML file".to_string()
    )?;

    let (lat, lon) = match (toml_config.latitude, toml_config.longitude) {
        (Some(lat), Some(lon)) => Ok((lat, lon)),
        (Some(_lat), None) => Err("Missing longitude".to_string()),
        (None, Some(_lon)) => Err("Missing latitude".to_string()),
        (None, None) => Err("Missing latitude and longitude".to_string()),
    }?;

    let lat = domain::Latitude::new(lat)?;
    let lon = domain::Longitude::new(lon)?;

    Ok(domain::Coordinates::new(lat, lon))
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
