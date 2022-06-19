use core::time;
use relm4::{
    gtk::traits::{
        BoxExt, ButtonExt, CheckButtonExt, DialogExt, GtkWindowExt, OrientableExt, WidgetExt,
    },
    gtk::{self, Align},
    send, AppUpdate, ComponentUpdate, MessageHandler, Model, RelmComponent, RelmMsgHandler, Sender,
    WidgetPlus, Widgets,
};

use std::sync::mpsc::{channel, TryRecvError};

use std::thread;

struct TimerHandler {
    sender: std::sync::mpsc::Sender<bool>,
}

#[derive(Debug)]
enum Status {
    Focus(u32),
    LongBreak,
    ShortBreak(u32),
}

impl Status {
    const FOCUS_DURATION: u32 = 60 * 45;
    const LONG_BREAK_DURATION: u32 = 60 * 15;
    const SHORT_BREAK_DURATION: u32 = 60 * 5;
    const SHORT_BREAK_PER_LONG_BREAK: u32 = 3;
    pub const EXTEND_DURATION: u32 = 60 * 5;

    pub fn next(&mut self) {
        *self = self.try_next()
    }

    pub fn time_duration(&self) -> u32 {
        match self {
            Status::Focus(_) => Self::FOCUS_DURATION,
            Status::LongBreak => Self::LONG_BREAK_DURATION,
            Status::ShortBreak(_) => Self::SHORT_BREAK_DURATION,
        }
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

pub struct AppModel {
    is_paused: bool,
    is_notify: bool,
    is_auto_next_task: bool,
    timer: u32,
    status: Status,
}

impl Default for AppModel {
    fn default() -> Self {
        let status = Status::Focus(0);
        Self {
            is_paused: false,
            is_notify: true,
            is_auto_next_task: false,
            timer: status.time_duration(),
            status,
        }
    }
}

impl AppModel {
    #[inline]
    fn show_time(duration: u32) -> String {
        let seconds = duration % 60;
        let minutes = (duration / 60) % 60;
        let hours = (duration / 60) / 60;
        format!("{}:{}:{}", hours, minutes, seconds)
    }

    pub fn get_label(&self) -> String {
        format!(
            "Timer {} / {} {}| Status: {:?}",
            Self::show_time(self.timer),
            Self::show_time(self.status.time_duration()),
            if self.is_paused {
                " Paused "
            } else {
                " Running "
            },
            self.status
        )
    }

    #[inline]
    pub fn reset_timer_if_empty(&mut self) -> bool {
        if self.timer == 0 {
            self.status.next();
            self.timer = self.status.time_duration();
            if !self.is_auto_next_task {
                self.is_paused = true;
                if self.is_notify {
                    return true;
                }
            }
        }
        false
    }
}

pub enum AppMsg {
    ResumeTimer,
    CountDown,
    PauseTimer,
    ResetTimer,
    NotifyToggled(bool),
    AutoNextTaskToggled(bool),
    ExtendStage,
    NextStage,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

#[relm4::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        gtk::ApplicationWindow {
            set_title: Some("Simple app"),
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,
                append = &gtk::ProgressBar {
                    set_margin_all: 5,
                    set_fraction: watch! {
                        model.timer as f64 / model.status.time_duration() as f64
                    },
                },
                append = &gtk::Label {
                    set_margin_all: 5,
                    set_label: watch! { &model.get_label() },
                },
                append = &gtk::CheckButton {
                    set_halign: Align::Center,
                    set_margin_all: 5,
                    set_label: Some("Auto Next Task"),
                    connect_toggled(sender) => move |b| {
                        send!(sender, AppMsg::AutoNextTaskToggled(b.is_active()));
                    },
                },
                append = &gtk::CheckButton {
                    set_halign: Align::Center,
                    set_active: model.is_notify,
                    set_visible: watch! {!model.is_auto_next_task},
                    set_margin_all: 5,
                    set_label: Some("Notify End of Task"),
                    connect_toggled(sender) => move |b| {
                        send!(sender, AppMsg::NotifyToggled(b.is_active()));
                    },
                },
                append = &gtk::Button {
                    set_margin_all: 5,
                    set_label: "Resume",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::ResumeTimer);
                    },
                },
                append = &gtk::Button {
                    set_margin_all: 5,
                    set_label: "Reset",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::ResetTimer);
                    },
                },
                append = &gtk::Button {
                    set_margin_all: 5,
                    set_label: "Pause",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::PauseTimer);
                    },
                },
                append = &gtk::Button {
                    set_margin_all: 5,
                    set_label: "Next",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::NextStage);
                    },
                }
            },
        }
    }
}
struct DialogModel {
    hidden: bool,
}

enum DialogMsg {
    Show,
    Accept,
    PartialAccept,
    Cancel,
}

impl Model for DialogModel {
    type Msg = DialogMsg;
    type Widgets = DialogWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for DialogModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        DialogModel { hidden: true }
    }

    fn update(
        &mut self,
        msg: DialogMsg,
        _components: &(),
        _sender: Sender<DialogMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            DialogMsg::Show => self.hidden = false,
            DialogMsg::Accept => {
                self.hidden = true;
                send!(parent_sender, AppMsg::NextStage);
            }
            DialogMsg::PartialAccept => {
                self.hidden = true;
                send!(parent_sender, AppMsg::ExtendStage);
            }
            DialogMsg::Cancel => self.hidden = true,
        }
    }
}

#[relm4::widget]
impl Widgets<DialogModel, AppModel> for DialogWidgets {
    view! {
        dialog = gtk::MessageDialog {
            set_transient_for: parent!(Some(&parent_widgets.root_widget())),
            set_modal: true,
            set_visible: watch!(!model.hidden),
            set_text: Some("Do you want to move to next task?"),
            set_secondary_text: Some("You can unset notification from the app by toggling the check button."),
            add_button: args!("Start Next Task", gtk::ResponseType::Accept),
            add_button: args!("Extend Timer", gtk::ResponseType::Other(0)),
            add_button: args!("Pause Timer", gtk::ResponseType::Cancel),
            connect_response(sender) => move |_, resp| {
                send!(sender, match resp  {
                    gtk::ResponseType::Accept => DialogMsg::Accept,
                    gtk::ResponseType::Other(0) => DialogMsg::PartialAccept,
                    _ => DialogMsg::Cancel,
                });
            }
        }
    }
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::CountDown => {
                self.timer -= 1;
                if self.reset_timer_if_empty() {
                    components.dialog.send(DialogMsg::Show).unwrap();
                }
            }
            AppMsg::ResumeTimer => {
                self.is_paused = false;
                self.reset_timer_if_empty();
                components.timer_handler.send(true);
            }
            AppMsg::ResetTimer => {
                self.is_paused = false;
                self.timer = self.status.time_duration();
                components.timer_handler.send(true);
            }
            AppMsg::PauseTimer => {
                self.is_paused = true;
            }
            AppMsg::NextStage => {
                self.timer = 0;
                self.reset_timer_if_empty();
                self.is_paused = false;
            }
            AppMsg::ExtendStage => {
                self.timer = Status::EXTEND_DURATION;
                self.is_paused = false;
            }
            AppMsg::NotifyToggled(is_notify) => self.is_notify = is_notify,
            AppMsg::AutoNextTaskToggled(is_auto_next_task) => {
                self.is_auto_next_task = is_auto_next_task
            }
        }
        components.timer_handler.send(!self.is_paused);
        true
    }
}
#[derive(relm4::Components)]
pub struct AppComponents {
    timer_handler: RelmMsgHandler<TimerHandler, AppModel>,
    dialog: RelmComponent<DialogModel, AppModel>,
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
