use std::{sync::{Arc, Mutex}, time::Duration, process::Command};
use tauri::{AppHandle, Manager};

use elgato_streamdeck::{
    new_hidapi,
    list_devices,
    DeviceStateUpdate,
    StreamDeck
};

pub struct AudioState {
    muted: bool,
}

impl AudioState {
    pub fn new() -> Self {
        AudioState { muted: false }
    }
    
    pub fn toggle_mic(&mut self) {
        let new_mute = !self.muted;
        let mute_value = if new_mute { "1" } else { "0" };
        
        // Lista todas as sources (entradas/microfones) e muta cada uma
        let output = Command::new("pactl")
            .args(["list", "short", "sources"])
            .output();
        
        if let Ok(output) = output {
            let sources = String::from_utf8_lossy(&output.stdout);
            for line in sources.lines() {
                // Cada linha: "index\tname\t..."
                if let Some(index) = line.split('\t').next() {
                    // Muta a source pelo índice
                    let _ = Command::new("pactl")
                        .args(["set-source-mute", index, mute_value])
                        .output();
                }
            }
        }
        
        self.muted = new_mute;
        println!("Entradas de áudio {}", if self.muted { "MUTADAS" } else { "DESMUTADAS" });
    }
}

pub fn start_streamdeck_listener(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let hid = match new_hidapi() {
            Ok(hid) => hid,
            Err(error) => {
                log::error!("Failed to create HID API: {}", error);
                return;
            }
        };

        let devices = list_devices(&hid);
        if devices.is_empty() {
            log::error!("No StreamDeck devices found");
            return;
        }

        let (kind, serial) = devices[0].clone();

        let device = match StreamDeck::connect(&hid, kind, &serial) {
            Ok(device) => Arc::new(device),
            Err(error) => {
                log::error!("Failed to connect to StreamDeck: {}", error);
                return;
            }
        };

        println!("Connected to StreamDeck: {}", &serial);
        let reader = device.get_reader();

        loop {
            match reader.read(Some(Duration::from_millis(100))) {
                Ok(updates) => {
                    for update in updates {
                        if let DeviceStateUpdate::ButtonDown(key) = update {
                            if key == 0 {
                                if let Some(audio_state) = app_handle.try_state::<Mutex<AudioState>>() {
                                    if let Ok(mut audio) = audio_state.lock() {
                                        audio.toggle_mic();
                                    }
                                }
                            }
                        }
                    }
                },
                Err(error) => {
                    eprintln!("Error reading from StreamDeck: {:?}", error);
                }
            }
        }
    });
}