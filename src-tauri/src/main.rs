#![feature(drain_filter)]
#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::command;

use crate::error::ApplicationError;
use crate::location::{Location, LocationDetails};

mod location;
mod error;
mod file_details;
mod extension;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_location, analyse_location])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
fn add_location(root: String) -> Result<Location, ApplicationError> {
    location::add_location(root)
}

#[command]
async fn analyse_location(location: Location) -> Result<LocationDetails, ApplicationError> {
    location::analyse_location(location)
}
