use super::timer::{Timer, TimerMsg};
use crate::tauri::notify;
use crate::utils::weak_component_link::WeakComponentLink;
use yew::prelude::*;

pub struct Pomodoro {
    status: Status,
    timer_link: WeakComponentLink<Timer>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PomodoroProps {}

enum Status {
    Focus(u32),
    LongBreak,
    ShortBreak(u32),
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Focus(session) => write!(f, "Focus Session {}", session),
            Status::LongBreak => write!(f, "Long Break !!!"),
            Status::ShortBreak(session) => write!(f, "Short Break {}", session),
        }
    }
}

impl Status {
    const FOCUS_DURATION: u32 = 45;
    const LONG_BREAK_DURATION: u32 = 15;
    const SHORT_BREAK_DURATION: u32 = 5;
    const SHORT_BREAK_PER_LONG_BREAK: u32 = 3;

    pub fn next(&mut self) {
        *self = self.try_next()
    }

    pub fn time_duration(&self) -> u32 {
        (match self {
            Status::Focus(_) => Self::FOCUS_DURATION,
            Status::LongBreak => Self::LONG_BREAK_DURATION,
            Status::ShortBreak(_) => Self::SHORT_BREAK_DURATION,
        }) * 60
    }

    fn try_next(&self) -> Self {
        match self {
            Status::Focus(breaks) => {
                if *breaks == Self::SHORT_BREAK_PER_LONG_BREAK {
                    Status::LongBreak
                } else {
                    Status::ShortBreak(*breaks)
                }
            }
            Status::LongBreak => Self::Focus(0),
            Status::ShortBreak(breaks) => Status::Focus(breaks + 1),
        }
    }
}

pub enum PomodoroMsg {
    NotifyToggled(bool),
    AutoNextTaskToggled(bool),
    ExtendStage,
    NextStage,
}

impl Component for Pomodoro {
    type Message = PomodoroMsg;

    type Properties = PomodoroProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let timer_link = WeakComponentLink::default();
        Self {
            status: Status::LongBreak.try_next(),
            timer_link,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PomodoroMsg::NotifyToggled(_) => todo!(),
            PomodoroMsg::AutoNextTaskToggled(_) => todo!(),
            PomodoroMsg::ExtendStage => todo!(),
            PomodoroMsg::NextStage => {
                if notify(format!("Timer Expired for {}", self.status)).is_err() {
                    gloo::console::log!("timer expired!");
                };
                self.status.next();
                self.timer_link
                    .borrow_clone_unwrap()
                    .send_message(TimerMsg::ResetTimer(self.status.time_duration()));
                true
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.timer_link
                .borrow_clone_unwrap()
                .send_message(TimerMsg::ResetTimer(self.status.time_duration()))
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let timer_link = self.timer_link.clone();
        let on_finish = Some(Callback::from({
            let link = ctx.link().clone();
            move |_| link.send_message(Self::Message::NextStage)
        }));
        html! {
            <Timer weak_link={timer_link} on_finish={on_finish}>
                <p>{format!("Current Status: {}", self.status)}</p>
            </Timer>
        }
    }
}
