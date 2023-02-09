use http_api_problem::HttpApiProblem;

use owlib::open_window::{
    measurement::Measurement,
    open_window_verdict,
    relative_humidity::{RelativeHumidity, RelativeHumidityInvalid},
    temperature::{Temperature, TemperatureInvalid},
};

use std::{collections::HashMap, net::SocketAddr};

use axum::{
    extract::OriginalUri,
    http::{StatusCode, Uri},
    response::Json,
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

// curl -i -X POST localhost:3000/open-window -H 'Content-Type: application/json' -d '{ "indoor_measurement": { "temperature": 18.0, "relative_humidity": 50 }, "outdoor_measurement": { "temperature": 0.0, "relative_humidity": 85 }}'

async fn post_open_window(
    OriginalUri(uri): OriginalUri,
    Json(payload): Json<OpenWindowRequest>,
) -> Result<Json<OpenWindowResponse>, HttpApiProblem> {
    let indoor_temperature_result = Temperature::try_new(payload.indoor_measurement.temperature);
    let indoor_humidity_result =
        RelativeHumidity::try_new(payload.indoor_measurement.relative_humidity);
    let outdoor_temperature_result = Temperature::try_new(payload.outdoor_measurement.temperature);
    let outdoor_humidity_result =
        RelativeHumidity::try_new(payload.outdoor_measurement.relative_humidity);

    if indoor_temperature_result.is_ok()
        && indoor_humidity_result.is_ok()
        && outdoor_temperature_result.is_ok()
        && outdoor_humidity_result.is_ok()
    {
        let indoor_measurement = Measurement {
            temperature: indoor_temperature_result.unwrap(),
            relative_humidity: indoor_humidity_result.unwrap(),
        };

        let outdoor_measurement = Measurement {
            temperature: outdoor_temperature_result.unwrap(),
            relative_humidity: outdoor_humidity_result.unwrap(),
        };

        let open_window_verdict = open_window_verdict(&indoor_measurement, &outdoor_measurement);

        let open_window_response = OpenWindowResponse {
            indoor_dew_point: open_window_verdict.indoor_dew_point,
            outdoor_dew_point: open_window_verdict.outdoor_dew_point,
            open_window: open_window_verdict.open_window,
        };

        return Ok(Json(open_window_response));
    }

    let mut errors: HashMap<String, Vec<String>> = HashMap::new();

    if let Err(TemperatureInvalid(msg)) = indoor_temperature_result {
        errors.insert(
            String::from("indoor_measurement.temperature"),
            vec![msg.into()],
        );
    }

    if let Err(RelativeHumidityInvalid(msg)) = indoor_humidity_result {
        errors.insert(
            String::from("indoor_measurement.relative_humidity"),
            vec![msg.into()],
        );
    }

    if let Err(TemperatureInvalid(msg)) = outdoor_temperature_result {
        errors.insert(
            String::from("outdoor_measurement.temperature"),
            vec![msg.into()],
        );
    }

    if let Err(RelativeHumidityInvalid(msg)) = outdoor_humidity_result {
        errors.insert(
            String::from("outdoor_measurement.relative_humidity"),
            vec![msg.into()],
        );
    }

    let problem = HttpApiProblem::new(StatusCode::BAD_REQUEST)
        .title("Invalid request.")
        .detail("Request validation failed.")
        .type_url("validation-error")
        .instance(uri.to_string())
        .value("errors", &errors);

    Err(problem)
}
