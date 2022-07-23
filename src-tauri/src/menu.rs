use tauri::{
    AppHandle, CustomMenuItem, Manager, Menu, MenuItem, Submenu, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem
};

fn open_activation_window(app: &AppHandle, url: String) -> () {
    println!("hello: {}", url);
    if let Some(window) = app.get_window("photo") {
        let _ = window.set_focus();
        let _ = window.center();
    } else {
        let _window =
            tauri::window::WindowBuilder::new(app, "photo", tauri::WindowUrl::App(url.into()))
                .title("Activate Recut")
                .center()
                .focus()
                .inner_size(500.0, 300.0)
                .resizable(false)
                .build()
                .map_err(|e| e.to_string());
        // hide the menu
    }
}

pub fn get_menu() -> Menu {
    // create a submenu
    #[cfg(target_os = "macos")]
    let my_app_menu = Menu::new()
        .add_native_item(MenuItem::CloseWindow)
        .add_native_item(MenuItem::Quit);

    let edit_menu = Submenu::new(
        "编辑",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    );

    // add all our childs to the menu (order is how they'll appear)
    Menu::new()
        .add_submenu(Submenu::new("Menu", my_app_menu))
        .add_submenu(edit_menu)
}

pub fn get_menu_tray() -> SystemTray {
    let setting = CustomMenuItem::new("setting".to_string(), "设置");
    let upload = CustomMenuItem::new("upload".to_string(), "上传");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let tray_menu = SystemTrayMenu::new()
        .add_item(setting)
        .add_item(upload)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

// tray icon click handler
pub fn click_tray_item(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            print!("{}", id);
            match id.as_str() {
                "setting" => {
                    open_activation_window(app, "http://10.141.141.81:31740".to_string());
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}
