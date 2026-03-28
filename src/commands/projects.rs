use crate::models::project::{Project, ProjectInput};
use crate::services::project_service::ProjectService;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn get_recent_projects(
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<Vec<Project>, String> {
    project_service
        .get_sorted_projects()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_project(
    input: ProjectInput,
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<Project, String> {
    project_service
        .add_project(input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pin_project(
    id: String,
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<(), String> {
    project_service
        .pin_project(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn unpin_project(
    id: String,
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<(), String> {
    project_service
        .unpin_project(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_project(
    id: String,
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<(), String> {
    project_service
        .remove_project(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_last_opened(
    id: String,
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<(), String> {
    project_service
        .update_last_opened(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn launch_claude_in_dir(
    path: String,
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<(), String> {
    project_service
        .launch_claude_in_directory(&path)
        .await
        .map_err(|e| e.to_string())
}
