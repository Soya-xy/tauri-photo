// #[tauri::command]

pub fn open_activation_window(app: tauri::AppHandle, url: String) {
    if let Some(window) = app.get_window(ACTIVATION_WINDOW_LABEL) {
        let _ = window.set_focus();
        let _ = window.center();
    } else {
        let window = tauri::window::WindowBuilder::new(
            &app,
            ACTIVATION_WINDOW_LABEL,
            tauri::WindowUrl::App(url.into()),
        )
        .title("Activate Recut")
        .center()
        .focus()
        .inner_size(500.0, 300.0)
        .resizable(false)
        .build()
        .map_err(|e| e.to_string())?;

        // hide the menu
        let _ = window.menu_handle().toggle();
    }
}
