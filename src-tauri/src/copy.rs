use arboard::Clipboard;
use std::fmt;

struct ClipboardContent {
    width: usize,
    height: usize,
    image: Vec<u8>,
}

impl fmt::Display for ClipboardContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\"width\":{}, \"height\":{},\"image\":{:?}}}",
            self.width, self.height, self.image
        )
    }
}

#[tauri::command]
pub fn hello() -> String {
    let mut clipboard = Clipboard::new().expect("Failed to create clipboard!");
    let image = clipboard.get_image().unwrap();
    let cli = ClipboardContent {
        width: image.width,
        height: image.height,
        image: image.bytes.to_vec(),
    };
    cli.to_string()
}
