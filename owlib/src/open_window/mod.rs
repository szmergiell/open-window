use self::measurement::Measurement;

/// Modules holds necesarry structures and functions for creating `Measurement`.
pub mod measurement;

/// Modules holds necesarry structures and functions for creating `RelativeHumidity`.
pub mod relative_humidity;

/// Modules holds necesarry structures and functions for creating `Temperature`.
pub mod temperature;

/// Answers a question whether one should open windows in order to decrease
/// indoor humidity.
///
/// The decision is made by comparing indoor and outdoor dew points, calculated
/// from indoor/outdoor measurements.
///
/// # Example
///
/// ```
/// use owlib::open_window::measurement::Measurement;
/// use owlib::open_window::relative_humidity::RelativeHumidity;
/// use owlib::open_window::temperature::Temperature;
/// use owlib::open_window::open_window;
///
/// let indoor_temperature = Temperature::new(18.0);
/// let indoor_humidity = RelativeHumidity::new(50);
/// let indoor_measurement = Measurement {
///     temperature: indoor_temperature,
///     relative_humidity: indoor_humidity,
/// };
///
/// let outdoor_temperature = Temperature::new(1.0);
/// let outdoor_humidity = RelativeHumidity::new(85);
/// let outdoor_measurement = Measurement {
///     temperature: outdoor_temperature,
///     relative_humidity: outdoor_humidity,
/// };
///
/// let open_window = open_window(&indoor_measurement, &outdoor_measurement);
/// ```
pub fn open_window(indoor_measurement: &Measurement, outdoor_measurement: &Measurement) -> bool {
    open_window_result(indoor_measurement, outdoor_measurement).open_window
}

/// Holds detailed calculation results performed while obtaining "open window" verdict.
pub struct OpenWindowResult {
    /// Indoor dew point.
    pub indoor_dew_point: f64,
    /// Outdoor dew point.
    pub outdoor_dew_point: f64,
    /// Open window verdict.
    pub open_window: bool,
}

/// Answers a question whether one should open windows in order to decrease
/// indoor humidity.
///
/// The decision is made by comparing indoor and outdoor dew points, calculated
/// from indoor/outdoor measurements.
///
/// As opposed to [open_window::open_window] this method - beside "open window" verdict -
/// also returns intermediate calculations steps - indoor / outdoor dew points.
pub fn open_window_result(
    indoor_measurement: &Measurement,
    outdoor_measurement: &Measurement,
) -> OpenWindowResult {
    let indoor_dew_point = indoor_measurement.calculate_dew_point();
    let outdoor_dew_point = outdoor_measurement.calculate_dew_point();
    let open_window = indoor_dew_point > outdoor_dew_point;

    OpenWindowResult {
        indoor_dew_point,
        outdoor_dew_point,
        open_window,
    }
}
