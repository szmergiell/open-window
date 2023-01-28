use serde::Serialize;

#[derive(Serialize)]
pub struct CliOutput {
    pub indoor_dew_point: f64,
    pub outdoor_dew_point: f64,
    pub open_window: bool,
}
