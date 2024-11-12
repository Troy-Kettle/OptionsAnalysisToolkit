use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Options Analysis Toolkit" }</h1>
                <p>{ "Welcome to the Options Analysis Toolkit!" }</p>
                <button onclick={_ctx.link().callback(|_| ())}>{ "Click Me" }</button>
            </div>
        }
    }
}
