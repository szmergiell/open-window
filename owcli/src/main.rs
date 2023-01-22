extern crate owlib;

use std::env::args;

use crate::owlib::open_window::measurement::*;

fn main() {
    let indoor_temperature = args().nth(1).expect("Indoor temperature is required");
    let indoor_temperature: f64 = indoor_temperature.parse().expect("Indoor temperature must be numeric");
    let indoor_humidity = args().nth(2).expect("Indoor humidity is required");
    let indoor_humidity: f64 = indoor_humidity.parse().expect("Indoor humidity must be numeric");

    let outdoor_temperature = args().nth(3).expect("Outdoor temperature is required");
    let outdoor_temperature: f64 = outdoor_temperature.parse().expect("Outdoor temperature must be numeric");
    let outdoor_humidity = args().nth(4).expect("Outdoor humidity is required");
    let outdoor_humidity: f64 = outdoor_humidity.parse().expect("Outdoor humidity must be numeric");

    let indoor_measurement = Measurement {
        temperature: indoor_temperature,
        relative_humidity: indoor_humidity,
    };

    let outdoor_measurement = Measurement {
        temperature: outdoor_temperature,
        relative_humidity: outdoor_humidity,
    };

    let indoor_dew_point = indoor_measurement.calculate_dew_point();
    let outdoor_dew_point = outdoor_measurement.calculate_dew_point();

    let message = match indoor_dew_point > outdoor_dew_point {
        true => "Open windows",
        false => "Close windows",
    };

    println!(
        "Indoor dew point: {}",
        indoor_measurement.calculate_dew_point()
    );

    println!(
        "Ourdoor dew point: {}",
        outdoor_measurement.calculate_dew_point()
    );

    println!("{}", message);
}
