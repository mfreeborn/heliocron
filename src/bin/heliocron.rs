use std::process;

use heliocron::{calc, config, errors, subcommands};

async fn run_heliocron() -> Result<(), errors::HeliocronError> {
    // here, we simply take a fully parsed Config and perform the selected action
    let config = config::parse_config()?;
    let solar_calculations = calc::SolarCalculations::new(config.date, config.coordinates);
    match config.action {
        config::Action::Report => subcommands::display_report(solar_calculations)?,
        config::Action::Wait {
            event,
            offset,
            run_missed_event,
        } => {
            subcommands::wait(event, offset, solar_calculations, run_missed_event).await?;
        }
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
