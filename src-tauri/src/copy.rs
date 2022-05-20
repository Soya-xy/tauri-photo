use arboard::Clipboard;

struct ClipboardContent {
    width: usize,
    height: usize,
    pub image: Vec<u8>,
}

// impl ClipboardContent {
//     fn show(&self) -> ClipboardContent {
//         ClipboardContent {
//             width: self.width,
//             height: self.height,
//             image: self.image,
//         }
//     }
// }

#[tauri::command]
pub fn hello() -> ClipboardContent {
    let mut clipboard = Clipboard::new().expect("Failed to create clipboard!");
    let image = clipboard.get_image().unwrap();
    ClipboardContent {
        width: image.width,
        height: image.height,
        image: Vec::from(&*image.bytes),
    }
}
