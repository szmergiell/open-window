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

    macro_rules! dew_point_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (temperature, relative_humidity, expected) = $value;

                    let measurement = Measurement {
                        temperature: Temperature::new(temperature),
                        relative_humidity: RelativeHumidity::new(relative_humidity),
                    };

                    let dew_point = measurement.calculate_dew_point();

                    assert_eq!(expected, format!("{dew_point:.2}"));
                }
             )*
        }
    }

    dew_point_tests! {
        indoor: (18.0, 55, "8.82"),
        outdoor: (-5.0, 80, "-7.92"),
        zero: (0.0, 1, "-50.35"),
    }
}
