#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

extern crate tauri;

use tauri::{Builder, generate_context};
use crate::util::write_to_terminal_multicolor;

pub fn ignite_desktop() -> tauri::Result<()> {
    write_to_terminal_multicolor("Starting up desktop application ...");
    Builder::default()
        .run(generate_context!())
        // .expect("error while running tauri application")
}