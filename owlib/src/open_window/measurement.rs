use super::{relative_humidity::RelativeHumidity, temperature::Temperature};

/// Holds a temperature and relative humidity values.
#[derive(Debug)]
pub struct Measurement {
    /// Temperature.
    pub temperature: Temperature,
    /// Relative humidity.
    pub relative_humidity: RelativeHumidity,
}

impl Measurement {
    /// Calculates a dew point based on temperature and relative humidity values
    /// held by this `Measurement` struct.
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

#[cfg(test)]
mod tests {
    use crate::open_window::{relative_humidity::RelativeHumidity, temperature::Temperature};

    use super::Measurement;

    #[test]
    fn indoor_measurement_test() {
        let measurement = Measurement {
            temperature: Temperature::new(18.0),
            relative_humidity: RelativeHumidity::new(55),
        };

        let dew_point = measurement.calculate_dew_point();

        assert_eq!(format!("{dew_point:.2}"), "8.82");
    }

    #[test]
    fn outdoor_measurement_test() {
        let measurement = Measurement {
            temperature: Temperature::new(-5.0),
            relative_humidity: RelativeHumidity::new(80),
        };

        let dew_point = measurement.calculate_dew_point();

        assert_eq!(format!("{dew_point:.2}"), "-7.92");
    }
}
