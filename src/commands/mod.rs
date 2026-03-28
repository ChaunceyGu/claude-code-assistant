use serde::{Deserialize, Serialize};
use tauri::command;

/// Simple greeting command for testing
#[command]
pub fn greet(name: String) -> String {
    format!("Hello, {}! Welcome to Claude Code Assistant.", name)
}

/// Get application version
#[command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// System information response
#[derive(Serialize, Debug)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub version: String,
}

/// Get system information
#[command]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        version: std::env::consts::FAMILY.to_string(),
    }
}
