use owlib::open_window::{
    measurement::Measurement, relative_humidity::RelativeHumidity, temperature::Temperature,
};
use yew::{function_component, html, use_state, Callback, Html, Properties};

use crate::{relative_humidity::RelativeHumidityComponent, temperature::TemperatureComponent};

#[derive(Properties, PartialEq)]
pub struct MeasurementProps {
    #[prop_or(String::from("Measurement"))]
    pub label: String,
    #[prop_or_default]
    pub measurement: Measurement,
    #[prop_or_default]
    pub measurement_changed: Callback<Measurement>,
}

#[function_component]
pub fn MeasurementComponent(
    MeasurementProps {
        label,
        measurement,
        measurement_changed,
    }: &MeasurementProps,
) -> Html {
    let measurement_state = use_state(|| measurement.clone());

    let humidity_changed = {
        let measurement_state = measurement_state.clone();
        let measurement_changed = measurement_changed.clone();

        Callback::from(move |relative_humidity: RelativeHumidity| {
            let measurement = Measurement {
                temperature: measurement_state.temperature.clone(),
                relative_humidity,
            };
            measurement_state.set(measurement.clone());
            measurement_changed.emit(measurement);
        })
    };

    let temperature_changed = {
        let measurement_state = measurement_state.clone();
        let measurement_changed = measurement_changed.clone();

        Callback::from(move |temperature: Temperature| {
            let measurement = Measurement {
                temperature,
                relative_humidity: measurement_state.relative_humidity.clone(),
            };
            measurement_state.set(measurement.clone());
            measurement_changed.emit(measurement);
        })
    };

    html! {
        <div class="measurement">
            <h2>{ label }</h2>
            <TemperatureComponent
                value={measurement_state.temperature.clone()}
                {temperature_changed}
            />
            <RelativeHumidityComponent
                value={measurement_state.relative_humidity.clone()}
                {humidity_changed}
            />
            <label>
                { "Dew Point [°C]" }
                <input type="number" disabled={true} value={format!("{:.2}", measurement_state.calculate_dew_point())}/>
            </label>
        </div>
    }
}
