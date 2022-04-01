#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use serde::Serialize;
use std::path::PathBuf;
use tauri::command;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_image_root])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize)]
struct Location {
    root: PathBuf,
    other: String,
    count: usize,
}

#[command]
fn add_image_root(root: String) -> Location {
    Location {
        root: PathBuf::from(root),
        other: "other".to_string(),
        count: 0,
    }
}
