use core::time;
use gtk::traits::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use gtk::ApplicationWindow;
use relm4::{
    send, AppUpdate, MessageHandler, Model, RelmApp, RelmMsgHandler, Sender, WidgetPlus, Widgets,
};

use std::sync::mpsc::{channel, TryRecvError};

use std::thread;

struct TimerHandler {
    sender: std::sync::mpsc::Sender<bool>,
}

#[derive(Default)]
struct AppModel {
    timer: Option<u32>,
}

enum AppMsg {
    StartTimer,
    CountDown,
    StopTimer,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        ApplicationWindow {
            set_title: Some("Simple app"),
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,
                append = &gtk::ProgressBar {
                    set_margin_all: 5,
                    set_fraction: watch! {
                        match model.timer {
                            Some(timer) => timer as f64 / 100.0,
                            _=> 100.0,
                        }
                    },
                },
                append = &gtk::Label {
                    set_margin_all: 5,
                    set_label: watch! { &format!("Timer: {:?}", model.timer) },
                },
                append = &gtk::Button {
                    set_margin_all: 5,
                    set_label: "Start",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::StartTimer);
                    },
                },
                append = &gtk::Button {
                    set_margin_all: 5,
                    set_label: "Stop",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::StopTimer);
                    },
                }
            },
        }
    }
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::CountDown => {
                if let Some(t) = &mut self.timer {
                    *t -= 1;
                    if *t == 0 {
                        self.timer = None;
                    }
                }
            }
            AppMsg::StartTimer => {
                components.timer_handler.send(true);
                self.timer = Some(100);
            }
            AppMsg::StopTimer => {
                self.timer = None;
                components.timer_handler.send(true);
            }
        }
        true
    }
}
#[derive(relm4::Components)]
struct AppComponents {
    timer_handler: RelmMsgHandler<TimerHandler, AppModel>,
}

impl MessageHandler<AppModel> for TimerHandler {
    type Msg = bool;
    type Sender = std::sync::mpsc::Sender<Self::Msg>;

    fn init(_parent_model: &AppModel, parent_sender: Sender<AppMsg>) -> Self {
        let (sender, receiver) = channel();
        let timer = move || {
            let mut active = false;
            loop {
                match receiver.try_recv() {
                    Ok(active_) => active = active_,
                    Err(TryRecvError::Empty) => {
                        if active {
                            thread::sleep(time::Duration::from_secs(1));
                            send!(parent_sender, AppMsg::CountDown);
                            continue;
                        }
                        match receiver.recv() {
                            Ok(active_) => active = active_,
                            Err(_) => break,
                        }
                    }
                    _ => break,
                }
            }
        };
        thread::spawn(timer);
        TimerHandler { sender }
    }

    fn send(&self, msg: Self::Msg) {
        self.sender.send(msg).unwrap();
    }

    fn sender(&self) -> Self::Sender {
        self.sender.clone()
    }
}

fn main() {
    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run();
}
