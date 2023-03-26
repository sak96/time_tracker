use crate::tauri::{invoke, listen, log, notify};
use print_duration::print_duration;
use serde::Serialize;
use std::time::Duration;
use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;

#[derive(PartialEq)]
enum Status {
    Focus(u32),
    LongBreak,
    ShortBreak(u32),
}

impl Default for Status {
    fn default() -> Self {
        Self::LongBreak.try_next()
    }
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
    const FOCUS_DURATION: u64 = 45;
    const LONG_BREAK_DURATION: u64 = 15;
    const SHORT_BREAK_DURATION: u64 = 5;
    const SHORT_BREAK_PER_LONG_BREAK: u32 = 3;

    pub fn time_duration(&self) -> u64 {
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

#[derive(Serialize)]
pub struct StartTimerArgs {
    duration: u64,
}

#[function_component(Pomodoro)]
pub fn pomodoro() -> Html {
    let status = use_state(Status::default);
    let time_left_handler = use_state(|| Duration::from_secs(status.time_duration()));
    let running = use_state(|| true);
    {
        let time_left_handler = time_left_handler.clone();
        use_effect_with_deps(
            move |status| time_left_handler.set(Duration::from_secs(status.time_duration())),
            status.clone(),
        );
    }
    {
        let status = status.clone();
        let time_left_handler = time_left_handler.clone();
        use_effect_with_deps(
            move |running| {
                let running = running.clone();
                let handler = if *running {
                    log("start_timer");
                    let start_timer = StartTimerArgs {
                        duration: time_left_handler.as_secs(),
                    };
                    spawn_local(async move {
                        invoke("plugin:timer|start_timer", start_timer)
                            .await
                            .expect("failed to invoke start timer")
                    });
                    let handler = listen(
                        "tick",
                        Box::new(move |time_left: u64| {
                            log("tick");
                            let time_left = Duration::from_secs(time_left);
                            if time_left.is_zero() {
                                time_left_handler.set(time_left);
                                notify(&format!("{} ended", *status)).expect("notify failed");
                                running.set(false);
                                status.set(status.try_next())
                            } else {
                                time_left_handler.set(time_left);
                            };
                        }),
                    );
                    Some(handler)
                } else {
                    spawn_local(async move {
                        invoke("plugin:timer|stop_timer", ())
                            .await
                            .expect("failed to invoke start timer")
                    });
                    None
                };
                move || {
                    log("closing handler");
                    drop(handler)
                }
            },
            running.clone(),
        );
    }
    let (icon, color) = if *running {
        ("pause", "red")
    } else {
        ("play_arrow", "green")
    };
    html! {
        <>
            <p>{format!("Time Left: {}s", print_duration(*time_left_handler, 0..3))}</p>
            <p>{format!("Current Status: {}", *status)}</p>
            <div >
                <button class={classes!("btn-floating", color)} onclick={move |_| {running.set(!*running)}}>
                    <i class="material-icons">{icon}</i>
                </button>
                <progress value={time_left_handler.as_secs().to_string()} max={status.time_duration().to_string()}
                 style="width: 91%"/>
            </div>
        </>
    }
}
