use crate::models::config::{AppConfig, ClaudeConfig};
use crate::services::config_service::ConfigService;
use crate::utils::paths::get_config_file_path;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_app_config(
    config_service: State<'_, Arc<ConfigService>>,
) -> Result<AppConfig, String> {
    config_service
        .load_app_config()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_app_config(
    config: AppConfig,
    config_service: State<'_, Arc<ConfigService>>,
) -> Result<(), String> {
    config_service
        .save_app_config(&config)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_claude_config(
    config_service: State<'_, Arc<ConfigService>>,
) -> Result<ClaudeConfig, String> {
    config_service
        .load_claude_config()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_claude_config(
    config: ClaudeConfig,
    config_service: State<'_, Arc<ConfigService>>,
) -> Result<(), String> {
    config_service
        .save_claude_config(&config)
        .await
        .map_err(|e| e.to_string())
}
