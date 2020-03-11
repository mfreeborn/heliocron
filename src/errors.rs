use std::error;

#[derive(Debug)]
pub enum HeliocronError {
    Config(ConfigErrorKind),
}

#[derive(Debug)]
pub enum ConfigErrorKind {
    InvalidCoordindates(&'static str),
}

impl ConfigErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            ConfigErrorKind::InvalidCoordindates(msg) => msg,
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
                }
            ),
        }
    }
}

impl error::Error for HeliocronError {
    fn description(&self) -> &str {
        match *self {
            HeliocronError::Config(ref err) => err.as_str(),
        }
    }
}
