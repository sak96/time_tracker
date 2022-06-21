use crate::utils::weak_component_link::WeakComponentLink;
use gloo::timers::callback::Timeout;
use yew::prelude::*;

pub struct Timer {
    time_left: u32,
    timeout: Option<Timeout>,
}

pub enum TimerMsg {
    ResumeTimer,
    PauseTimer,
    ResetTimer,
    CountDown,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TimerProps {
    pub weak_link: WeakComponentLink<Timer>,
}

impl Timer {
    pub const MAX_TIME: u32 = 100;
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
            time_left: 0,
            timeout: None,
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
            TimerMsg::ResetTimer => {
                self.time_left = Self::MAX_TIME;
                self.tick(ctx);
                true
            }
            TimerMsg::CountDown => {
                self.time_left = self.time_left.saturating_sub(1);
                self.tick(ctx);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <progress value={self.time_left.to_string()} max={Self::MAX_TIME.to_string()}/>
                <p>{"Time Left: "}{self.time_left.to_string()}</p>
            </>
        }
    }
}
