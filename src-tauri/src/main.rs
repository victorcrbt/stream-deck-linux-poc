// Prevents additional console window on Windows in release, DO NOT REMOVE!!
mod streamdeck;

use std::sync::Mutex;
use tauri::Manager;
use streamdeck::{start_streamdeck_listener, AudioState};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      app.manage(Mutex::new(AudioState::new()));
      start_streamdeck_listener(app.handle().clone());
      AudioState::init();
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
