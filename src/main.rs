use components::pomodoro::Pomodoro;
use yew::prelude::*;

pub mod components;
pub mod tauri;
pub mod utils;

#[function_component(App)]
fn app() -> Html {
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

fn main() {
    yew::Renderer::<App>::new().render();
}
