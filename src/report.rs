use std::{collections::HashMap, fmt};

use chrono::{DateTime, Duration, FixedOffset};
use serde::ser::{Serialize, SerializeStruct};

use super::{
    calc,
    domain::EventTime,
    domain::{self, Coordinates},
};

#[derive(Debug)]
pub struct SolarReport {
    pub date: DateTime<FixedOffset>,
    pub coordinates: Coordinates,

    pub solar_noon: EventTime,
    pub day_length: Duration,

    pub sunrise: EventTime,
    pub sunset: EventTime,

    pub civil_dawn: EventTime,
    pub civil_dusk: EventTime,

    pub nautical_dawn: EventTime,
    pub nautical_dusk: EventTime,

    pub astronomical_dawn: EventTime,
    pub astronomical_dusk: EventTime,
}

impl fmt::Display for SolarReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_str = self.format_report();
        write!(f, "{}", fmt_str)
    }
}

impl Serialize for SolarReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("SolarReport", 12)?;
        state.serialize_field("date", &self.date.to_rfc3339())?;
        state.serialize_field("location", &self.coordinates)?;
        state.serialize_field("day_length", &self.day_length.num_seconds())?;
        state.serialize_field("solar_noon", &self.solar_noon)?;
        state.serialize_field("sunrise", &self.sunrise)?;
        state.serialize_field("sunset", &self.sunset)?;

        let mut dawn = HashMap::with_capacity(3);
        dawn.insert("civil", &self.civil_dawn);
        dawn.insert("nautical", &self.nautical_dawn);
        dawn.insert("astronomical", &self.astronomical_dawn);
        state.serialize_field("dawn", &dawn)?;

        let mut dusk = HashMap::with_capacity(3);
        dusk.insert("civil", &self.civil_dusk);
        dusk.insert("nautical", &self.nautical_dusk);
        dusk.insert("astronomical", &self.astronomical_dusk);
        state.serialize_field("dusk", &dusk)?;

        state.end()
    }
}

impl SolarReport {
    pub fn new(solar_calculations: calc::SolarCalculations) -> SolarReport {
        // we can unwrap all of these safely because they have been manually validated against the Events::new constructor
        let sunrise = solar_calculations
            .event_time(domain::Event::from_event_name(domain::EventName::Sunrise));
        let sunset = solar_calculations
            .event_time(domain::Event::from_event_name(domain::EventName::Sunset));
        let civil_dawn = solar_calculations
            .event_time(domain::Event::from_event_name(domain::EventName::CivilDawn));
        let civil_dusk = solar_calculations
            .event_time(domain::Event::from_event_name(domain::EventName::CivilDusk));
        let nautical_dawn = solar_calculations.event_time(domain::Event::from_event_name(
            domain::EventName::NauticalDawn,
        ));
        let nautical_dusk = solar_calculations.event_time(domain::Event::from_event_name(
            domain::EventName::NauticalDusk,
        ));
        let astronomical_dawn = solar_calculations.event_time(domain::Event::from_event_name(
            domain::EventName::AstronomicalDawn,
        ));
        let astronomical_dusk = solar_calculations.event_time(domain::Event::from_event_name(
            domain::EventName::AstronomicalDusk,
        ));
        let solar_noon = solar_calculations
            .event_time(domain::Event::from_event_name(domain::EventName::SolarNoon));

        SolarReport {
            date: solar_calculations.date,
            coordinates: solar_calculations.coordinates.clone(),
            solar_noon,
            day_length: solar_calculations.day_length(),
            sunrise,
            sunset,
            civil_dawn,
            civil_dusk,
            nautical_dawn,
            nautical_dusk,
            astronomical_dawn,
            astronomical_dusk,
        }
    }

    fn format_report(&self) -> String {
        format!(
            "LOCATION\n\
        --------\n\
        Latitude: {}\n\
        Longitude: {}\n\n\
        DATE\n\
        ----\n\
        {}\n\n\
        Solar noon is at:         {}\n\
        The day length is:        {}\n\n\
        Sunrise is at:            {}\n\
        Sunset is at:             {}\n\n\
        Civil dawn is at:         {}\n\
        Civil dusk is at:         {}\n\n\
        Nautical dawn is at:      {}\n\
        Nautical dusk is at:      {}\n\n\
        Astronomical dawn is at:  {}\n\
        Astronomical dusk is at:  {}
        ",
            self.coordinates.latitude,
            self.coordinates.longitude,
            self.date,
            self.solar_noon,
            SolarReport::day_length_hms(self.day_length),
            self.sunrise,
            self.sunset,
            self.civil_dawn,
            self.civil_dusk,
            self.nautical_dawn,
            self.nautical_dusk,
            self.astronomical_dawn,
            self.astronomical_dusk
        )
    }

    fn day_length_hms(day_length: Duration) -> String {
        let day_length = day_length.num_seconds();
        let hours = (day_length / 60) / 60;
        let minutes = (day_length / 60) % 60;
        let seconds = day_length % 60;

        format!("{}h {}m {}s", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::domain::{Latitude, Longitude};

    #[test]
    fn test_solar_report_new() {
        // check that a 'new' method is defined and takes a coordinate and a date as parameters
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = Coordinates {
            latitude: Latitude::new(0.0).unwrap(),
            longitude: Longitude::new(0.0).unwrap(),
        };

        let calcs = calc::SolarCalculations::new(date, coordinates);
        // Default trait should handle the rest of the parameters
        let _new_report = SolarReport::new(calcs);
    }

    #[test]
    fn test_report_content() {
        // check that the report contains all the correct metrics
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = Coordinates {
            latitude: Latitude::new(0.0).unwrap(),
            longitude: Longitude::new(0.0).unwrap(),
        };

        let calcs = calc::SolarCalculations::new(date, coordinates);

        let report = SolarReport::new(calcs);
        let report_str = report.format_report();

        assert!(report_str.contains("LOCATION"));
        assert!(report_str.contains("DATE"));
        assert!(report_str.contains("Latitude"));
        assert!(report_str.contains("Longitude"));
        assert!(report_str.contains("Sunrise is at"));
        assert!(report_str.contains("Solar noon is at"));
        assert!(report_str.contains("Sunset is at"));
        assert!(report_str.contains("The day length is"));

        let sunrise_str = format!("{}", report.sunrise);
        assert!(report_str.contains(&sunrise_str));

        let solar_noon_str = format!("{}", report.solar_noon);
        assert!(report_str.contains(&solar_noon_str));

        let sunset_str = format!("{}", report.sunset);
        assert!(report_str.contains(&sunset_str));

        let day_length_str = SolarReport::day_length_hms(report.day_length);
        assert!(report_str.contains(&day_length_str));
    }

    #[test]
    #[rustfmt::skip]
    fn test_sunrise_sunset() {
        // validated against NOAA calculations https://www.esrl.noaa.gov/gmd/grad/solcalc/calcdetails.html
        // ~Springtime
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates =Coordinates {
            latitude: Latitude::new(55.9533).unwrap(),
            longitude: Longitude::new(-3.1883).unwrap(),
        }
            ;
        let calcs = calc::SolarCalculations::new(date, coordinates);

        let report = SolarReport::new(calcs);

        assert_eq!("06:00:07", report.sunrise.time().unwrap().to_string());
        assert_eq!("18:36:59", report.sunset.time().unwrap().to_string());
        assert_eq!("12:18:33", report.solar_noon.time().unwrap().to_string());
        assert_eq!("05:22:43", report.civil_dawn.time().unwrap().to_string());
        assert_eq!("19:14:23", report.civil_dusk.time().unwrap().to_string());
        assert_eq!("04:37:42", report.nautical_dawn.time().unwrap().to_string());
        assert_eq!("19:59:24", report.nautical_dusk.time().unwrap().to_string());
        assert_eq!("03:49:09", report.astronomical_dawn.time().unwrap().to_string());
        assert_eq!("20:47:57", report.astronomical_dusk.time().unwrap().to_string());

        // mid-summer (there is no true night; it stays astronomical twilight)
        let date = DateTime::parse_from_rfc3339("2020-06-21T12:00:00+01:00").unwrap();
        let coordinates =
        Coordinates {
            latitude: Latitude::new(55.9533).unwrap(),
            longitude: Longitude::new(-3.1883).unwrap(),
        };

        let calcs = calc::SolarCalculations::new(date, coordinates);
        let report = SolarReport::new(calcs);

        assert_eq!("04:26:26", report.sunrise.time().unwrap().to_string());
        assert_eq!("22:02:52", report.sunset.time().unwrap().to_string());
        assert_eq!("13:14:39", report.solar_noon.time().unwrap().to_string());
        assert_eq!("03:23:57", report.civil_dawn.time().unwrap().to_string());
        assert_eq!("23:05:20", report.civil_dusk.time().unwrap().to_string());
        assert_eq!(None, report.nautical_dawn.0);
        assert_eq!("Never".to_string(), format!("{}", report.nautical_dawn));
        assert_eq!(None, report.nautical_dusk.0);
        assert_eq!("Never".to_string(), format!("{}", report.nautical_dusk));
        assert_eq!(None, report.astronomical_dawn.0);
        assert_eq!("Never".to_string(), format!("{}", report.astronomical_dawn));
        assert_eq!(None, report.astronomical_dusk.0);
        assert_eq!("Never".to_string(), format!("{}", report.astronomical_dusk));

        // now try with a non-zero time zone
        let date = DateTime::parse_from_rfc3339("2020-03-30T12:00:00+01:00").unwrap();
        let coordinates =
        Coordinates {
            latitude: Latitude::new(55.9533).unwrap(),
            longitude: Longitude::new(-3.1883).unwrap(),
        };

        let calcs = calc::SolarCalculations::new(date, coordinates);
        let report = SolarReport::new(calcs);

        assert_eq!("06:47:03", report.sunrise.time().unwrap().to_string());
        assert_eq!("19:47:03", report.sunset.time().unwrap().to_string());
        assert_eq!("13:17:03", report.solar_noon.time().unwrap().to_string());
        assert_eq!("06:09:13", report.civil_dawn.time().unwrap().to_string());
        assert_eq!("20:24:53", report.civil_dusk.time().unwrap().to_string());
        assert_eq!("05:23:09", report.nautical_dawn.time().unwrap().to_string());
        assert_eq!("21:10:57", report.nautical_dusk.time().unwrap().to_string());
        assert_eq!("04:32:31", report.astronomical_dawn.time().unwrap().to_string());
        assert_eq!("22:01:36", report.astronomical_dusk.time().unwrap().to_string());

        // at an extreme longitude with a very non-local timezone
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = Coordinates {
            latitude: Latitude::new(55.9533).unwrap(),
            longitude: Longitude::new(-174.0).unwrap(),
        }
        ;

        let calcs = calc::SolarCalculations::new(date, coordinates);
        let report = SolarReport::new(calcs);

        assert_eq!("2020-03-25 17:23:21 +00:00", report.sunrise.0.unwrap().to_string());
        assert_eq!("2020-03-26 06:00:14 +00:00", report.sunset.0.unwrap().to_string());
        assert_eq!("2020-03-25 23:41:48 +00:00", report.solar_noon.to_string());
        assert_eq!("2020-03-25 16:45:58 +00:00", report.civil_dawn.0.unwrap().to_string());
        assert_eq!("2020-03-26 06:37:37 +00:00", report.civil_dusk.0.unwrap().to_string());
        assert_eq!("2020-03-25 16:00:57 +00:00", report.nautical_dawn.0.unwrap().to_string());
        assert_eq!("2020-03-26 07:22:39 +00:00", report.nautical_dusk.0.unwrap().to_string());
        assert_eq!("2020-03-25 15:12:24 +00:00", report.astronomical_dawn.0.unwrap().to_string());
        assert_eq!("2020-03-26 08:11:12 +00:00", report.astronomical_dusk.0.unwrap().to_string());

        // an extreme northern latitude during the summer
        let date = DateTime::parse_from_rfc3339("2020-06-21T12:00:00+02:00").unwrap();
        let coordinates = Coordinates {
            latitude: Latitude::new(78.22).unwrap(),
            longitude: Longitude::new(15.635).unwrap(),
        };

        let calcs = calc::SolarCalculations::new(date, coordinates);
        let report = SolarReport::new(calcs);

        assert_eq!(None, report.sunrise.0);
        assert_eq!(None, report.sunset.0);
        assert_eq!("12:59:21", report.solar_noon.time().unwrap().to_string());
        assert_eq!(None, report.civil_dawn.0);
        assert_eq!(None, report.civil_dusk.0);
        assert_eq!(None, report.nautical_dawn.0);
        assert_eq!(None, report.nautical_dusk.0);
        assert_eq!(None, report.astronomical_dawn.0);
        assert_eq!(None, report.astronomical_dusk.0);
        assert_eq!("24h 0m 0s", SolarReport::day_length_hms(report.day_length));
    }

    #[test]
    fn test_json_output_format() {
        // validated against NOAA calculations https://www.esrl.noaa.gov/gmd/grad/solcalc/calcdetails.html
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = Coordinates {
            latitude: Latitude::new(55.9533).unwrap(),
            longitude: Longitude::new(-3.1883).unwrap(),
        };
        let calcs = calc::SolarCalculations::new(date, coordinates);

        let report = SolarReport::new(calcs);

        let expected = serde_json::json!({
            "location": {"latitude": 55.9533, "longitude": -3.1883},
            "date": "2020-03-25T12:00:00+00:00",
            "day_length": 45412,
            "solar_noon": "2020-03-25T12:18:33+00:00",
            "sunrise": "2020-03-25T06:00:07+00:00",
            "sunset": "2020-03-25T18:36:59+00:00",
            "dawn": {"civil": "2020-03-25T05:22:43+00:00", "nautical": "2020-03-25T04:37:42+00:00", "astronomical": "2020-03-25T03:49:09+00:00"},
            "dusk": {"civil": "2020-03-25T19:14:23+00:00", "nautical": "2020-03-25T19:59:24+00:00", "astronomical": "2020-03-25T20:47:57+00:00"},
        });

        assert_eq!(serde_json::to_value(report).unwrap(), expected);
    }
    #[test]
    fn test_json_output_format_with_null() {
        // validated against NOAA calculations https://www.esrl.noaa.gov/gmd/grad/solcalc/calcdetails.html
        let date = DateTime::parse_from_rfc3339("2022-06-11T12:00:00+01:00").unwrap();
        let coordinates = Coordinates {
            latitude: Latitude::new(51.4000).unwrap(),
            longitude: Longitude::new(-5.4670).unwrap(),
        };
        let calcs = calc::SolarCalculations::new(date, coordinates);

        let report = SolarReport::new(calcs);

        let expected = serde_json::json!({
            "location": {"latitude": 51.4000, "longitude": -5.4670},
            "date": "2022-06-11T12:00:00+01:00",
            "day_length": 59534,
            "solar_noon": "2022-06-11T13:21:31+01:00",
            "sunrise": "2022-06-11T05:05:24+01:00",
            "sunset": "2022-06-11T21:37:38+01:00",
            "dawn": {"civil": "2022-06-11T04:18:29+01:00", "nautical": "2022-06-11T03:06:40+01:00", "astronomical": null},
            "dusk": {"civil": "2022-06-11T22:24:34+01:00", "nautical": "2022-06-11T23:36:23+01:00", "astronomical": null},
        });

        assert_eq!(serde_json::to_value(report).unwrap(), expected);
    }
}
