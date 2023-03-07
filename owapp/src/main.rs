use owlib::open_window::{measurement::Measurement, temperature::Temperature, relative_humidity::RelativeHumidity};
use web_sys::HtmlInputElement;
use yew::prelude::*;

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
    let dew_point = use_state(|| {
        Measurement {
            temperature: temperature.clone(),
            relative_humidity: relative_humidity.clone()
        }.calculate_dew_point()
    });

    html! {
        <>
            <TemperatureComponent value={temperature.value()} />
            <RelativeHumidityComponent value={relative_humidity.value()} />
            <input type="number" value={dew_point.to_string()}/>
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
    value: u8
}

#[function_component(RelativeHumidityComponent)]
fn relative_humidity(RelativeHumidityProps { value }: &RelativeHumidityProps) -> Html {

    let oninput: Callback<InputEvent> = Callback::from(move |e: InputEvent| {
        let input_el: HtmlInputElement = e.target_unchecked_into();
    });

    html! {
        <>
            <input type="number" min="0" max="100" step="1" value={value.to_string()} {oninput}/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
