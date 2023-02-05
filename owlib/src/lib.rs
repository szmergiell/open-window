#![warn(missing_docs)]

//! Answers a question whether one should open windows or not in order to decrease
//! indoor humidity.
//!
//! Question is answered by comparing indoor and outdoor dew points calculated
//! based on indoor/outdoor temperature and relative humidity measurements.
//!
//! # Example
//!
//! ```
//! use owlib::open_window::measurement::Measurement;
//! use owlib::open_window::relative_humidity::RelativeHumidity;
//! use owlib::open_window::temperature::Temperature;
//! use owlib::open_window::open_window;
//!
//! let indoor_temperature = Temperature::new(18.0);
//! let indoor_humidity = RelativeHumidity::new(55);
//! let indoor_measurement = Measurement {
//!     temperature: indoor_temperature,
//!     relative_humidity: indoor_humidity,
//! };
//!
//! let outdoor_temperature = Temperature::new(1.0);
//! let outdoor_humidity = RelativeHumidity::new(85);
//! let outdoor_measurement = Measurement {
//!     temperature: outdoor_temperature,
//!     relative_humidity: outdoor_humidity,
//! };
//!
//! let open_window = open_window(&indoor_measurement, &outdoor_measurement);

/// Module holds necesarry strucutres and functions to calculate dew points and
/// answer the "should you open windows" question.
pub mod open_window;
