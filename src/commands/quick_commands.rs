use crate::models::quick_command::{QuickCommand, QuickCommandInput};

#[tauri::command]
pub async fn get_quick_commands() -> Result<Vec<QuickCommand>, String> {
    // TODO: Implement quick commands service
    Ok(vec![])
}

#[tauri::command]
pub async fn execute_quick_command(
    id: String,
    params: Option<std::collections::HashMap<String, String>>,
) -> Result<String, String> {
    // TODO: Implement command execution
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn add_custom_command(input: QuickCommandInput) -> Result<QuickCommand, String> {
    // TODO: Implement custom command addition
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn remove_custom_command(id: String) -> Result<(), String> {
    // TODO: Implement custom command removal
    Err("Not implemented".to_string())
}
