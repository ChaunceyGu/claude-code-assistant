#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod services;
mod utils;

use tauri::{generate_context, Manager};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            println!("Claude Code Assistant started successfully!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // System commands
            commands::greet,
            commands::get_app_version,
            commands::get_system_info,
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
