use std::{fmt, result};

use chrono::{DateTime, FixedOffset, NaiveTime};
use serde::Serialize;

use super::errors::HeliocronError;

type Result<T, E = HeliocronError> = result::Result<T, E>;

/// A newtype representing an optional datetime. This allows us to provide custom
/// serialization methods when converting to a String or JSON.
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

#[cfg(test)]
mod tests {
    use super::*;

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
