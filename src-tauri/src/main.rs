use crate::{copy}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![copy::my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
