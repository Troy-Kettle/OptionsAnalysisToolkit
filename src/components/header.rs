// src/components/header.rs
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <h1>{ "Options Analysis Toolkit" }</h1>
            <p>{ "Welcome to the Options Analysis Toolkit!" }</p>
        </header>
    }
}

