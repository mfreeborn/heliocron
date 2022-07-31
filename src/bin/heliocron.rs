use std::process;

use heliocron::{calc, cli, domain, errors, subcommands};

async fn run_heliocron() -> Result<(), errors::HeliocronError> {
    let config = cli::parse_config()?;
    let solar_calculations = calc::SolarCalculations::new(config.date, config.coordinates);

    match config.action {
        domain::Action::Report { json } => subcommands::display_report(solar_calculations, json)?,
        domain::Action::Wait {
            event,
            offset,
            run_missed_task,
        } => subcommands::wait(event, offset, solar_calculations, run_missed_task).await?,
        domain::Action::Poll => subcommands::poll(solar_calculations)?,
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // returns 0 if execution completes successfully, otherwise it prints the error and returns 1
    process::exit(match run_heliocron().await {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
