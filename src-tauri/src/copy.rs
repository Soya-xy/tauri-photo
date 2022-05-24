use arboard::Clipboard;
use std::fmt;
use crate::setup::{ClipboardValue,ImageData};
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
pub fn get_image() -> ClipboardValue {
    let mut clipboard = Clipboard::new().expect("Failed to create clipboard!");
    let image = clipboard.get_image().unwrap();
    ClipboardValue::Image(ImageData {
        width: image.width,
        height: image.height,
        bytes: Vec::from(&*image.bytes)
    })
}
