use crate::watcher::{Handler, Master};
use arboard::{self, Clipboard as ArClipboard};
use serde::Serialize;
use std::{
    error::Error,
    sync::mpsc,
    thread::{self, JoinHandle},
};
use tauri::{App, Manager, Runtime};

#[derive(Debug, Clone, Serialize)]
pub struct ImageData {
   pub width: usize,
   pub height: usize,
   pub bytes: Vec<u8>,
}
#[derive(Debug, Clone, Serialize)]
pub enum ClipboardValue {
    Text(String),
    Image(ImageData),
}

struct Clipboard {
    core: ArClipboard,
    handler: Option<JoinHandle<()>>,
    rx: Option<mpsc::Receiver<bool>>,
}

impl Clipboard {
    pub fn new() -> Self {
        Clipboard {
            core: ArClipboard::new().unwrap(),
            handler: None,
            rx: None,
        }
    }

    pub fn get_text(&mut self) -> Result<String, arboard::Error> {
        self.core.get_text()
    }

    pub fn get_image(&mut self) -> Result<ImageData, arboard::Error> {
        let img = self.core.get_image();
        match img {
            Ok(image) => Ok(ImageData {
                width: image.width,
                height: image.height,
                bytes: Vec::from(&*image.bytes),
            }),
            Err(e) => Err(e),
        }
    }

    pub fn listen(&mut self) {
        if let Some(_) = self.handler {
            return;
        } else {
            let (tx, rx) = mpsc::channel();
            let handler = {
                thread::spawn(move || {
                    if let Err(e) = Master::new(Handler { tx }).run() {
                        panic!("=={:#}", e);
                    }
                })
            };
            self.handler = Some(handler);
            self.rx = Some(rx);
        }
    }
}

impl Drop for Clipboard {
    fn drop(&mut self) {
        if let Some(handler) = self.handler.take() {
            if let Err(e) = handler.join() {
                panic!("{:#?}", e);
            }
        }
    }
}

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
