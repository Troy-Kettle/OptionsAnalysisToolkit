use yew::prelude::*;
use crate::components::{header::Header, counter::Counter}; // Import Header and Counter directly

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <Header />
            <Counter />
        </div>
    }
}
