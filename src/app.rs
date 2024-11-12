use yew::prelude::*;
use crate::components::{header::Header, counter::Counter, greeks_table::GreeksTable};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <Header />
            <Counter />
            <GreeksTable />
        </div>
    }
}
