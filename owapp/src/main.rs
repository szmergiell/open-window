use owlib::open_window::{measurement::Measurement, temperature::Temperature, relative_humidity::RelativeHumidity};
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

    html! {
        <>
            <h1>{ "Open Window App" }</h1>
            <MeasurementComponent
                temperature={indoor_measurement.temperature.clone()}
                relative_humidity={indoor_measurement.relative_humidity.clone()}
            />
        </>
    }
}

#[derive(Properties, PartialEq)]
struct MeasurementProps {
    temperature: Temperature,
    relative_humidity: RelativeHumidity
}

#[function_component(MeasurementComponent)]
fn measurement(MeasurementProps { temperature, relative_humidity }: &MeasurementProps) -> Html {
    let temperature_state = use_state(|| *temperature);
    let relative_humidity_state = use_state(|| *relative_humidity);

    let measurement_memo = use_memo(|(temperature, relative_humidity)| {
        Measurement {
            temperature: *temperature,
            relative_humidity: *relative_humidity
        }
    }, (*temperature_state, *relative_humidity_state));

    let dew_point_memo = use_memo(|measurement| {
        measurement.calculate_dew_point()
    }, *measurement_memo);

    let humidity_change = {
        let relative_humidity_state = relative_humidity_state.clone();
        Callback::from(move |number: u8| {
            relative_humidity_state.set(RelativeHumidity::new(number));
        })
    };

    html! {
        <>
            <TemperatureComponent value={temperature_state.value()} />
            <RelativeHumidityComponent value={relative_humidity_state.value()} humidity_change={humidity_change} />
            <input type="number" value={dew_point_memo.to_string()}/>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct TemperatureProps {
    value: f64
}

#[function_component(TemperatureComponent)]
fn temperature(TemperatureProps { value }: &TemperatureProps) -> Html {
    html! {
        <>
            <input type="number" min="-100" max="100" step="0.1" value={value.to_string()}/>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct RelativeHumidityProps {
    value: u8,
    humidity_change: Callback<u8>
}

#[function_component(RelativeHumidityComponent)]
fn relative_humidity(RelativeHumidityProps { value, humidity_change }: &RelativeHumidityProps) -> Html {

    let internal_onchange = {
        let humidity_change = humidity_change.clone();

        Callback::from(move |e: Event| {
            let target = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let string_value = input.value();
                let value_result = string_value.parse::<u8>();
                if let Ok(number) = value_result {
                    humidity_change.emit(number);
                }
            }
        })
    };

    html! {
        <>
            <input type="number" min="1" max="100" step="1" value={value.to_string()} onchange={internal_onchange}/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
