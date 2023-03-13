mod measurement;
mod number_input;
use crate::measurement::MeasurementComponent;
mod relative_humidity;
mod temperature;

use owlib::open_window::{measurement::Measurement, self};
use yew::{function_component, use_state, Html, use_memo, Callback, html};

#[function_component(App)]
pub fn app() -> Html {
    let indoor_measurement = use_state(Measurement::default);

    let outdoor_measurement = use_state(Measurement::default);

    let open_window_memo = use_memo(|(indoor_measurement, outdoor_measurement)| {
        let open_window = open_window::open_window(indoor_measurement, outdoor_measurement);
        if open_window {
            return String::from("YES!")
        }
        String::from("NO :(")
    }, (indoor_measurement.clone(), outdoor_measurement.clone()));

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
        <>
            <h1>{ "Open Window App" }</h1>
            <MeasurementComponent
                measurement={(*indoor_measurement).clone()}
                measurement_changed={indoor_measurement_changed}
            />
            <MeasurementComponent
                measurement={(*outdoor_measurement).clone()}
                measurement_changed={outdoor_measurement_changed}
            />
            <h2>{open_window_memo}</h2>
        </>
    }
}
