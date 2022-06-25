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
        Self {
            max_time: 0,
            time_left: 0,
            timeout: None,
            stop_notify: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TimerMsg::ResumeTimer => self.tick(ctx).is_none(),
            TimerMsg::PauseTimer => self.timeout.take().is_some(),
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
                        if let Some(ref on_finish) = ctx.props().on_finish {
                            on_finish.emit(());
                        }
                        self.stop_notify = true;
                    }
                    self.timeout = None;
                } else {
                    self.time_left -= 1;
                    self.tick(ctx);
                }
                true
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
                <p>{format!("Time Left: {}s", self.time_left)}</p>
                { for ctx.props().children.iter() }
                <div class={css!("
                    width: 100%;
                    text-align: center;
                    button { display: inline; margin: auto 10px; border-radius: 50%; border: 0px; }
                ")} >
                    if self.timeout.is_some() {
                        <button style="background: LightCoral;" onclick={pause}>{ "||" }</button>
                    } else {
                        <button style="background: LightGreen;" disabled={self.time_left == 0} onclick={resume}>
                            { ">" }
                        </button>
                    }
                    <progress value={self.time_left.to_string()} max={self.max_time.to_string()}
                        class={css!(r#"width: 70%; margin: 5px 10px;"#)}/>
                </div>
            </>
        }
    }
}
