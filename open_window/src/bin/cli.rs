extern crate open_window;

use crate::open_window::open_window::measurement::*;

fn main() {
    let indoor_measurement = Measurement {
        temperature: 18.0,
        relative_humidity: 55.0,
    };

    let outdoor_measurement = Measurement {
        temperature: 1.8,
        relative_humidity: 90.0,
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
