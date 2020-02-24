use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveTime, Offset, TimeZone};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    sub_cmd: SubCommand,

    #[structopt(flatten)]
    date_args: DateArgs,

    #[structopt(short = "l", long = "latitude", default_value = "51.0782N", parse(from_str=parse_latlon))]
    latitude: f64,

    #[structopt(short = "o", long = "longitude", default_value = "4.0583W", parse(from_str=parse_latlon))]
    longitude: f64,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    Report {},

    Wait {
        #[structopt(short = "t", long = "offset", default_value = "00:00:00")]
        offset: String,
    },
}

#[derive(Debug, StructOpt)]
struct DateArgs {
    #[structopt(short = "d", long = "date", default_value = "2020-03-22")]
    date: String,

    #[structopt(short = "f", long = "date-format", default_value = "%Y-%m-%d")]
    date_fmt: String,

    #[structopt(short = "t", long = "time-zone")]
    time_zone: Option<String>,
}

fn parse_date(date: &str, date_fmt: &str, time_zone: Option<&str>) -> DateTime<FixedOffset> {
    // default date format
    let time_fmt = "%H:%M:%S";
    let datetime_fmt = format!("{}T{}", date_fmt, time_fmt);

    // customisable date
    // e.g. 2020-02-24
    let date = date;
    let time = "12:00:00";
    let datetime = format!("{}T{}", date, time);

    // default time zone is the local time zone at the given date
    let time_zone = match time_zone {
        Some(tz) => tz.to_string(),
        None => Local
            .datetime_from_str(&datetime, &datetime_fmt)
            .expect("Error parsing date!")
            .offset()
            .to_string(),
    };
    let datetimetz = format!("{}{}", datetime, time_zone);
    let datetimetz_fmt = format!("{}%:z", datetime_fmt);

    DateTime::parse_from_str(&datetimetz, &datetimetz_fmt).expect("Error parsing date!")
}

fn parse_latlon(latlon: &str) -> f64 {
    // W and S should be negative
    let compass_direction: &str = &latlon[latlon.len() - 1..].to_lowercase();

    let compass_correction = match compass_direction {
        "n" | "e" => 1.0,
        "w" | "s" => -1.0,
        _ => panic!("Expected latitude/longitude to end with one of: N, S, E, W"),
    };

    let latlon: &f64 = &latlon[..latlon.len() - 1]
        .parse()
        .expect("Error, float expected!");

    latlon * compass_correction
}

pub trait DateTimeExt {
    fn to_julian_date(&self) -> f64;
}

impl<Tz: TimeZone> DateTimeExt for DateTime<Tz> {
    fn to_julian_date(&self) -> f64 {
        let (year, month, day): (i32, i32, i32) =
            (self.year(), self.month() as i32, self.day() as i32);

        (367 * year - 7 * (year + (month + 9) / 12) / 4 + 275 * month / 9 + day + 1721014) as f64
    }
}
#[derive(Debug)]
struct SolarReport {
    solar_noon: NaiveTime,
    sunrise: NaiveTime,
    sunset: NaiveTime,

    date: DateTime<FixedOffset>,
    latitude: f64,
    longitude: f64,
}

impl Default for SolarReport {
    fn default() -> SolarReport {
        SolarReport {
            solar_noon: NaiveTime::from_hms(0, 0, 0),
            sunrise: NaiveTime::from_hms(0, 0, 0),
            sunset: NaiveTime::from_hms(0, 0, 0),
            date: Local::now().with_timezone(&FixedOffset::from_offset(Local::now().offset())),
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

impl std::fmt::Display for SolarReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_str = format!(
            "LOCATION\n\
        --------\n\
        Latitude:  {}\n\
        Longitude: {}\n\n\
        DATE\n\
        ----\n\
        {}\n\n\
        Sunrise on this day is at {}\n\
        Sunset on this day is at  {}\n\
        Solar noon is at          {}",
            self.latitude, self.longitude, self.date, self.sunrise, self.sunset, self.solar_noon
        );

        write!(f, "{}", fmt_str)
    }
}

impl SolarReport {
    fn new(date: DateTime<FixedOffset>, latitude: f64, longitude: f64) -> SolarReport {
        let mut report = SolarReport {
            date,
            latitude,
            longitude,
            ..Default::default()
        };

        report.run();

        // make it immutable again by default
        let report = report;
        report
    }

    fn day_fraction_to_time(day_fraction: f64) -> NaiveTime {
        // day_fraction should be 0 <= day_fraction < 1
        // let day_fraction = day.fract();
        let hour_fraction = day_fraction * 24.0;
        let minute_fraction = hour_fraction.fract() * 60.0;
        let second_fraction = minute_fraction.fract() * 60.0;

        NaiveTime::from_hms(
            hour_fraction.trunc() as u32,
            minute_fraction.trunc() as u32,
            second_fraction.trunc() as u32,
        )
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

        let hour_angle = (((90.833f64.to_radians().cos()
            / (self.latitude.to_radians().cos() * solar_declination.to_radians().cos()))
            - self.latitude.to_radians().tan() * solar_declination.to_radians().tan())
        .acos())
        .to_degrees();

        let solar_noon =
            (720.0 - 4.0 * self.longitude - equation_of_time + time_zone * 60.0) / 1440.0;

        self.sunrise = SolarReport::day_fraction_to_time(
            ((solar_noon - hour_angle * 4.0) / 1440.0) + solar_noon,
        );
        self.sunset = SolarReport::day_fraction_to_time(
            ((solar_noon + hour_angle * 4.0) / 1440.0) + solar_noon,
        );
        self.solar_noon = SolarReport::day_fraction_to_time(solar_noon)
    }
}

fn main() {
    let args = Cli::from_args();

    let date = parse_date(
        &args.date_args.date,
        &args.date_args.date_fmt,
        args.date_args.time_zone.as_deref(),
    );

    let latitude: f64 = args.latitude;
    let longitude: f64 = args.longitude;

    let report = SolarReport::new(date, latitude, longitude);

    match Cli::from_args().sub_cmd {
        SubCommand::Report {} => println!("{}", report),
        SubCommand::Wait { offset } => println!("We need to wait for this long: {}", offset),
    }
}
