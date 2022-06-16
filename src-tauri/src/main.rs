#![feature(drain_filter)]
#![feature(hash_drain_filter)]
#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::path::PathBuf;

use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::command;
use crate::error::ApplicationError;
use crate::file::LocationStatistic;

mod error;
mod file;
mod extension;
mod menu;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![analyse_location, open_dialog])
        .menu(menu::main_menu())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
async fn open_dialog() -> Option<PathBuf> {
    println!("open_dialog");
    FileDialogBuilder::new()
        .set_title("Open")
        .set_directory("/Users/james/")
        .pick_folder()
}

#[command]
async fn analyse_location(root: String) -> Result<LocationStatistic, ApplicationError> {
    println!("Analysing location: {}", root);
    file::generate_location_statistics(root)
}
