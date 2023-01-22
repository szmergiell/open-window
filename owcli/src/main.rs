mod cli;

extern crate owlib;

use crate::cli::*;
use crate::owlib::open_window::measurement::*;
use clap::Parser;

fn main() {
    let args = Cli::parse();

    let indoor_measurement = Measurement {
        temperature: args.indoor_temperature,
        relative_humidity: args.indoor_humidity,
    };

    let outdoor_measurement = Measurement {
        temperature: args.outdoor_temperature,
        relative_humidity: args.outdoor_humidity,
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
}
