#[tauri::command]

pub fn hello(your_name: String) {
  println!("hello: {}", your_name);
}
