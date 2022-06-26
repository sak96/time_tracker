use components::pomodoro::Pomodoro;
use yew::prelude::*;

pub mod components;
pub mod tauri;
pub mod utils;

struct App {}

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <div class="card grey darken-3 center">
                    <h1 class="card-title">{ "Pomodoro Timer" }</h1>
                    <div class="card-content">
                        <Pomodoro />
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
