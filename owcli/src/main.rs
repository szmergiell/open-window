mod cli;

extern crate owlib;

use clap::Parser;
use cli::Cli;
use owlib::open_window::measurement::Measurement;
use owlib::open_window::open_window;
use owlib::open_window::relative_humidity::RelativeHumidity;
use owlib::open_window::temperature::Temperature;
use std::error::Error;

fn run() -> Result<&'static str, Box<dyn Error>> {
    let args = Cli::parse();

    let indoor_humidity = RelativeHumidity::try_new(args.indoor_humidity)?;
    let indoor_temperature = Temperature::try_new(args.indoor_temperature)?;
    let indoor_measurement = Measurement {
        temperature: indoor_temperature,
        relative_humidity: indoor_humidity,
    };

    let outdoor_humidity = RelativeHumidity::try_new(args.outdoor_humidity)?;
    let outdoor_temperature = Temperature::try_new(args.outdoor_temperature)?;
    let outdoor_measurement = Measurement {
        temperature: outdoor_temperature,
        relative_humidity: outdoor_humidity,
    };

    let message = match open_window(&indoor_measurement, &outdoor_measurement) {
        true => "Open window!",
        false => "Close window!",
    };

    println!(
        "Indoor dew point: {:.2}",
        indoor_measurement.calculate_dew_point()
    );

    println!(
        "Ourdoor dew point: {:.2}",
        outdoor_measurement.calculate_dew_point()
    );

    Ok(message)
}

fn main() {
    match run() {
        Ok(message) => {
            println!("{message}");
            std::process::exit(0)
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1)
        }
    }
}
