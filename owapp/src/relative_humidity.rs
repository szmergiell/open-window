use owlib::open_window::relative_humidity::{RelativeHumidity, MAX_HUMIDITY, MIN_HUMIDITY};
use yew::{function_component, html, use_state, Callback, Html, Properties};

use crate::number_input::{Number, NumberInput};

impl Number for u8 {}

#[derive(Properties, PartialEq)]
pub struct RelativeHumidityProps {
    #[prop_or_default]
    pub value: RelativeHumidity,
    #[prop_or_default]
    pub humidity_changed: Callback<RelativeHumidity>,
}

#[function_component]
pub fn RelativeHumidityComponent(
    RelativeHumidityProps {
        value,
        humidity_changed,
    }: &RelativeHumidityProps,
) -> Html {
    let relative_humidity_state = use_state(|| value.clone());

    let number_changed = {
        let relative_humidity_state = relative_humidity_state.clone();
        let humidity_changed = humidity_changed.clone();

        Callback::from(move |number: u8| {
            let relative_humidity = RelativeHumidity::new(number);
            relative_humidity_state.set(relative_humidity.clone());
            humidity_changed.emit(relative_humidity);
        })
    };

    html! {
        <NumberInput<u8>
            label="Relative Humidity [%]"
            min={MIN_HUMIDITY}
            max={MAX_HUMIDITY}
            step={1u8}
            value={relative_humidity_state.value()}
            {number_changed}
        />
    }
}
