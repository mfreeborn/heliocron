use std::fmt;
use std::ops::RangeInclusive;

use chrono::{DateTime, Duration, FixedOffset, NaiveTime};
use serde::Serialize;

/// An enumeration of the different parts of the day. Not all of them necessarily occur during a
/// given 24-hour period.
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DayPart {
    Day,
    CivilTwilight,
    NauticalTwilight,
    AstronomicalTwilight,
    Night,
}

impl DayPart {
    pub fn from_elevation_angle(angle: f64) -> Self {
        if angle < -18.0 {
            Self::Night
        } else if angle < -12.0 {
            Self::AstronomicalTwilight
        } else if angle < -6.0 {
            Self::NauticalTwilight
        } else if angle < 0.833 {
            Self::CivilTwilight
        } else {
            Self::Day
        }
    }
}

impl fmt::Display for DayPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Day => "Day",
                Self::CivilTwilight => "Civil Twilight",
                Self::NauticalTwilight => "Nautical Twilight",
                Self::AstronomicalTwilight => "Astronomical Twilight",
                Self::Night => "Night",
            }
        )
    }
}

/// An enumeration of parsed commands.
pub enum Action {
    Report {
        json: bool,
    },
    Wait {
        event: Event,
        offset: Duration,
        run_missed_task: bool,
    },
    Poll {
        watch: bool,
        json: bool,
    },
}

/// A newtype representing an optional datetime.
///
/// This allows us to provide custom serialization methods when converting to a String or JSON.
#[derive(Debug)]
pub struct EventTime(pub Option<DateTime<FixedOffset>>);

impl EventTime {
    pub fn new(datetime: Option<DateTime<FixedOffset>>) -> Self {
        Self(datetime)
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn time(&self) -> Option<NaiveTime> {
        self.0.map(|dt| dt.time())
    }
}

impl Serialize for EventTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            Some(datetime) => serializer.serialize_str(&datetime.to_rfc3339()),
            None => serializer.serialize_none(),
        }
    }
}

impl fmt::Display for EventTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                Some(datetime) => datetime.to_string(),
                None => "Never".to_string(),
            }
        )
    }
}

/// Newtype wrapper for validating an altitude between -90.0 and 90.0.
#[derive(Clone)]
pub struct Altitude(f64);

impl Altitude {
    pub fn new(alt: f64) -> Result<Self, String> {
        if (-90.0..=90.0).contains(&alt) {
            Ok(Self(alt))
        } else {
            Err(format!(
                "Expected a number between -90.0 and 90.0. Found '{alt}'"
            ))
        }
    }

    pub fn parse(alt: &str) -> Result<Self, String> {
        match alt.parse() {
            Ok(alt) => Self::new(alt),
            Err(alt) => Err(format!(
                "Expected a number between -90.0 and 90.0. Found '{alt}'"
            )),
        }
    }
}

impl From<f64> for Altitude {
    fn from(alt: f64) -> Self {
        Self::new(alt).unwrap()
    }
}

impl std::ops::Deref for Altitude {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A list of plain event names supported by the command line interface.
#[derive(Clone)]
pub enum RawEventName {
    Sunrise,
    Sunset,
    CivilDawn,
    CivilDusk,
    NauticalDawn,
    NauticalDusk,
    AstronomicalDawn,
    AstronomicalDusk,
    CustomAM,
    CustomPM,
    SolarNoon,
}

impl clap::ValueEnum for RawEventName {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Sunrise,
            Self::Sunset,
            Self::CivilDawn,
            Self::CivilDusk,
            Self::NauticalDawn,
            Self::NauticalDusk,
            Self::AstronomicalDawn,
            Self::AstronomicalDusk,
            Self::CustomAM,
            Self::CustomPM,
            Self::SolarNoon,
        ]
    }

    fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::PossibleValue<'a>> {
        match self {
            Self::Sunrise => Some(clap::PossibleValue::new("sunrise")),
            Self::Sunset => Some(clap::PossibleValue::new("sunset")),
            Self::CivilDawn => Some(clap::PossibleValue::new("civil_dawn")),
            Self::CivilDusk => Some(clap::PossibleValue::new("civil_dusk")),
            Self::NauticalDawn => Some(clap::PossibleValue::new("nautical_dawn")),
            Self::NauticalDusk => Some(clap::PossibleValue::new("nautical_dusk")),
            Self::AstronomicalDawn => Some(clap::PossibleValue::new("astronomical_dawn")),
            Self::AstronomicalDusk => Some(clap::PossibleValue::new("astronomical_dusk")),
            Self::CustomAM => Some(clap::PossibleValue::new("custom_am")),
            Self::CustomPM => Some(clap::PossibleValue::new("custom_pm")),
            Self::SolarNoon => Some(clap::PossibleValue::new("solar_noon")),
        }
    }
}

/// An enumeration of possible event names, with required data attached.
///
/// For example, CustomAM/PM here include the custom altitude, in contrast to
/// `RawEventName` where that data is absent.
pub enum EventName {
    Sunrise,
    Sunset,
    CivilDawn,
    CivilDusk,
    NauticalDawn,
    NauticalDusk,
    AstronomicalDawn,
    AstronomicalDusk,
    CustomAM(Altitude),
    CustomPM(Altitude),
    SolarNoon,
}

/// The set of possible directions of travel for a celestial object relative to the obeserver, i.e.
/// either ascending or descending.
pub enum Direction {
    Ascending,
    Descending,
}

/// Events which occur when the Sun reaches a specific elevation relative to the horizon.
///
/// For example, sunrise always occurs when the centre of the Sun is 0.833 degrees below the horizon.
pub struct FixedElevationEvent {
    pub degrees_below_horizon: Altitude,
    pub solar_direction: Direction,
}

impl FixedElevationEvent {
    pub fn new(degrees_below_horizon: Altitude, solar_direction: Direction) -> Self {
        Self {
            degrees_below_horizon,
            solar_direction,
        }
    }
}

/// Events which occur when the Sun is at a variable elevation.
///
/// For example, solar noon occurs at the maximum solar elevation, which varies based on time and location.
pub enum VariableElevationEvent {
    SolarNoon,
}

/// Any supported solar event.
///
/// Some events, such as sunrise and sunset, occur when the Sun is at a specific altitude relative to the horizon,
/// but other events, such as solar noon, occur not at a fixed altitude, but a variable one. Each of these has a
/// different way of calculating the time of the event, hence they are separated into two variants.
pub enum Event {
    Fixed(FixedElevationEvent),
    Variable(VariableElevationEvent),
}

impl Event {
    pub fn from_event_name(event: EventName) -> Self {
        // We can just use `.into()` (a method which can panic) for these float conversions because we can manually
        // verify that all of them are valid altitudes.
        match event {
            EventName::Sunrise => {
                Self::Fixed(FixedElevationEvent::new(0.833.into(), Direction::Ascending))
            }
            EventName::Sunset => Self::Fixed(FixedElevationEvent::new(
                0.833.into(),
                Direction::Descending,
            )),
            EventName::CivilDawn => {
                Self::Fixed(FixedElevationEvent::new(6.0.into(), Direction::Ascending))
            }
            EventName::CivilDusk => {
                Self::Fixed(FixedElevationEvent::new(6.0.into(), Direction::Descending))
            }
            EventName::NauticalDawn => {
                Self::Fixed(FixedElevationEvent::new(12.0.into(), Direction::Ascending))
            }
            EventName::NauticalDusk => {
                Self::Fixed(FixedElevationEvent::new(12.0.into(), Direction::Descending))
            }
            EventName::AstronomicalDawn => {
                Self::Fixed(FixedElevationEvent::new(18.0.into(), Direction::Ascending))
            }
            EventName::AstronomicalDusk => {
                Self::Fixed(FixedElevationEvent::new(18.0.into(), Direction::Descending))
            }
            EventName::CustomAM(alt) => {
                Self::Fixed(FixedElevationEvent::new(alt, Direction::Ascending))
            }
            EventName::CustomPM(alt) => {
                Self::Fixed(FixedElevationEvent::new(alt, Direction::Descending))
            }
            EventName::SolarNoon => Self::Variable(VariableElevationEvent::SolarNoon),
        }
    }
}

const LATITUDE_RANGE: RangeInclusive<f64> = RangeInclusive::new(-90.0, 90.0);
const LONGITUDE_RANGE: RangeInclusive<f64> = RangeInclusive::new(-180.0, 180.0);

/// Represents a latitude in decimal degrees. Valid values are from -90.0..=+90.0.
/// Positive values are to the north, whilst negative values are to the south.
#[derive(PartialEq, Debug, Clone, serde::Serialize)]
pub struct Latitude(f64);

impl Latitude {
    /// Create a new instance of `Latitude` from an f64.
    pub fn new(value: f64) -> Result<Self, String> {
        match LATITUDE_RANGE.contains(&value) {
            true => Ok(Self(value)),
            false => Err(format!(
                "Latitude must be between -90.0 and 90.0, inclusive. Found `{value}`."
            )),
        }
    }

    /// Create a new instance of `Latitude` from an &str, such as when parsing command line
    /// arguments.
    pub fn parse(value: &str) -> Result<Self, String> {
        value
            .parse()
            .map_err(|_| {
                format!("Latitude must be between -90.0 and 90.0, inclusive. Found `{value}`.")
            })
            .and_then(Self::new)
    }
}

impl fmt::Display for Latitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for Latitude {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a longitude in decimal degrees. Valid values are from -180.0..=+180.0.
/// Positive values are to the east, whilst negative values are to the west.
#[derive(PartialEq, Debug, Clone, serde::Serialize)]
pub struct Longitude(f64);

impl Longitude {
    /// Create a new `Longitude` from an f64.
    pub fn new(value: f64) -> Result<Self, String> {
        match LONGITUDE_RANGE.contains(&value) {
            true => Ok(Self(value)),
            false => Err(format!(
                "Longitude must be between -180.0 and 180.0, inclusive. Found '{value}'."
            )),
        }
    }

    /// Create a new instance of `Longitude` from an &str, such as when parsing command line
    /// arguments.
    pub fn parse(value: &str) -> Result<Self, String> {
        value
            .parse()
            .map_err(|_| {
                format!("Longitude must be between -180.0 and 180.0, inclusive. Found `{value}`.")
            })
            .and_then(Self::new)
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for Longitude {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents poisition on a map described by a latitude and longitude.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Coordinates {
    pub latitude: Latitude,
    pub longitude: Longitude,
}

impl Coordinates {
    pub fn new(latitude: Latitude, longitude: Longitude) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_latitude() {
        let lat = Latitude::new(15.1234).unwrap();
        let expected = Latitude(15.1234);

        assert_eq!(lat, expected);
    }

    #[test]
    fn test_new_latitude_with_valid_values() {
        let vals = &[-90.0, -89.9999999999, -0.0, 0.0, 90.0];

        for val in vals {
            assert!(Latitude::new(*val).is_ok());
        }
    }

    #[test]
    fn test_new_latitude_with_invalid_values() {
        let vals = &[-180.0, -90.00000000001, 90.00000001, 100.0];

        for val in vals {
            assert!(Latitude::new(*val).is_err());
        }
    }

    #[test]
    fn test_parse_latitude_with_valid_values() {
        let vals = &["-90.0", "-89.9999999999", "-0.0", "0.0", "90.0"];

        for val in vals {
            assert!(Latitude::parse(*val).is_ok());
        }
    }

    #[test]
    fn test_parse_latitude_with_invalid_values() {
        let vals = &["-180.0", "-90.00000000001", "90.00000001", "100.0"];

        for val in vals {
            assert!(Latitude::parse(*val).is_err());
        }
    }

    #[test]
    fn test_new_longitude() {
        let lat = Longitude::new(-150.1234).unwrap();
        let expected = Longitude(-150.1234);

        assert_eq!(lat, expected);
    }

    #[test]
    fn test_new_longitude_with_valid_values() {
        let vals = &[-180.0, -90.0, -89.9999, -0.0, 0.0, 90.0, 180.0];

        for val in vals {
            assert!(Longitude::new(*val).is_ok());
        }
    }

    #[test]
    fn test_new_longitude_with_invalid_values() {
        let vals = &[-180.1, 180.01];

        for val in vals {
            assert!(Longitude::new(*val).is_err());
        }
    }

    #[test]
    fn test_parse_longitude_with_valid_values() {
        let vals = &[
            "-180.0", "-90.0", "-89.9999", "-0.0", "0.0", "90.0", "180.0",
        ];

        for val in vals {
            assert!(Longitude::parse(*val).is_ok());
        }
    }

    #[test]
    fn test_parse_longitude_with_invalid_values() {
        let vals = &["-180.1", "180.01"];

        for val in vals {
            assert!(Longitude::parse(*val).is_err());
        }
    }

    #[test]
    fn test_new_coordinates() {
        let latitude = Latitude::new(10.0).unwrap();
        let longitude = Longitude::new(20.0).unwrap();

        let coords = Coordinates::new(latitude.clone(), longitude.clone());
        let expected = Coordinates {
            latitude,
            longitude,
        };

        assert_eq!(coords, expected);
    }

    #[test]
    fn test_serialize_event_time() {
        let dt = DateTime::parse_from_rfc3339("2022-06-11T12:00:00+01:00").unwrap();
        let et = EventTime::new(Some(dt));
        // serialize to rfc3339
        let expected = serde_json::json!("2022-06-11T12:00:00+01:00");
        assert_eq!(serde_json::to_value(et).unwrap(), expected);

        let et = EventTime::new(None);
        //serialize to null
        let expected = serde_json::json!(null);
        assert_eq!(serde_json::to_value(et).unwrap(), expected);
    }

    #[test]
    fn test_display_event_time() {
        let dt = DateTime::parse_from_rfc3339("2022-06-11T12:00:00+01:00").unwrap();
        let et = EventTime::new(Some(dt));
        let expected = "2022-06-11 12:00:00 +01:00";
        assert_eq!(et.to_string(), expected);

        let et = EventTime::new(None);
        let expected = "Never";
        assert_eq!(et.to_string(), expected);
    }
}
