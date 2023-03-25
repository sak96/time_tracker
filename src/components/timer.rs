use crate::utils::weak_component_link::WeakComponentLink;
use gloo::timers::callback::Timeout;
use instant::SystemTime;
use print_duration::print_duration;
use std::time::Duration;
use yew::prelude::*;

#[derive(Default)]
pub struct Timer {
    start_time: Option<SystemTime>,
    time_left: Duration,
    max_time: Duration,
    paused_time: Duration,
    timeout: Option<Timeout>,
}

#[derive(Clone)]
pub enum TimerMsg {
    ResumeTimer,
    PauseTimer,
    ResetTimer(u64),
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
            TimerMsg::ResumeTimer => {
                if self.tick(ctx).is_none() {
                    if let Some(time) = self.start_time {
                        self.paused_time = SystemTime::now()
                            .duration_since(time)
                            .expect("rewind of clock not supported")
                            .saturating_add(self.time_left)
                            .checked_sub(self.max_time)
                            .unwrap_or(Duration::ZERO);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            TimerMsg::PauseTimer => self.timeout.take().is_some(),
            TimerMsg::ResetTimer(max_time) => {
                self.start_time = Some(SystemTime::now());
                self.max_time = Duration::from_secs(max_time);
                self.time_left = self.max_time;
                self.tick(ctx);
                true
            }
            TimerMsg::CountDown => {
                if let Some(time) = self.start_time {
                    self.tick(ctx);
                    if let Some(time_left) =
                        self.max_time.saturating_add(self.paused_time).checked_sub(
                            SystemTime::now()
                                .duration_since(time)
                                .expect("rewind of clock not supported"),
                        )
                    {
                        if time_left != self.time_left {
                            // time changed
                            self.time_left = time_left;
                            return true;
                        }
                    } else {
                        // time ended
                        if let Some(ref on_finish) = ctx.props().on_finish {
                            on_finish.emit(());
                        }
                        self.timeout.take();
                        self.start_time.take();
                        return true;
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (icon, color, msg) = if self.timeout.is_some() {
            ("pause", "red", TimerMsg::PauseTimer)
        } else {
            ("play_arrow", "green", TimerMsg::ResumeTimer)
        };
        let onclick = {
            let link = ctx.link().clone();
            move |_| {
                link.send_message(msg.clone());
            }
        };
        html! {
            <>
                if self.start_time.is_some() {
                    <p>{format!("Time Left: {}s", print_duration(self.time_left, 0..3))}</p>
                    { for ctx.props().children.iter() }
                    <div >
                        <button class={classes!("btn-floating", color)} onclick={onclick}>
                            <i class="material-icons">{icon}</i>
                        </button>
                        <progress value={self.time_left.as_secs().to_string()} max={self.max_time.as_secs().to_string()}
                         style="width: 90%"/>
                    </div>
                } else {
                    <p>{"Timer Ended!"}</p>
                }
            </>
        }
    }
}
