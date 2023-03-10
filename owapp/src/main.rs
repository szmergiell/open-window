use std::{str::FromStr, fmt::Display};

use owlib::open_window::{measurement::Measurement, temperature::{Temperature, MIN_TEMP, MAX_TEMP}, relative_humidity::{RelativeHumidity, MIN_HUMIDITY, MAX_HUMIDITY}, self};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[function_component(App)]
fn app() -> Html {
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

#[derive(Properties, PartialEq)]
struct MeasurementProps {
    #[prop_or_default]
    measurement: Measurement,
    #[prop_or_default]
    measurement_changed: Callback<Measurement>
}

#[function_component]
fn MeasurementComponent(MeasurementProps { measurement, measurement_changed }: &MeasurementProps) -> Html {
    let measurement_state = use_state(|| measurement.clone());

    let humidity_changed = {
        let measurement_state = measurement_state.clone();
        let measurement_changed = measurement_changed.clone();

        Callback::from(move |relative_humidity: RelativeHumidity| {
            let measurement = Measurement {
                temperature: measurement_state.temperature.clone(),
                relative_humidity
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
                relative_humidity: measurement_state.relative_humidity.clone()
            };
            measurement_state.set(measurement.clone());
            measurement_changed.emit(measurement);
        })
    };

    html! {
        <div>
            <TemperatureComponent
                value={measurement_state.temperature.clone()}
                {temperature_changed}
            />
            <RelativeHumidityComponent
                value={measurement_state.relative_humidity.clone()}
                {humidity_changed}
            />
            <input type="number" disabled={true} value={measurement_state.calculate_dew_point().to_string()}/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TemperatureProps {
    #[prop_or_default]
    value: Temperature,
    #[prop_or_default]
    temperature_changed: Callback<Temperature>
}

#[function_component]
fn TemperatureComponent(TemperatureProps { value, temperature_changed }: &TemperatureProps) -> Html {
    let temperature_state = use_state(|| value.clone());
    let temperature_changed = temperature_changed.clone();

    let number_changed = {
        let temperature_state = temperature_state.clone();

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

trait Number: PartialEq + PartialOrd + Default + FromStr + Display + Clone + 'static {}

impl Number for u8 {}
impl Number for f64 {}

#[derive(Properties, PartialEq)]
struct RelativeHumidityProps {
    #[prop_or_default]
    value: RelativeHumidity,
    #[prop_or_default]
    humidity_changed: Callback<RelativeHumidity>
}

#[function_component]
fn RelativeHumidityComponent(RelativeHumidityProps { value, humidity_changed }: &RelativeHumidityProps) -> Html {
    let relative_humidity_state = use_state(|| value.clone());
    let humidity_changed = humidity_changed.clone();

    let number_changed = {
        let relative_humidity_state = relative_humidity_state.clone();

        Callback::from(move |number: u8| {
            let relative_humidity = RelativeHumidity::new(number);
            relative_humidity_state.set(relative_humidity.clone());
            humidity_changed.emit(relative_humidity);
        })
    };

    html! {
        <NumberInput<u8>
            min={MIN_HUMIDITY}
            max={MAX_HUMIDITY}
            step={1u8}
            value={relative_humidity_state.value()}
            {number_changed}
        />
    }
}

#[derive(Properties, PartialEq)]
struct NumberInputProps<T>
where T: Number {
    #[prop_or_default]
    value: T,
    #[prop_or_default]
    min: T,
    #[prop_or_default]
    max: T,
    #[prop_or_default]
    step: T,
    #[prop_or_default]
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
