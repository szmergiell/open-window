mod measurement;
mod number_input;
use crate::measurement::MeasurementComponent;
mod relative_humidity;
mod temperature;

use owlib::open_window::{self, measurement::Measurement};
use yew::{function_component, html, use_memo, use_state, Callback, Html};

#[function_component(App)]
pub fn app() -> Html {
    let indoor_measurement = use_state(Measurement::default);

    let outdoor_measurement = use_state(Measurement::default);

    let open_window_memo = use_memo(
        |(indoor_measurement, outdoor_measurement)| {
            let open_window = open_window::open_window(indoor_measurement, outdoor_measurement);
            if open_window {
                return String::from("YES!");
            }
            String::from("NO :(")
        },
        (indoor_measurement.clone(), outdoor_measurement.clone()),
    );

    let indoor_measurement_changed = {
        let indoor_measurement = indoor_measurement.clone();
        Callback::from(move |measurement: Measurement| {
            indoor_measurement.set(measurement);
        })
    };

    let outdoor_measurement_changed = {
        let outdoor_measurement = outdoor_measurement.clone();
        Callback::from(move |measurement: Measurement| {
            outdoor_measurement.set(measurement);
        })
    };

    html! {
        <main>
            <article>
                <h1>{ "Open Window App" }</h1>
                <p>
                {"This little tool answers a question whether one should open windows in order to decrease indoor humidity."}
                </p>
                <p>
                {"Question is answered by comparing indoor and outdoor dew points calculated based on indoor/outdoor temperature and relative humidity measurements."}
                </p>
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
