use crate::tauri::notify;
use crate::utils::weak_component_link::WeakComponentLink;
use gloo::timers::callback::Timeout;
use stylist::css;
use yew::prelude::*;

pub struct Timer {
    time_left: u32,
    max_time: u32,
    timeout: Option<Timeout>,
    stop_notify: bool,
}

pub enum TimerMsg {
    ResumeTimer,
    PauseTimer,
    ResetTimer(u32),
    CountDown,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TimerProps {
    pub weak_link: WeakComponentLink<Timer>,
}

impl Timer {
    fn tick(&mut self, ctx: &Context<Self>) -> Option<Timeout> {
        let handle = {
            let link = ctx.link().clone();
            Timeout::new(1000, move || link.send_message(TimerMsg::CountDown))
        };
        self.timeout.replace(handle)
    }
}

impl Component for Timer {
    type Message = TimerMsg;

    type Properties = TimerProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props()
            .weak_link
            .borrow_mut()
            .replace(ctx.link().clone());
        Self {
            max_time: 0,
            time_left: 0,
            timeout: None,
            stop_notify: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TimerMsg::ResumeTimer => {
                self.tick(ctx);
                false
            }
            TimerMsg::PauseTimer => {
                self.timeout.take();
                false
            }
            TimerMsg::ResetTimer(max_time) => {
                self.max_time = max_time;
                self.time_left = max_time;
                self.tick(ctx);
                self.stop_notify = false;
                true
            }
            TimerMsg::CountDown => {
                if self.time_left == 0 {
                    if !self.stop_notify {
                        if notify("Timer Expired".to_string()).is_err() {
                            gloo::console::log!("timer expired!");
                        };
                        self.stop_notify = true;
                    }
                } else {
                    self.time_left -= 1;
                }
                self.tick(ctx);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <progress value={self.time_left.to_string()} max={self.max_time.to_string()}
                    class={css!(r#"
                        border-radius: 20%;
                        height: 10px;
                        width: 70%;
                    "#)}
                />
                <p>{"Time Left: "}{self.time_left.to_string()}</p>
            </>
        }
    }
}
