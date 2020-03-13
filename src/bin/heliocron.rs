use std::process;

use heliocron::{config, errors, report, subcommands};

fn run_heliocron() -> Result<(), errors::HeliocronError> {
    let config = config::get_config()?;

    let report = report::SolarReport::new(config.date, config.coordinates);

    match config.subcommand {
        Some(config::Subcommand::Report {}) => subcommands::display_report(report),
        Some(config::Subcommand::Wait { offset, event }) => {
            subcommands::wait(offset?, report, event?)
        }
        // will never match None as this is caught earlier by StructOpt
        None => println!("No subcommand provided!"),
    }
    Ok(())
}

fn main() {
    process::exit(match run_heliocron() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
