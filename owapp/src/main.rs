use std::{str::FromStr, fmt::Display, string};

use owlib::open_window::{measurement::Measurement, temperature::Temperature, relative_humidity::RelativeHumidity, self};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let indoor_measurement = use_state(|| {
        Measurement {
            temperature: Temperature::new(0.0),
            relative_humidity: RelativeHumidity::new(1)
        }
    });

    let outdoor_measurement = use_state(|| {
        Measurement {
            temperature: Temperature::new(0.0),
            relative_humidity: RelativeHumidity::new(1)
        }
    });

    let open_window_memo = use_memo(|(indoor_measurement, outdoor_measurement)| {
        let open_window = open_window::open_window(indoor_measurement, outdoor_measurement);
        if open_window {
            return String::from("YES!")
        }
        String::from("NO :(")
    }, (indoor_measurement.clone(), outdoor_measurement.clone()));

    html! {
        <>
            <h1>{ "Open Window App" }</h1>
            <MeasurementComponent
                temperature={indoor_measurement.temperature.clone()}
                relative_humidity={indoor_measurement.relative_humidity.clone()}
            />
            <MeasurementComponent
                temperature={outdoor_measurement.temperature.clone()}
                relative_humidity={outdoor_measurement.relative_humidity.clone()}
            />
            <h2>{open_window_memo}</h2>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct MeasurementProps {
    temperature: Temperature,
    relative_humidity: RelativeHumidity,
    dew_point_changed: Option<Callback<f64>>
}

#[function_component(MeasurementComponent)]
fn measurement(MeasurementProps { temperature, relative_humidity, dew_point_changed }: &MeasurementProps) -> Html {
    let temperature_state = use_state(|| temperature.clone());
    let relative_humidity_state = use_state(|| relative_humidity.clone());

    let measurement_memo = use_memo(|(temperature, relative_humidity)| {
        Measurement {
            temperature: temperature.to_owned(),
            relative_humidity: relative_humidity.to_owned()
        }
    }, ((*temperature_state).clone(), (*relative_humidity_state).clone()));

    let dew_point_memo = use_memo(|measurement| {
        let dew_point = measurement.calculate_dew_point();
        if let Some(dew_point_changed) = dew_point_changed {
            dew_point_changed.emit(dew_point);
        }
        dew_point
    }, (*measurement_memo).clone());

    let humidity_changed = {
        let relative_humidity_state = relative_humidity_state.clone();
        Callback::from(move |number: u8| {
            relative_humidity_state.set(RelativeHumidity::new(number));
        })
    };

    let temperature_changed = {
        let temperature_state = temperature_state.clone();
        Callback::from(move |number: f64| {
            temperature_state.set(Temperature::new(number));
        })
    };

    html! {
        <>
            <TemperatureComponent
                value={temperature_state.value()}
                {temperature_changed}
            />
            <RelativeHumidityComponent
                value={relative_humidity_state.value()}
                {humidity_changed}
            />
            <input type="number" value={dew_point_memo.to_string()}/>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct TemperatureProps {
    value: f64,
    temperature_changed: Callback<f64>
}

#[function_component(TemperatureComponent)]
fn temperature(TemperatureProps { value, temperature_changed }: &TemperatureProps) -> Html {
    html! {
        <NumberInput<f64>
            min={-100.0}
            max={100.0}
            step={0.1}
            {value}
            number_changed={temperature_changed}
        />
    }
}

trait Number: PartialEq + PartialOrd + Default + FromStr + Display + Clone + 'static {}

impl Number for u8 {}
impl Number for f64 {}

#[derive(Properties, PartialEq)]
struct RelativeHumidityProps {
    value: u8,
    humidity_changed: Callback<u8>
}

#[function_component]
fn RelativeHumidityComponent(RelativeHumidityProps { value, humidity_changed }: &RelativeHumidityProps) -> Html {
    html! {
        <NumberInput<u8>
            min={1u8}
            max={100u8}
            step={1u8}
            {value}
            number_changed={humidity_changed}
        />
    }
}

#[derive(Properties, PartialEq)]
struct NumberInputProps<T>
where T: Number {
    value: T,
    min: T,
    max: T,
    step: T,
    number_changed: Callback<T>
}

#[function_component]
fn NumberInput<T>(NumberInputProps { value, min, max, step, number_changed }: &NumberInputProps<T>) -> Html
where T: Number {

    let onchange = {
        let number_changed = number_changed.clone();
        let min = min.clone();
        let max = max.clone();

        Callback::from(move |e: Event| {
            let target = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let string_value = input.value();
                let value_result = string_value.parse::<T>();
                let mut number = value_result.unwrap_or(T::default());
                if number < min {
                    number = min.clone();
                }
                if number > max {
                    number = max.clone();
                }
                input.set_value(&number.to_string());
                number_changed.emit(number);
            }
        })
    };

    html! {
        <input
            type="number"
            min={min.to_string()}
            max={max.to_string()}
            step={step.to_string()}
            value={value.to_string()}
            {onchange}
        />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
