#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod clip;
mod copy;
mod menu;
mod setup;
mod watcher;

fn main() {
    tauri::Builder::default()
        .setup(setup::watcher_clip)
        .menu(menu::get_menu())
        .system_tray(menu::get_menu_tray())
        .invoke_handler(tauri::generate_handler![copy::get_image,copy::set_image])
        .on_system_tray_event(menu::click_tray_item)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
