mod measurement;
mod number_input;
use crate::measurement::MeasurementComponent;
mod relative_humidity;
mod temperature;

use gloo_storage::{LocalStorage, Storage};
use owlib::open_window::{
    self, measurement::Measurement, relative_humidity::RelativeHumidity, temperature::Temperature,
};
use yew::{function_component, html, use_memo, use_state, Callback, Html};

#[function_component(App)]
pub fn app() -> Html {
    let indoor_measurement = use_state(|| {
        let mut temperature = Temperature::default();
        let stored_temperature: Result<f64, _> =
            LocalStorage::get("indoor_measurement_temperature");
        if let Ok(stored_temperature) = stored_temperature {
            temperature = Temperature::new(stored_temperature);
        }
        let mut relative_humidity = RelativeHumidity::default();
        let stored_relative_humidity: Result<u8, _> =
            LocalStorage::get("indoor_measurement_relative_humidity");
        if let Ok(stored_relative_humidity) = stored_relative_humidity {
            relative_humidity = RelativeHumidity::new(stored_relative_humidity);
        }
        Measurement {
            temperature,
            relative_humidity,
        }
    });

    let outdoor_measurement = use_state(|| {
        let mut temperature = Temperature::default();
        let stored_temperature: Result<f64, _> =
            LocalStorage::get("outdoor_measurement_temperature");
        if let Ok(stored_temperature) = stored_temperature {
            temperature = Temperature::new(stored_temperature);
        }
        let mut relative_humidity = RelativeHumidity::default();
        let stored_relative_humidity: Result<u8, _> =
            LocalStorage::get("outdoor_measurement_relative_humidity");
        if let Ok(stored_relative_humidity) = stored_relative_humidity {
            relative_humidity = RelativeHumidity::new(stored_relative_humidity);
        }
        Measurement {
            temperature,
            relative_humidity,
        }
    });

    let open_window_memo = use_memo(
        |(indoor_measurement, outdoor_measurement)| {
            let open_window = open_window::open_window(indoor_measurement, outdoor_measurement);
            if open_window {
                return String::from("YES :)");
            }
            String::from("NO :(")
        },
        (indoor_measurement.clone(), outdoor_measurement.clone()),
    );

    let indoor_measurement_changed = {
        let indoor_measurement = indoor_measurement.clone();
        let _ = LocalStorage::set(
            "indoor_measurement_temperature",
            indoor_measurement.temperature.value(),
        );
        let _ = LocalStorage::set(
            "indoor_measurement_relative_humidity",
            indoor_measurement.relative_humidity.value(),
        );
        Callback::from(move |measurement: Measurement| {
            indoor_measurement.set(measurement);
        })
    };

    let outdoor_measurement_changed = {
        let outdoor_measurement = outdoor_measurement.clone();
        let _ = LocalStorage::set(
            "outdoor_measurement_temperature",
            outdoor_measurement.temperature.value(),
        );
        let _ = LocalStorage::set(
            "outdoor_measurement_relative_humidity",
            outdoor_measurement.relative_humidity.value(),
        );
        Callback::from(move |measurement: Measurement| {
            outdoor_measurement.set(measurement);
        })
    };

    html! {
        <main>
            <article>
                <h2>{ "Should you open windows?" }</h2>
                <details>
                    <summary>{ "How it works?" }</summary>
                    <p>
                        {"This little tool answers a question whether one should open windows in order to decrease indoor humidity."}
                    </p>
                    <p>
                        {"Question is answered by comparing indoor and outdoor dew points calculated based on indoor/outdoor temperature and relative humidity measurements."}
                    </p>
                </details>
                <MeasurementComponent
                    label="Indoor Measurement"
                    measurement={(*indoor_measurement).clone()}
                    measurement_changed={indoor_measurement_changed}
                />
                <MeasurementComponent
                    label="Outdoor Measurement"
                    measurement={(*outdoor_measurement).clone()}
                    measurement_changed={outdoor_measurement_changed}
                />
                <h2>{open_window_memo}</h2>
            </article>
        </main>
    }
}
