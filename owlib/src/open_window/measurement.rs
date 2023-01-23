use super::{relative_humidity::RelativeHumidity, temperature::Temperature};

#[derive(Debug)]
pub struct Measurement {
    pub temperature: Temperature,
    pub relative_humidity: RelativeHumidity,
}

impl Measurement {
    pub fn calculate_dew_point(&self) -> f64 {
        // 243.04*(LN(RH/100)+((17.625*T)/(243.04+T)))/(17.625-LN(RH/100)-((17.625*T)/(243.04+T)))
        243.04
            * ((self.relative_humidity.value() as f64 / 100.0).ln()
                + ((17.625 * self.temperature.value()) / (243.04 + self.temperature.value())))
            / (17.625
                - (self.relative_humidity.value() as f64 / 100.0).ln()
                - ((17.625 * self.temperature.value()) / (243.04 + self.temperature.value())))
    }
}
