use owlib::open_window::temperature::{Temperature, MIN_TEMP, MAX_TEMP};
use yew::{function_component, use_state, Html, Callback, html, Properties};

use crate::number_input::{NumberInput, Number};

impl Number for f64 {}

#[derive(Properties, PartialEq)]
pub struct TemperatureProps {
    #[prop_or_default]
    pub value: Temperature,
    #[prop_or_default]
    pub temperature_changed: Callback<Temperature>
}

#[function_component]
pub fn TemperatureComponent(TemperatureProps { value, temperature_changed }: &TemperatureProps) -> Html {
    let temperature_state = use_state(|| value.clone());

    let number_changed = {
        let temperature_state = temperature_state.clone();
        let temperature_changed = temperature_changed.clone();

        Callback::from(move |number: f64| {
            let temperature = Temperature::new(number);
            temperature_state.set(temperature.clone());
            temperature_changed.emit(temperature);
        })
    };

    html! {
        <NumberInput<f64>
            min={MIN_TEMP}
            max={MAX_TEMP}
            step={0.1}
            value={temperature_state.value()}
            {number_changed}
        />
    }
}
