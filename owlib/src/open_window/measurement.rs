#[derive(Debug)]
pub struct Measurement {
    pub temperature: f64,
    pub relative_humidity: f64,
}

impl Measurement {
    pub fn calculate_dew_point(&self) -> f64 {
        // 243.04*(LN(RH/100)+((17.625*T)/(243.04+T)))/(17.625-LN(RH/100)-((17.625*T)/(243.04+T)))
        243.04
            * ((self.relative_humidity / 100.0).ln()
               + ((17.625 * self.temperature) / (243.04 + self.temperature)))
            / (17.625
               - (self.relative_humidity / 100.0).ln()
               - ((17.625 * self.temperature) / (243.04 + self.temperature)))
    }
}
