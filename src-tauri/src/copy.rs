use crate::clip::{ClipboardValue, ImageData};
use arboard::{Clipboard, ImageData as ArImageData};
use std::borrow::Cow;

#[tauri::command]
pub fn get_image() -> Result<ClipboardValue, String> {
    let mut clipboard = Clipboard::new().expect("Failed to create clipboard!");
    if let Ok(image) = clipboard.get_image() {
        Ok(ClipboardValue::Image(ImageData {
            width: image.width,
            height: image.height,
            bytes: Vec::from(&*image.bytes),
        }))
    }else {
        Err("Not Image".to_string())
    }
}

#[tauri::command]
pub fn set_image(image: ImageData) {
    let mut clipboard = Clipboard::new().expect("Failed to create clipboard!");
    clipboard
        .set_image(ArImageData {
            width: image.width,
            height: image.height,
            bytes: Cow::Owned(image.bytes),
        })
        .unwrap();
}
