use core::fmt;
use std::error::Error;

const VALIDATION_ERROR: &str = "relative humidity should be a percentage value between 0 and 100";
const MIN_HUMIDITY: u8 = 0;
const MAX_HUMIDITY: u8 = 100;

#[derive(Debug)]
struct RelativeHumidityInvalid {}

impl Error for RelativeHumidityInvalid {}

impl fmt::Display for RelativeHumidityInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", VALIDATION_ERROR)
    }
}

#[derive(Debug)]
pub struct RelativeHumidity {
    value: u8,
}

impl RelativeHumidity {
    pub fn new(value: u8) -> Self {
        if !Self::valid(value) {
            panic!("{}", VALIDATION_ERROR);
        }

        Self { value }
    }

    pub fn try_new(value: u8) -> Result<Self, Box<dyn Error>> {
        if !Self::valid(value) {
            return Err(Box::new(RelativeHumidityInvalid {}));
        }

        Ok(Self::new(value))
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    fn valid(value: u8) -> bool {
        value >= MIN_HUMIDITY && value <= MAX_HUMIDITY
    }
}
