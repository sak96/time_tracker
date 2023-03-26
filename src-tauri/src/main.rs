#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod timer_plugin;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(timer_plugin::init())
        .run(context)
        .expect("error while running tauri application");
}
