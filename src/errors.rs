use std::error::Error;

use crate::sleep;
use chrono::{self, DateTime, FixedOffset};

#[derive(Debug)]
pub enum HeliocronError {
    Config(ConfigErrorKind),
    Runtime(RuntimeErrorKind),
}

#[derive(Debug, Clone)]
pub enum ConfigErrorKind {
    InvalidCoordindates(&'static str),
    InvalidTomlFile,
    ParseDate,
    ParseAltitude,
    ParseOffset,
    InvalidEvent,
}

impl ConfigErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            ConfigErrorKind::InvalidCoordindates(msg) => msg,
            ConfigErrorKind::InvalidTomlFile => {
                "Error parsing TOML file. Ensure that it is of the correct format."
            }
            ConfigErrorKind::ParseDate => {
                "Error parsing date. Ensure the date is formatted correctly."
            }
            ConfigErrorKind::ParseAltitude => {
                "Error parsing altitude. Must be a number between -90.0 and 90.0."
            }
            ConfigErrorKind::ParseOffset => {
                "Error parsing offset. Expected a string in the format HH:MM:SS or HH:MM."
            }
            ConfigErrorKind::InvalidEvent => "Error parsing event.",
        }
    }
}

#[derive(Debug)]
pub enum RuntimeErrorKind {
    NonOccurringEvent,
    PastEvent(DateTime<FixedOffset>),
    EventMissed(i64),
    SleepError(sleep::Error),
}

impl std::fmt::Display for HeliocronError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Config(ref err) => write!(
                f,
                "Config error: {}",
                match err {
                    ConfigErrorKind::InvalidCoordindates(msg) =>
                        format!("Invalid coordinates - {msg}"),
                    ConfigErrorKind::InvalidTomlFile => err.as_str().to_string(),
                    ConfigErrorKind::ParseDate => err.as_str().to_string(),
                    ConfigErrorKind::ParseAltitude => err.as_str().to_string(),
                    ConfigErrorKind::ParseOffset => err.as_str().to_string(),
                    ConfigErrorKind::InvalidEvent => err.as_str().to_string(),
                }
            ),
            Self::Runtime(ref err) => write!(
                f,
                "Runtime error: {}",
                match err {
                    RuntimeErrorKind::NonOccurringEvent =>
                        "The chosen event does not occur on this day.".to_string(),
                    RuntimeErrorKind::PastEvent(when) => {
                        format!("The chosen event occurred in the past: {when}. Cannot wait a negative amount of time.")
                    }
                    RuntimeErrorKind::EventMissed(by) => format!("Event missed by {by}s"),
                    RuntimeErrorKind::SleepError(e) => e.to_string(),
                }
            ),
        }
    }
}

impl Error for HeliocronError {}

impl From<chrono::ParseError> for HeliocronError {
    fn from(_err: chrono::ParseError) -> Self {
        HeliocronError::Config(ConfigErrorKind::ParseDate)
    }
}

impl From<sleep::Error> for HeliocronError {
    fn from(err: sleep::Error) -> Self {
        HeliocronError::Runtime(RuntimeErrorKind::SleepError(err))
    }
}
