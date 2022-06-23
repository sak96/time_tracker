use crate::utils::weak_component_link::WeakComponentLink;
use components::timer::{Timer, TimerMsg};
use stylist::css;
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
        let timer_link = WeakComponentLink::default();
        let reset_timer = {
            let timer_link = timer_link.clone();
            move |_| {
                timer_link
                    .borrow_clone_unwrap()
                    .send_message(TimerMsg::ResetTimer(100))
            }
        };
        let pause_timer = {
            let timer_link = timer_link.clone();
            move |_| {
                timer_link
                    .borrow_clone_unwrap()
                    .send_message(TimerMsg::PauseTimer)
            }
        };
        let resume_timer = {
            let timer_link = timer_link.clone();
            move |_| {
                timer_link
                    .borrow_clone_unwrap()
                    .send_message(TimerMsg::ResumeTimer)
            }
        };

        html! {
            <div
                class={css!(r#"
                    display: flex;
                    flex-wrap: wrap;
                    align-items: center;
                    flex-direction: column;
                    justify-content: center;
                "#)}
            >
                <h1>{ "Pomodoro Timer" }</h1>
                <Timer weak_link={timer_link}></Timer>
                <div
                class={css!(r#"
                    display: flex;
                    flex-wrap: wrap;
                    button {
                        margin: 10px;
                    }
                "#)}
                >
                    <button onclick={reset_timer}>{ "Reset" }</button>
                    <button onclick={pause_timer}>{ "Pause" }</button>
                    <button onclick={resume_timer}>{ "Resume" }</button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
