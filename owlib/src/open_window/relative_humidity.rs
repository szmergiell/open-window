use core::fmt;
use std::error::Error;

const VALIDATION_ERROR: &str = "relative humidity should be a percentage value between 1 and 100";

/// Minimum value of relative humidity
pub const MIN_HUMIDITY: u8 = 1;

/// Maximum value of relative humidity
pub const MAX_HUMIDITY: u8 = 100;

/// An error returned by [RelativeHumidity::try_new] if provided value is invalid.
#[derive(Debug)]
pub struct RelativeHumidityInvalid(pub &'static str);

impl Error for RelativeHumidityInvalid {}

impl fmt::Display for RelativeHumidityInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{VALIDATION_ERROR}")
    }
}

/// Holds an integer value represeting relative humidity expressed as percentage (%).
#[derive(Debug, Clone, PartialEq)]
pub struct RelativeHumidity {
    value: u8,
}

impl RelativeHumidity {
    /// Creates a new `RelativeHumidity` struct.
    ///
    /// # Panics
    ///
    /// Panics if provided value is not a valid relative humidity value (`(1..=100)`).
    pub fn new(value: u8) -> Self {
        if !Self::valid(value) {
            panic!("{}", VALIDATION_ERROR);
        }

        Self { value }
    }

    /// Creates a new `RelativeHumidity` struct.
    ///
    /// As opposed to [RelativeHumidity::new] function it does not panic, but returns a validation
    /// error instead.
    pub fn try_new(value: u8) -> Result<Self, RelativeHumidityInvalid> {
        if !Self::valid(value) {
            return Err(RelativeHumidityInvalid(VALIDATION_ERROR));
        }

        Ok(Self::new(value))
    }

    /// Returns a relative humidity value.
    pub fn value(&self) -> u8 {
        self.value
    }

    fn valid(value: u8) -> bool {
        (MIN_HUMIDITY..=MAX_HUMIDITY).contains(&value)
    }
}

impl Default for RelativeHumidity {
    fn default() -> Self {
        Self {
            value: MIN_HUMIDITY,
        }
    }
}
