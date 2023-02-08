use owlib::open_window::{
    measurement::Measurement, open_window_result, relative_humidity::{RelativeHumidity, RelativeHumidityInvalid},
    temperature::{Temperature, TemperatureInvalid},
};
use serde_json::json;

use std::{net::SocketAddr, collections::{HashMap, HashSet}};

use axum::{
    http::{StatusCode, Uri},
    response::{Json, IntoResponse, Response},
    routing::post,
    Router,
};

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(fallback)
        .route("/open-window", post(post_open_window));

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");

    println!("shutdown signal");
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route {uri}"))
}

#[derive(Serialize)]
pub struct OpenWindowResponse {
    pub indoor_dew_point: f64,
    pub outdoor_dew_point: f64,
    pub open_window: bool,
}

#[derive(Deserialize)]
pub struct OpenWindowRequest {
    pub indoor_measurement: MeasurementRequest,
    pub outdoor_measurement: MeasurementRequest,
}

#[derive(Deserialize)]
pub struct MeasurementRequest {
    pub temperature: f64,
    pub relative_humidity: u8,
}

enum ValidationError {
    InvalidTemperature(String),
    InvalidHumidity(String),
}

#[derive(Serialize)]
pub struct ValidationErrorResponse {
    pub errors: HashMap<String, Vec<String>>
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let body = match self {
            ValidationError::InvalidTemperature(msg) => msg,
            ValidationError::InvalidHumidity(msg) => msg
        };

        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

impl IntoResponse for ValidationErrorResponse {
    fn into_response(self) -> Response {
        let body = Json(json!(self));
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

// curl -i -X POST localhost:3000/open-window -H 'Content-Type: application/json' -d '{ "indoor_measurement": { "temperature": 18.0, "relative_humidity": 50 }, "outdoor_measurement": { "temperature": 0.0, "relative_humidity": 85 }}'

async fn post_open_window(Json(payload): Json<OpenWindowRequest>) -> Result<Json<OpenWindowResponse>, ValidationErrorResponse> {
    // TODO: validation, error codes, etc.
    // if let Err(TemperatureInvalid(msg)) = Temperature::try_new(payload.indoor_measurement.temperature) {
    //     return Err(ValidationError::InvalidTemperature(format!("indoor_measurement.temperature: {msg}")))
    // }

    // if let Err(TemperatureInvalid(msg)) = Temperature::try_new(payload.outdoor_measurement.temperature) {
    //     return Err(ValidationError::InvalidTemperature(format!("outdoor_measurement.temperature: {msg}")))
    // }

    // if let Err(RelativeHumidityInvalid(msg)) = RelativeHumidity::try_new(payload.indoor_measurement.relative_humidity) {
    //     return Err(ValidationError::InvalidHumidity(format!("indoor_measurement.relative_humidity: {msg}")))
    // }

    if let Err(RelativeHumidityInvalid(msg)) = RelativeHumidity::try_new(payload.outdoor_measurement.relative_humidity) {
        let mut validation_error_response = ValidationErrorResponse {
            errors: HashMap::new()
        };

        validation_error_response.errors.insert(String::from("outdoor_measurement.relative_humidity"), vec![msg.into()]);
        return Err(validation_error_response)
    }

    let indoor_temperature = Temperature::new(payload.indoor_measurement.temperature);
    let indoor_humidity = RelativeHumidity::new(payload.indoor_measurement.relative_humidity);
    let indoor_measurement = Measurement {
        temperature: indoor_temperature,
        relative_humidity: indoor_humidity,
    };

    let outdoor_temperature = Temperature::new(payload.outdoor_measurement.temperature);
    let outdoor_humidity = RelativeHumidity::new(payload.outdoor_measurement.relative_humidity);
    let outdoor_measurement = Measurement {
        temperature: outdoor_temperature,
        relative_humidity: outdoor_humidity,
    };

    let open_window_result = open_window_result(&indoor_measurement, &outdoor_measurement);

    let open_window_response = OpenWindowResponse {
        indoor_dew_point: open_window_result.indoor_dew_point,
        outdoor_dew_point: open_window_result.outdoor_dew_point,
        open_window: open_window_result.open_window,
    };

    Ok(Json(open_window_response))
}
