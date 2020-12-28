use std::fmt;

use chrono::{DateTime, Duration, FixedOffset};

use super::{
    calc, enums,
    structs::{Coordinates, EventTime},
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

impl SolarReport {
    pub fn new(solar_calculations: calc::SolarCalculations) -> SolarReport {
        // we can unwrap all of these safely because they have been manually validated against the Events::new constructor
        let sunrise =
            solar_calculations.calculate_event_time(enums::Event::new("sunrise", None).unwrap());
        let sunset =
            solar_calculations.calculate_event_time(enums::Event::new("sunset", None).unwrap());
        let civil_dawn =
            solar_calculations.calculate_event_time(enums::Event::new("civil_dawn", None).unwrap());
        let civil_dusk =
            solar_calculations.calculate_event_time(enums::Event::new("civil_dusk", None).unwrap());
        let nautical_dawn = solar_calculations
            .calculate_event_time(enums::Event::new("nautical_dawn", None).unwrap());
        let nautical_dusk = solar_calculations
            .calculate_event_time(enums::Event::new("nautical_dusk", None).unwrap());
        let astronomical_dawn = solar_calculations
            .calculate_event_time(enums::Event::new("astronomical_dawn", None).unwrap());
        let astronomical_dusk = solar_calculations
            .calculate_event_time(enums::Event::new("astronomical_dusk", None).unwrap());
        SolarReport {
            date: solar_calculations.date,
            coordinates: solar_calculations.coordinates,
            solar_noon: solar_calculations.get_solar_noon(),
            day_length: solar_calculations.calculate_day_length(),
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
        {}\n\
        {}\n\n\
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
    use super::*;
    use crate::structs;

    #[test]
    fn test_solar_report_new() {
        // check that a 'new' method is defined and takes a coordinate and a date as parameters
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = structs::Coordinates {
            latitude: structs::Latitude { value: 0.0 },
            longitude: structs::Longitude { value: 0.0 },
        };

        let calcs = calc::SolarCalculations::new(date, coordinates);
        // Default trait should handle the rest of the parameters
        let _new_report = SolarReport::new(calcs);
    }

    #[test]
    fn test_report_content() {
        // check that the report contains all the correct metrics
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = structs::Coordinates {
            latitude: structs::Latitude { value: 0.0 },
            longitude: structs::Longitude { value: 0.0 },
        };

        let calcs = calc::SolarCalculations::new(date, coordinates);

        let report = SolarReport::new(calcs);
        let report_str = report.format_report();

        assert!(report_str.contains("LOCATION"));
        assert!(report_str.contains("DATE"));
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

        let day_length_str = format!("{}", SolarReport::day_length_hms(report.day_length));
        assert!(report_str.contains(&day_length_str));
    }

    #[test]
    fn test_sunrise_sunset() {
        // validated against NOAA calculations https://www.esrl.noaa.gov/gmd/grad/solcalc/calcdetails.html
        // ~Springtime
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates =
            structs::Coordinates::from_decimal_degrees("55.9533N", "3.1883W").unwrap();
        let calcs = calc::SolarCalculations::new(date, coordinates);

        let report = SolarReport::new(calcs);

        assert_eq!("06:00:07", report.sunrise.time().unwrap().to_string());
        assert_eq!("18:36:59", report.sunset.time().unwrap().to_string());
        assert_eq!("12:18:33", report.solar_noon.time().unwrap().to_string());
        assert_eq!("05:22:43", report.civil_dawn.time().unwrap().to_string());
        assert_eq!("19:14:23", report.civil_dusk.time().unwrap().to_string());
        assert_eq!("04:37:42", report.nautical_dawn.time().unwrap().to_string());
        assert_eq!("19:59:24", report.nautical_dusk.time().unwrap().to_string());
        assert_eq!(
            "03:49:09",
            report.astronomical_dawn.time().unwrap().to_string()
        );
        assert_eq!(
            "20:47:57",
            report.astronomical_dusk.time().unwrap().to_string()
        );

        // mid-summer (there is no true night; it stays astronomical twilight)
        let date = DateTime::parse_from_rfc3339("2020-06-21T12:00:00+01:00").unwrap();
        let coordinates =
            structs::Coordinates::from_decimal_degrees("55.9533N", "3.1883W").unwrap();

        let calcs = calc::SolarCalculations::new(date, coordinates);
        let report = SolarReport::new(calcs);

        assert_eq!("04:26:26", report.sunrise.time().unwrap().to_string());
        assert_eq!("22:02:52", report.sunset.time().unwrap().to_string());
        assert_eq!("13:14:39", report.solar_noon.time().unwrap().to_string());
        assert_eq!("03:23:57", report.civil_dawn.time().unwrap().to_string());
        assert_eq!("23:05:20", report.civil_dusk.time().unwrap().to_string());
        assert_eq!(None, report.nautical_dawn.datetime);
        assert_eq!("Never".to_string(), format!("{}", report.nautical_dawn));
        assert_eq!(None, report.nautical_dusk.datetime);
        assert_eq!("Never".to_string(), format!("{}", report.nautical_dusk));
        assert_eq!(None, report.astronomical_dawn.datetime);
        assert_eq!("Never".to_string(), format!("{}", report.astronomical_dawn));
        assert_eq!(None, report.astronomical_dusk.datetime);
        assert_eq!("Never".to_string(), format!("{}", report.astronomical_dusk));

        // now try with a non-zero time zone
        let date = DateTime::parse_from_rfc3339("2020-03-30T12:00:00+01:00").unwrap();
        let coordinates =
            structs::Coordinates::from_decimal_degrees("55.9533N", "3.1883W").unwrap();

        let calcs = calc::SolarCalculations::new(date, coordinates);
        let report = SolarReport::new(calcs);

        assert_eq!("06:47:03", report.sunrise.time().unwrap().to_string());
        assert_eq!("19:47:03", report.sunset.time().unwrap().to_string());
        assert_eq!("13:17:03", report.solar_noon.time().unwrap().to_string());
        assert_eq!("06:09:13", report.civil_dawn.time().unwrap().to_string());
        assert_eq!("20:24:53", report.civil_dusk.time().unwrap().to_string());
        assert_eq!("05:23:09", report.nautical_dawn.time().unwrap().to_string());
        assert_eq!("21:10:57", report.nautical_dusk.time().unwrap().to_string());
        assert_eq!(
            "04:32:31",
            report.astronomical_dawn.time().unwrap().to_string()
        );
        assert_eq!(
            "22:01:36",
            report.astronomical_dusk.time().unwrap().to_string()
        );

        // at an extreme longitude with a very non-local timezone
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = structs::Coordinates::from_decimal_degrees("55.9533N", "174.0W").unwrap();

        let calcs = calc::SolarCalculations::new(date, coordinates);
        let report = SolarReport::new(calcs);

        assert_eq!(
            "2020-03-25 17:23:21 +00:00",
            report.sunrise.datetime.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-26 06:00:14 +00:00",
            report.sunset.datetime.unwrap().to_string()
        );
        assert_eq!("2020-03-25 23:41:48 +00:00", report.solar_noon.to_string());
        assert_eq!(
            "2020-03-25 16:45:58 +00:00",
            report.civil_dawn.datetime.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-26 06:37:37 +00:00",
            report.civil_dusk.datetime.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-25 16:00:57 +00:00",
            report.nautical_dawn.datetime.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-26 07:22:39 +00:00",
            report.nautical_dusk.datetime.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-25 15:12:24 +00:00",
            report.astronomical_dawn.datetime.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-26 08:11:12 +00:00",
            report.astronomical_dusk.datetime.unwrap().to_string()
        );

        // an extreme northern latitude during the summer
        let date = DateTime::parse_from_rfc3339("2020-06-21T12:00:00+02:00").unwrap();
        let coordinates = structs::Coordinates::from_decimal_degrees("78.22N", "15.635E").unwrap();

        let calcs = calc::SolarCalculations::new(date, coordinates);
        let report = SolarReport::new(calcs);

        assert_eq!(None, report.sunrise.datetime);
        assert_eq!(None, report.sunset.datetime);
        assert_eq!("12:59:21", report.solar_noon.time().unwrap().to_string());
        assert_eq!(None, report.civil_dawn.datetime);
        assert_eq!(None, report.civil_dusk.datetime);
        assert_eq!(None, report.nautical_dawn.datetime);
        assert_eq!(None, report.nautical_dusk.datetime);
        assert_eq!(None, report.astronomical_dawn.datetime);
        assert_eq!(None, report.astronomical_dusk.datetime);
        assert_eq!("24h 0m 0s", SolarReport::day_length_hms(report.day_length));
    }
}
