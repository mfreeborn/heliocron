use std::error;

use chrono;

#[derive(Debug, Clone)]
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
                "Error parsing date. Ensure the date and timezone formats are correct."
            }
            ConfigErrorKind::ParseAltitude => {
                "Error parsing altitude. Must be a number which is <= 90.0 and >= -90.0."
            }
            ConfigErrorKind::ParseOffset => {
                "Error parsing offset. Expected a string in the format HH:MM:SS or HH:MM."
            }
            ConfigErrorKind::InvalidEvent => "Error parsing event.",
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeErrorKind {
    NonOccurringEvent,
    PastEvent,
}

impl RuntimeErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            RuntimeErrorKind::NonOccurringEvent => "The chosen event does not occur on this day.",
            RuntimeErrorKind::PastEvent => {
                "The chosen event occurred in the past; cannot wait a negative amount of time."
            }
        }
    }
}

impl std::fmt::Display for HeliocronError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            HeliocronError::Config(ref err) => write!(
                f,
                "Config error: {}",
                match err {
                    ConfigErrorKind::InvalidCoordindates(msg) =>
                        format!("Invalid coordinates - {}", msg),
                    ConfigErrorKind::InvalidTomlFile => err.as_str().to_string(),
                    ConfigErrorKind::ParseDate => err.as_str().to_string(),
                    ConfigErrorKind::ParseAltitude => err.as_str().to_string(),
                    ConfigErrorKind::ParseOffset => err.as_str().to_string(),
                    ConfigErrorKind::InvalidEvent => err.as_str().to_string(),
                }
            ),
            HeliocronError::Runtime(ref err) => write!(
                f,
                "Runtime error: {}",
                match err {
                    RuntimeErrorKind::NonOccurringEvent => err.as_str().to_string(),
                    RuntimeErrorKind::PastEvent => err.as_str().to_string(),
                }
            ),
        }
    }
}

impl error::Error for HeliocronError {
    fn description(&self) -> &str {
        match *self {
            HeliocronError::Config(ref err) => err.as_str(),
            HeliocronError::Runtime(ref err) => err.as_str(),
        }
    }
}

impl From<chrono::ParseError> for HeliocronError {
    fn from(err: chrono::ParseError) -> Self {
        match err {
            _err => HeliocronError::Config(ConfigErrorKind::ParseDate),
        }
    }
}
