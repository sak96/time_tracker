use crate::utils::weak_component_link::WeakComponentLink;
use gloo::timers::callback::Timeout;
use print_duration::print_duration;
use std::time::Duration;
use stylist::css;
use yew::prelude::*;

#[derive(Default)]
pub struct Timer {
    time_left: Option<Duration>,
    max_time: u32,
    timeout: Option<Timeout>,
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
    pub on_finish: Option<Callback<()>>,
    #[prop_or_default]
    pub children: Children,
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
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TimerMsg::ResumeTimer => self.tick(ctx).is_none(),
            TimerMsg::PauseTimer => self.timeout.take().is_some(),
            TimerMsg::ResetTimer(max_time) => {
                self.max_time = max_time;
                self.time_left = Some(Duration::from_secs(max_time as u64));
                self.tick(ctx);
                true
            }
            TimerMsg::CountDown => {
                if let Some(duration) = self.time_left.as_mut() {
                    self.time_left = duration.checked_sub(Duration::from_secs(1));
                    if self.time_left.is_none() {
                        if let Some(ref on_finish) = ctx.props().on_finish {
                            on_finish.emit(());
                        }
                        self.timeout.take();
                    } else {
                        self.tick(ctx);
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let pause = {
            let link = ctx.link().clone();
            move |_| {
                link.send_message(TimerMsg::PauseTimer);
            }
        };
        let resume = {
            let link = ctx.link().clone();
            move |_| {
                link.send_message(TimerMsg::ResumeTimer);
            }
        };
        html! {
            <>
                if let Some(duration) = self.time_left {
                    <p>{format!("Time Left: {}s", print_duration(duration, 0..3))}</p>
                    { for ctx.props().children.iter() }
                    <div class={css!("
                        width: 100%;
                        text-align: center;
                        button { display: inline; margin: auto 10px; border-radius: 50%; border: 0px; }
                    ")} >
                        if self.timeout.is_some() {
                            <button style="background: LightCoral;" onclick={pause}>{ "||" }</button>
                        } else {
                            <button style="background: LightGreen;" onclick={resume}>{ ">" }</button>
                        }
                        <progress value={duration.as_secs().to_string()} max={self.max_time.to_string()}
                            class={css!(r#"width: 70%; margin: 5px 10px;"#)}/>
                    </div>
                } else {
                    <p>{"Timer Ended!"}</p>
                }
            </>
        }
    }
}
