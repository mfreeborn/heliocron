use chrono::{DateTime, FixedOffset, TimeZone};
use criterion::{criterion_group, criterion_main, Criterion};
use heliocron::{report, structs};

fn run_report(date: DateTime<FixedOffset>, coordinates: structs::Coordinates) -> () {
    report::SolarReport::new(date, coordinates);
}

fn criterion_benchmark(c: &mut Criterion) {
    // set up parameters
    let date = FixedOffset::east(0).ymd(2020, 2, 25).and_hms(12, 0, 0);

    let coordinates = structs::Coordinates {
        latitude: structs::Latitude { value: 51.0 },
        longitude: structs::Longitude { value: 4.0 },
    };

    c.bench_function("run_report", |b| b.iter(|| run_report(date, coordinates)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
