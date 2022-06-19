#[path="../pomodoro.rs"]
mod pomodoro;

use relm4::RelmApp;
use pomodoro::AppModel;

fn main() {
    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run();
}
