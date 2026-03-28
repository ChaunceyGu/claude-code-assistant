use crate::models::skill::{Skill, SkillInput, SkillUpdateInfo};

#[tauri::command]
pub async fn get_installed_skills() -> Result<Vec<Skill>, String> {
    // TODO: Implement skill service
    Ok(vec![])
}

#[tauri::command]
pub async fn get_available_skills() -> Result<Vec<Skill>, String> {
    // TODO: Implement skill marketplace
    Ok(vec![])
}

#[tauri::command]
pub async fn install_skill(name: String) -> Result<(), String> {
    // TODO: Implement skill installation
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn install_skill_from_url(url: String) -> Result<(), String> {
    // TODO: Implement custom skill installation
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn update_skill(name: String) -> Result<(), String> {
    // TODO: Implement skill update
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn uninstall_skill(name: String) -> Result<(), String> {
    // TODO: Implement skill uninstallation
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn toggle_skill(name: String, enabled: bool) -> Result<(), String> {
    // TODO: Implement skill toggle
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn get_skill_config(name: String) -> Result<serde_json::Value, String> {
    // TODO: Implement skill config retrieval
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn update_skill_config(name: String, config: serde_json::Value) -> Result<(), String> {
    // TODO: Implement skill config update
    Err("Not implemented".to_string())
}

#[tauri::command]
pub async fn check_skill_updates() -> Result<Vec<SkillUpdateInfo>, String> {
    // TODO: Implement skill update checking
    Ok(vec![])
}
