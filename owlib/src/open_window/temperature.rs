use core::fmt;
use std::error::Error;

const VALIDATION_ERROR: &str = "temperature must be a decimal value between -100.0 and 100.0";
/// Minimum value of temperature
pub const MIN_TEMP: f64 = -100.0;
/// Maximum value of temperature
pub const MAX_TEMP: f64 = 100.0;

/// An error returned by [Temperature::try_new] if provided value is invalid.
#[derive(Debug)]
pub struct TemperatureInvalid(pub &'static str);

impl Error for TemperatureInvalid {}

impl fmt::Display for TemperatureInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{VALIDATION_ERROR}")
    }
}

/// Holds a temperature value expressed in Celcius degrees (°C).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Temperature {
    value: f64,
}

impl Temperature {
    /// Creates new `Temperature` struct.
    ///
    /// # Panics
    ///
    /// Panics if provided value does not fall within following range `(-100.0..=100.0)`.
    pub fn new(value: f64) -> Self {
        if !Self::valid(value) {
            panic!("{}", VALIDATION_ERROR);
        }

        Self { value }
    }

    /// Creates new `Temperature` struct.
    ///
    /// As opposed to [Temperature::new] function it does not panic, but returns
    /// a [TemperatureInvalid] error instead.
    pub fn try_new(value: f64) -> Result<Self, TemperatureInvalid> {
        if !Self::valid(value) {
            return Err(TemperatureInvalid(VALIDATION_ERROR));
        }

        Ok(Self::new(value))
    }

    /// Returns a temperature value.
    pub fn value(&self) -> f64 {
        self.value
    }

    fn valid(value: f64) -> bool {
        (MIN_TEMP..=MAX_TEMP).contains(&value)
    }
}
