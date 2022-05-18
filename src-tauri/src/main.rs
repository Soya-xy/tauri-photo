mod copy;
mod menu;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {

            // listen to the `event-name` (emitted on any window)
             app.listen_global("tauri://file-drop-hover", |event| {
                println!("got event-name with payload {:?}", event.payload());
            });

            Ok(())
        })
        .menu(menu::get_menu())
        .system_tray(menu::get_menu_tray())
        .invoke_handler(tauri::generate_handler![copy::hello])
        .on_system_tray_event(menu::click_tray_item)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
