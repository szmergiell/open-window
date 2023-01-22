mod cli;

extern crate owlib;

use std::error::Error;

use crate::cli::*;
use crate::owlib::open_window::measurement::*;
use clap::Parser;
use owlib::open_window::relative_humidity::RelativeHumidity;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let indoor_humidity = RelativeHumidity::try_new(args.indoor_humidity)?;

    let indoor_measurement = Measurement {
        temperature: args.indoor_temperature,
        relative_humidity: indoor_humidity,
    };

    let outdoor_humidity = RelativeHumidity::try_new(args.outdoor_humidity)?;

    let outdoor_measurement = Measurement {
        temperature: args.outdoor_temperature,
        relative_humidity: outdoor_humidity,
    };

    let indoor_dew_point = indoor_measurement.calculate_dew_point();
    let outdoor_dew_point = outdoor_measurement.calculate_dew_point();

    let message = match indoor_dew_point > outdoor_dew_point {
        true => "Open windows",
        false => "Close windows",
    };

    println!("Indoor dew point: {}", indoor_dew_point);

    println!("Ourdoor dew point: {}", outdoor_dew_point);

    println!("{}", message);

    Ok(())
}
