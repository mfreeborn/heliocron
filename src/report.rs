#[path = "./traits.rs"]
mod traits;

use std::fmt;

use chrono::{DateTime, Duration, FixedOffset, Local, NaiveTime, Offset, TimeZone, Timelike};

use super::{enums, structs, structs::Coordinate};
use traits::DateTimeExt;

#[derive(Debug)]
pub struct SolarReport {
    // required parameters
    pub date: DateTime<FixedOffset>,
    pub coordinates: structs::Coordinates,

    // these attributes are always calculable
    pub solar_noon: DateTime<FixedOffset>,
    pub day_length: Duration,

    // these attributes are sometimes not valid i.e. at high latitudes or
    // mid-summer when there may never be a particular dawn or dusk on a
    // given day
    pub sunrise: Option<DateTime<FixedOffset>>,
    pub sunset: Option<DateTime<FixedOffset>>,

    pub civil_dawn: Option<DateTime<FixedOffset>>,
    pub civil_dusk: Option<DateTime<FixedOffset>>,

    pub nautical_dawn: Option<DateTime<FixedOffset>>,
    pub nautical_dusk: Option<DateTime<FixedOffset>>,

    pub astronomical_dawn: Option<DateTime<FixedOffset>>,
    pub astronomical_dusk: Option<DateTime<FixedOffset>>,
}

impl Default for SolarReport {
    fn default() -> SolarReport {
        let local_time = Local::now();
        let default_datetime =
            local_time.with_timezone(&FixedOffset::from_offset(local_time.offset()));
        let default_day_length = Duration::seconds(0);
        SolarReport {
            solar_noon: default_datetime,
            sunrise: None,
            sunset: None,
            civil_dawn: None,
            civil_dusk: None,
            nautical_dawn: None,
            nautical_dusk: None,
            astronomical_dawn: None,
            astronomical_dusk: None,
            day_length: default_day_length,
            date: local_time.with_timezone(&FixedOffset::from_offset(local_time.offset())),
            coordinates: structs::Coordinates::from_decimal_degrees("0.0N", "0.0W").unwrap(),
        }
    }
}

impl fmt::Display for SolarReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_str = self.format_report();
        write!(f, "{}", fmt_str)
    }
}

impl SolarReport {
    pub fn new(date: DateTime<FixedOffset>, coordinates: structs::Coordinates) -> SolarReport {
        let mut report = SolarReport {
            date,
            coordinates,
            ..Default::default()
        };

        report.run();

        // make it immutable again by default
        let report = report;
        report
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
        Sunrise is at:            {}\n\
        Solar noon is at:         {}\n\
        Sunset is at:             {}\n\n\
        The day length is:        {}\n\n\
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
            match self.sunrise {
                Some(datetime) => datetime.to_string(),
                None => "Never".to_string(),
            },
            self.solar_noon,
            match self.sunset {
                Some(datetime) => datetime.to_string(),
                None => "Never".to_string(),
            },
            self.day_length_hms(),
            match self.civil_dawn {
                Some(datetime) => datetime.to_string(),
                None => "Never".to_string(),
            },
            match self.civil_dusk {
                Some(datetime) => datetime.to_string(),
                None => "Never".to_string(),
            },
            match self.nautical_dawn {
                Some(datetime) => datetime.to_string(),
                None => "Never".to_string(),
            },
            match self.nautical_dusk {
                Some(datetime) => datetime.to_string(),
                None => "Never".to_string(),
            },
            match self.astronomical_dawn {
                Some(datetime) => datetime.to_string(),
                None => "Never".to_string(),
            },
            match self.astronomical_dusk {
                Some(datetime) => datetime.to_string(),
                None => "Never".to_string(),
            }
        )
    }

    fn calculate_day_length(&self) -> Duration {
        if self.sunrise.is_some() & self.sunset.is_some() {
            self.sunset.unwrap() - self.sunrise.unwrap()
        } else {
            // 24 hours if sunrise and sunset don't occur
            Duration::hours(24)
        }
    }

    fn day_length_hms(&self) -> String {
        let day_length = self.day_length.num_seconds();
        let hours = (day_length / 60) / 60;
        let minutes = (day_length / 60) % 60;
        let seconds = day_length % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    fn day_fraction_to_datetime(&self, mut day_fraction: f64) -> DateTime<FixedOffset> {
        let mut date = self.date;

        // correct the date if the event rolls over to the next day, or happens on the previous day
        if day_fraction < 0.0 {
            date = date - Duration::days(1);
            day_fraction = day_fraction.abs();
        } else if day_fraction >= 1.0 {
            date = date + Duration::days(1);
            day_fraction -= 1.0;
        }

        let hour_fraction = day_fraction * 24.0;
        let minute_fraction = hour_fraction.fract() * 60.0;
        let second_fraction = minute_fraction.fract() * 60.0;
        println!("{}", hour_fraction.trunc());
        println!("{}", minute_fraction);
        println!("{}", second_fraction);

        let time = NaiveTime::from_hms(
            hour_fraction.trunc() as u32,
            minute_fraction.trunc() as u32,
            second_fraction.trunc() as u32,
        );
        println!("{}", time);

        date.with_hour(time.hour())
            .unwrap()
            .with_minute(time.minute())
            .unwrap()
            .with_second(time.second())
            .unwrap()
    }

    fn calculate_hour_angle(
        &self,
        event: Option<enums::TwilightType>,
        solar_declination: f64,
    ) -> f64 {
        let event_angle: f64 = match event {
            None => 90.833,
            Some(enums::TwilightType::Civil) => 96.0,
            Some(enums::TwilightType::Nautical) => 102.0,
            Some(enums::TwilightType::Astronomical) => 108.0,
        };

        (((event_angle.to_radians().cos()
            / (self.coordinates.latitude.to_radians().cos()
                * solar_declination.to_radians().cos()))
            - self.coordinates.latitude.to_radians().tan() * solar_declination.to_radians().tan())
        .acos())
        .to_degrees()
    }

    fn calculate_event_start_and_end(
        &self,
        twilight_type: Option<enums::TwilightType>,
        solar_noon: f64,
        solar_declination: f64,
    ) -> (Option<DateTime<FixedOffset>>, Option<DateTime<FixedOffset>>) {
        let hour_angle = self.calculate_hour_angle(twilight_type, solar_declination);

        if hour_angle.is_nan() {
            return (None, None);
        }

        let start_fraction = solar_noon - (hour_angle * 4.0) / 1440.0;
        let end_fraction = solar_noon + (hour_angle * 4.0) / 1440.0;

        let start_time = self.day_fraction_to_datetime(start_fraction);
        let end_time = self.day_fraction_to_datetime(end_fraction);

        println!("start: {:?}\nend:   {:?}\n", start_time, end_time);
        (Some(start_time), Some(end_time))
    }

    fn run(&mut self) {
        let time_zone = self.date.offset().fix().local_minus_utc() as f64 / 3600.0;

        let julian_date: f64 = self.date.to_julian_date();

        let julian_century = (julian_date - 2451545.0) / 36525.0;

        let geometric_solar_mean_longitude =
            (280.46646 + julian_century * (36000.76983 + julian_century * 0.0003032)) % 360.0;

        let solar_mean_anomaly =
            357.52911 + julian_century * (35999.05029 - 0.0001537 * julian_century);

        let eccent_earth_orbit =
            0.016708634 - julian_century * (0.000042037 + 0.0000001267 * julian_century);

        let equation_of_the_center = solar_mean_anomaly.to_radians().sin()
            * (1.914602 - julian_century * (0.004817 + 0.000014 * julian_century))
            + (2.0 * solar_mean_anomaly).to_radians().sin()
                * (0.019993 - 0.000101 * julian_century)
            + (3.0 * solar_mean_anomaly).to_radians().sin() * 0.000289;

        let solar_true_longitude = geometric_solar_mean_longitude + equation_of_the_center;

        let solar_apparent_longitude = solar_true_longitude
            - 0.00569
            - 0.00478 * (125.04 - 1934.136 * julian_century).to_radians().sin();

        let mean_oblique_ecliptic = 23.0
            + (26.0
                + (21.448
                    - julian_century
                        * (46.815 + julian_century * (0.00059 - julian_century * 0.001813)))
                    / 60.0)
                / 60.0;

        let oblique_corrected = mean_oblique_ecliptic
            + 0.00256 * (125.04 - 1934.136 * julian_century).to_radians().cos();

        let solar_declination = ((oblique_corrected.to_radians().sin()
            * solar_apparent_longitude.to_radians().sin())
        .asin())
        .to_degrees();

        let var_y = (oblique_corrected / 2.0).to_radians().tan().powi(2);

        let equation_of_time = 4.0
            * (var_y * (geometric_solar_mean_longitude.to_radians() * 2.0).sin()
                - 2.0 * eccent_earth_orbit * solar_mean_anomaly.to_radians().sin()
                + 4.0
                    * eccent_earth_orbit
                    * var_y
                    * solar_mean_anomaly.to_radians().sin()
                    * (geometric_solar_mean_longitude.to_radians() * 2.0).cos()
                - 0.5 * var_y * var_y * (geometric_solar_mean_longitude.to_radians() * 4.0).sin()
                - 1.25
                    * eccent_earth_orbit
                    * eccent_earth_orbit
                    * (solar_mean_anomaly.to_radians() * 2.0).sin())
            .to_degrees();

        let solar_noon = (720.0 - 4.0 * self.coordinates.longitude.value - equation_of_time
            + time_zone * 60.0)
            / 1440.0;

        // plain sunrise/sunset
        let (sunrise, sunset) =
            self.calculate_event_start_and_end(None, solar_noon, solar_declination);

        // civil twilight
        let (civil_twilight_start, civil_twilight_end) = self.calculate_event_start_and_end(
            Some(enums::TwilightType::Civil),
            solar_noon,
            solar_declination,
        );

        // nautical twilight
        let (nautical_twilight_start, nautical_twilight_end) = self.calculate_event_start_and_end(
            Some(enums::TwilightType::Nautical),
            solar_noon,
            solar_declination,
        );

        // astronomical twilight
        let (astronomical_twilight_start, astronomical_twilight_end) = self
            .calculate_event_start_and_end(
                Some(enums::TwilightType::Astronomical),
                solar_noon,
                solar_declination,
            );

        self.solar_noon = self.day_fraction_to_datetime(solar_noon);

        self.sunrise = sunrise;
        self.sunset = sunset;

        self.civil_dawn = civil_twilight_start;
        self.civil_dusk = civil_twilight_end;

        self.nautical_dawn = nautical_twilight_start;
        self.nautical_dusk = nautical_twilight_end;

        self.astronomical_dawn = astronomical_twilight_start;
        self.astronomical_dusk = astronomical_twilight_end;

        self.day_length = self.calculate_day_length();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_report_new() {
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = structs::Coordinates {
            latitude: structs::Latitude { value: 0.0 },
            longitude: structs::Longitude { value: 0.0 },
        };
        // Default trait should handle the rest
        let _new_report = SolarReport::new(date, coordinates);
    }

    #[test]
    fn test_report_content() {
        // checks that the report contains all the corrent metrics
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = structs::Coordinates {
            latitude: structs::Latitude { value: 0.0 },
            longitude: structs::Longitude { value: 0.0 },
        };
        let report = SolarReport::new(date, coordinates);
        let report_str = report.format_report();

        assert!(report_str.contains("LOCATION"));
        assert!(report_str.contains("DATE"));
        assert!(report_str.contains("Sunrise is at"));
        assert!(report_str.contains("Solar noon is at"));
        assert!(report_str.contains("Sunset is at"));
        assert!(report_str.contains("The day length is"));

        let sunrise_str = format!("{}", report.sunrise.unwrap());
        assert!(report_str.contains(&sunrise_str));

        let solar_noon_str = format!("{}", report.solar_noon);
        assert!(report_str.contains(&solar_noon_str));

        let sunset_str = format!("{}", report.sunset.unwrap());
        assert!(report_str.contains(&sunset_str));

        let day_length_str = format!("{}", report.day_length_hms());
        assert!(report_str.contains(&day_length_str));
    }

    #[test]
    fn test_sunrise_sunset() {
        // validated against NOAA calculations https://www.esrl.noaa.gov/gmd/grad/solcalc/calcdetails.html
        // ~Springtime
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates =
            structs::Coordinates::from_decimal_degrees("55.9533N", "3.1883W").unwrap();
        let mut report = SolarReport {
            date,
            coordinates,
            solar_noon: date,
            sunrise: None,
            sunset: None,
            civil_dawn: None,
            civil_dusk: None,
            nautical_dawn: None,
            nautical_dusk: None,
            astronomical_dawn: None,
            astronomical_dusk: None,
            day_length: Duration::seconds(0),
        };

        report.run();
        assert_eq!("06:00:07", report.sunrise.unwrap().time().to_string());
        assert_eq!("18:36:59", report.sunset.unwrap().time().to_string());
        assert_eq!("12:18:33", report.solar_noon.time().to_string());
        assert_eq!("05:22:43", report.civil_dawn.unwrap().time().to_string());
        assert_eq!("19:14:23", report.civil_dusk.unwrap().time().to_string());
        assert_eq!("04:37:42", report.nautical_dawn.unwrap().time().to_string());
        assert_eq!("19:59:24", report.nautical_dusk.unwrap().time().to_string());
        assert_eq!(
            "03:49:09",
            report.astronomical_dawn.unwrap().time().to_string()
        );
        assert_eq!(
            "20:47:57",
            report.astronomical_dusk.unwrap().time().to_string()
        );

        // mid-summer (there is no true night; it stays astronomical twilight)
        let date = DateTime::parse_from_rfc3339("2020-06-21T12:00:00+01:00").unwrap();
        let coordinates =
            structs::Coordinates::from_decimal_degrees("55.9533N", "3.1883W").unwrap();
        let mut report = SolarReport {
            date,
            coordinates,
            solar_noon: date,
            sunrise: None,
            sunset: None,
            civil_dawn: None,
            civil_dusk: None,
            nautical_dawn: None,
            nautical_dusk: None,
            astronomical_dawn: None,
            astronomical_dusk: None,
            day_length: Duration::seconds(0),
        };

        report.run();
        assert_eq!("04:26:26", report.sunrise.unwrap().time().to_string());
        assert_eq!("22:02:52", report.sunset.unwrap().time().to_string());
        assert_eq!("13:14:39", report.solar_noon.time().to_string());
        assert_eq!("03:23:57", report.civil_dawn.unwrap().time().to_string());
        assert_eq!("23:05:20", report.civil_dusk.unwrap().time().to_string());
        assert_eq!(None, report.nautical_dawn);
        assert_eq!(None, report.nautical_dusk);
        assert_eq!(None, report.astronomical_dawn);
        assert_eq!(None, report.astronomical_dusk);

        // now try with a non-zero time zone
        let date = DateTime::parse_from_rfc3339("2020-03-30T12:00:00+01:00").unwrap();
        let coordinates =
            structs::Coordinates::from_decimal_degrees("55.9533N", "3.1883W").unwrap();
        let mut report = SolarReport {
            date,
            coordinates,
            solar_noon: date,
            sunrise: None,
            sunset: None,
            civil_dawn: None,
            civil_dusk: None,
            nautical_dawn: None,
            nautical_dusk: None,
            astronomical_dawn: None,
            astronomical_dusk: None,
            day_length: Duration::seconds(0),
        };

        report.run();
        assert_eq!("06:47:03", report.sunrise.unwrap().time().to_string());
        assert_eq!("19:47:03", report.sunset.unwrap().time().to_string());
        assert_eq!("13:17:03", report.solar_noon.time().to_string());
        assert_eq!("06:09:13", report.civil_dawn.unwrap().time().to_string());
        assert_eq!("20:24:53", report.civil_dusk.unwrap().time().to_string());
        assert_eq!("05:23:09", report.nautical_dawn.unwrap().time().to_string());
        assert_eq!("21:10:57", report.nautical_dusk.unwrap().time().to_string());
        assert_eq!(
            "04:32:31",
            report.astronomical_dawn.unwrap().time().to_string()
        );
        assert_eq!(
            "22:01:36",
            report.astronomical_dusk.unwrap().time().to_string()
        );

        // at an extreme longitude with a very non-local timezone
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = structs::Coordinates::from_decimal_degrees("55.9533N", "174.0W").unwrap();
        let mut report = SolarReport {
            date,
            coordinates,
            solar_noon: date,
            sunrise: None,
            sunset: None,
            civil_dawn: None,
            civil_dusk: None,
            nautical_dawn: None,
            nautical_dusk: None,
            astronomical_dawn: None,
            astronomical_dusk: None,
            day_length: Duration::seconds(0),
        };

        report.run();
        assert_eq!(
            "2020-03-25 17:23:21 +00:00",
            report.sunrise.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-26 06:00:14 +00:00",
            report.sunset.unwrap().to_string()
        );
        assert_eq!("2020-03-25 23:41:48 +00:00", report.solar_noon.to_string());
        assert_eq!(
            "2020-03-25 16:45:58 +00:00",
            report.civil_dawn.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-26 06:37:37 +00:00",
            report.civil_dusk.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-25 16:00:57 +00:00",
            report.nautical_dawn.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-26 07:22:39 +00:00",
            report.nautical_dusk.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-25 15:12:24 +00:00",
            report.astronomical_dawn.unwrap().to_string()
        );
        assert_eq!(
            "2020-03-26 08:11:12 +00:00",
            report.astronomical_dusk.unwrap().to_string()
        );

        // an extreme northern latitude during the summer
        let date = DateTime::parse_from_rfc3339("2020-06-21T12:00:00+02:00").unwrap();
        let coordinates = structs::Coordinates::from_decimal_degrees("78.22N", "15.635E").unwrap();
        let mut report = SolarReport {
            date,
            coordinates,
            solar_noon: date,
            sunrise: None,
            sunset: None,
            civil_dawn: None,
            civil_dusk: None,
            nautical_dawn: None,
            nautical_dusk: None,
            astronomical_dawn: None,
            astronomical_dusk: None,
            day_length: Duration::seconds(0),
        };

        report.run();
        assert_eq!(None, report.sunrise);
        assert_eq!(None, report.sunset);
        assert_eq!("12:59:21", report.solar_noon.time().to_string());
        assert_eq!(None, report.civil_dawn);
        assert_eq!(None, report.civil_dusk);
        assert_eq!(None, report.nautical_dawn);
        assert_eq!(None, report.nautical_dusk);
        assert_eq!(None, report.astronomical_dawn);
        assert_eq!(None, report.astronomical_dusk);
        assert_eq!("24:00:00", report.day_length_hms());
    }

    #[test]
    fn test_day_fraction_to_time_underoverflow() {
        // when a location is selected which is in a different time zone, it is possible for the sunrise/sunset to
        // occur either the following or previous day. This results in a day fraction which is either negative
        // or >= 1
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = structs::Coordinates::from_decimal_degrees("0.0N", "0.0W").unwrap();
        let report = SolarReport::new(date, coordinates);

        let params = [
            ("2020-03-26 12:00:00 +00:00", 1.5),
            ("2020-03-24 12:00:00 +00:00", -0.5),
        ];

        for (expected_time, arg) in params.iter() {
            let result = report.day_fraction_to_datetime(*arg);
            assert_eq!(*expected_time, result.to_string());
        }
    }

    #[test]
    fn test_day_fraction_to_time() {
        let date = DateTime::parse_from_rfc3339("2020-03-25T12:00:00+00:00").unwrap();
        let coordinates = structs::Coordinates::from_decimal_degrees("0.0N", "0.0W").unwrap();
        let report = SolarReport::new(date, coordinates);
        let params = [
            ("2020-03-25 00:00:00 +00:00", 0.0),
            ("2020-03-25 12:00:00 +00:00", 0.5),
            ("2020-03-25 23:59:59 +00:00", 0.99999),
            ("2020-03-25 01:23:45 +00:00", 0.05816),
            ("2020-03-25 23:42:12 +00:00", 0.987639),
        ];

        for (expected, arg) in params.iter() {
            let result = report.day_fraction_to_datetime(*arg).to_string();
            assert_eq!(*expected, result);
        }
    }
}
