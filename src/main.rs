use components::app::App;

pub mod components;
pub mod tauri;
pub mod utils;

fn main() {
    yew::Renderer::<App>::new().render();
}
