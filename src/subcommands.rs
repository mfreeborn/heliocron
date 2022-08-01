use std::io::Write;
use std::result;

use chrono::{Duration, Local};
use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};

use super::{calc, domain, errors, report, utils};

type Result<T> = result::Result<T, errors::HeliocronError>;

pub fn display_report(solar_calculations: calc::SolarCalculations, json: bool) -> Result<()> {
    let report = report::Report::new(solar_calculations);
    let output = if json {
        serde_json::to_string(&report).unwrap()
    } else {
        report.to_string()
    };
    println!("{}", output);
    Ok(())
}

pub async fn wait(
    event: domain::Event,
    offset: Duration,
    solar_calculations: calc::SolarCalculations,
    run_missed_task: bool,
) -> Result<()> {
    let event_time = solar_calculations.event_time(event);

    match event_time.0 {
        Some(datetime) => {
            let wait_until = datetime + offset;
            utils::wait(wait_until).await?;

            // If the device running heliocron is asleep for whetever reason, it is possible that this future
            // will return after `wait_until`. As such, we need to handle whether to run or skip the task
            // if the event was missed. We allow a default tolerance of 30s, which should be more than enough to
            // catch any scheduling delays that could cause a second or two's delay. At some point, this arbitrary
            // number could be made configurable, if desired.
            if run_missed_task {
                Ok(())
            } else {
                let now = chrono::Utc::now().with_timezone(wait_until.offset());
                let missed_by = (now - wait_until).num_seconds();
                if missed_by > 30 {
                    Err(errors::HeliocronError::Runtime(
                        errors::RuntimeErrorKind::EventMissed(missed_by),
                    ))
                } else {
                    Ok(())
                }
            }
        }
        None => Err(errors::HeliocronError::Runtime(
            errors::RuntimeErrorKind::NonOccurringEvent,
        )),
    }
}

pub fn poll(solar_calculations: calc::SolarCalculations, watch: bool, json: bool) -> Result<()> {
    let mut report = report::PollReport::new(&solar_calculations);
    let output = if json {
        serde_json::to_string(&report).unwrap()
    } else {
        report.to_string()
    };

    if !watch {
        println!("{output}");
    } else {
        if !json {
            println!("Displaying solar calculations in real time. Press ctrl+C to cancel.\n");
        }

        // Set up stdout and make a record of the current cursor location. We unwrap
        let mut stdout = std::io::stdout();
        stdout.queue(cursor::SavePosition).unwrap();
        stdout.execute(cursor::Hide).unwrap();

        loop {
            if json {
                println!("{}", serde_json::to_string(&report).unwrap());
            } else {
                stdout.queue(cursor::RestorePosition).unwrap();
                stdout
                    .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
                    .unwrap();
                stdout.write_all(report.to_string().as_bytes()).unwrap();
                stdout.flush().unwrap();
            }

            std::thread::sleep(std::time::Duration::from_secs(1));

            let now = Local::now();
            let now = now.with_timezone(now.offset());

            let calcs = solar_calculations.refresh(now);

            report = report::PollReport::new(&calcs);
        }
    }

    Ok(())
}
