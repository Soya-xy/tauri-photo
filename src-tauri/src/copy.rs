#[tauri::command]
pub fn my_custom_command(invoke_message: String) -> String {
    ("Hello from Rust! {}", invoke_message).into()
}
