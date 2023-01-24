use self::measurement::Measurement;

pub mod measurement;
pub mod relative_humidity;
pub mod temperature;

pub fn open_window(indoor_measurement: &Measurement, outdoor_measurement: &Measurement) -> bool {
    let indoor_dew_point = indoor_measurement.calculate_dew_point();
    let outdoor_dew_point = outdoor_measurement.calculate_dew_point();

    indoor_dew_point > outdoor_dew_point
}
