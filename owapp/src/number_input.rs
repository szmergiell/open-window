use std::{fmt::Display, str::FromStr};

use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Event};
use yew::{function_component, Html, Callback, html, Properties};

pub trait Number: PartialEq + PartialOrd + Default + FromStr + Display + Clone + 'static {}

#[derive(Properties, PartialEq)]
pub struct NumberInputProps<T>
where T: Number {
    #[prop_or_default]
    pub value: T,
    #[prop_or_default]
    pub min: T,
    #[prop_or_default]
    pub max: T,
    #[prop_or_default]
    pub step: T,
    #[prop_or_default]
    pub number_changed: Callback<T>
}

#[function_component]
pub fn NumberInput<T>(NumberInputProps { value, min, max, step, number_changed }: &NumberInputProps<T>) -> Html
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
                input.set_value(&(number.to_string()));
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
