#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::command;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
fn greet(name: String, age: u8) -> String {
    format!("Hello {}, {} year-old human!", name, age)
}

#[command]
fn my_custom_command() -> Result<String, String> {
    // If something fails
    Err("This failed!".into())
    // If it worked
    // Ok("This worked!".into())
}
