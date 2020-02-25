use chrono::{FixedOffset, TimeZone};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sunwait::report;

fn run_report(report: &mut report::SolarReport) -> () {
    report.run()
}

fn criterion_benchmark(c: &mut Criterion) {
    // set up parameters
    let date = FixedOffset::east(0).ymd(2020, 2, 25).and_hms(12, 0, 0);
    let lat = 51.0;
    let lon = 4.0;

    let mut rep = report::SolarReport::new(date, lat, lon);

    c.bench_function("run_report", |b| b.iter(|| run_report(black_box(&mut rep))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
