use yew::prelude::*;

struct App {}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ "Hello World" }</h1>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
