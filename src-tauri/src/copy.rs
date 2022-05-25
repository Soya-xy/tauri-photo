use crate::clip::{ClipboardValue, ImageData};
use arboard::{Clipboard, ImageData as ArImageData};
use std::borrow::Cow;

#[tauri::command]
pub fn get_image() -> ClipboardValue {
    let mut clipboard = Clipboard::new().expect("Failed to create clipboard!");
    let image = clipboard.get_image().unwrap();
    println!("123");
    ClipboardValue::Image(ImageData {
        width: image.width,
        height: image.height,
        bytes: Vec::from(&*image.bytes),
    })
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
