#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod copy;
mod menu;

fn main() {
    tauri::Builder::default()
        .menu(menu::get_menu())
        .system_tray(menu::get_menu_tray())
        .invoke_handler(tauri::generate_handler![copy::hello])
        .on_system_tray_event(menu::click_tray_item)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
