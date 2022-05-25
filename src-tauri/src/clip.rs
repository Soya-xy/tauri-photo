use arboard::{self, Clipboard as ArClipboard};
use crate::watcher::{Handler, Master};
use serde::{Serialize, Deserialize};
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};
pub struct Clipboard {
  core: ArClipboard,
  handler: Option<JoinHandle<()>>,
  pub rx: Option<mpsc::Receiver<bool>>,
}

#[derive(Debug, Clone, Serialize,Deserialize)]
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
