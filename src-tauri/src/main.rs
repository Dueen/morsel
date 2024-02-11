// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rdev::listen;
use tauri::Manager;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").expect("no main window");
            let _handle = std::thread::spawn(move || {
                let window = window.clone();
                let callback = move |event| {
                    if let Ok(message) = serde_json::to_string(&event) {
                        match window.emit("keyEvent", Payload { message }) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    }
                };

                if let Err(_) = listen(callback) {
                    println!("Failed to listen for events");
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
