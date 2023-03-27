use crate::components::pomodoro::Pomodoro;
use crate::components::title_bar::TitleBar;
use stylist::yew::use_style;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let class = use_style!("display: flex; flex-direction: column;");
    html! {
        <div {class} >
            <TitleBar />
            <Pomodoro />
        </div>
    }
}
