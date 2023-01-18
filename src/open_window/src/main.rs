#[derive(Debug)]
struct Measurement {
    temperature: f64,
    relative_humidity: f64,
}

impl Measurement {
    fn calculate_dew_point(&self) -> f64 {
        // 243.04*(LN(RH/100)+((17.625*T)/(243.04+T)))/(17.625-LN(RH/100)-((17.625*T)/(243.04+T)))
        243.04
            * ((self.relative_humidity / 100.0).ln()
                + ((17.625 * self.temperature) / (243.04 + self.temperature)))
            / (17.625
                - (self.relative_humidity / 100.0).ln()
                - ((17.625 * self.temperature) / (243.04 + self.temperature)))
    }
}

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
