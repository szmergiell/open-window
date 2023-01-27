use core::fmt;
use std::error::Error;

const VALIDATION_ERROR: &str = "temperature must be a decimal value between -100.0 and 100.0";
const MIN_TEMP: f64 = -100.0;
const MAX_TEMP: f64 = 100.0;

#[derive(Debug)]
struct TemperatureInvalid {}

impl Error for TemperatureInvalid {}

impl fmt::Display for TemperatureInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{VALIDATION_ERROR}")
    }
}

#[derive(Debug)]
pub struct Temperature {
    value: f64,
}

impl Temperature {
    pub fn new(value: f64) -> Self {
        if !Self::valid(value) {
            panic!("{}", VALIDATION_ERROR);
        }

        Self { value }
    }

    pub fn try_new(value: f64) -> Result<Self, Box<dyn Error>> {
        if !Self::valid(value) {
            return Err(Box::new(TemperatureInvalid {}));
        }

        Ok(Self::new(value))
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    fn valid(value: f64) -> bool {
        (MIN_TEMP..=MAX_TEMP).contains(&value)
    }
}
