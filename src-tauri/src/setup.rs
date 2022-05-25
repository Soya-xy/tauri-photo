use crate::clip::{Clipboard, ClipboardValue};
use std::error::Error;
use tauri::{App, Manager, Runtime};

pub fn watcher_clip<R>(app: &mut App<R>) -> Result<(), Box<dyn Error>>
where
    R: Runtime,
{
    let main_window = app.get_window("main").unwrap();
    std::thread::spawn(move || {
        let mut clipboard = Clipboard::new();
        clipboard.listen();
        if let Some(rx) = clipboard.rx.take() {
            for _exists_msg in rx {
                if let Ok(text) = clipboard.get_text() {
                    println!("text: {:?}", text);
                    main_window
                        .emit("ClipboardEvent", ClipboardValue::Text(text))
                        .unwrap()
                } else if let Ok(image) = clipboard.get_image() {
                    println!("image");
                    main_window
                        .emit("ClipboardEvent", ClipboardValue::Image(image))
                        .unwrap()
                }
            }
        }
    });

    Ok(())
}
