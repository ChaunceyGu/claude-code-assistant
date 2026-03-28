#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;
use tauri::Manager;

mod commands;
mod models;
mod services;
mod utils;

use services::config_service::ConfigService;
use services::file_service::FileService;
use services::project_service::ProjectService;
use services::claude_service::ClaudeService;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize services
            let file_service = Arc::new(FileService::new());
            let config_service = Arc::new(ConfigService::new(file_service.clone()));
            let project_service = Arc::new(ProjectService::new(file_service.clone()));
            let claude_service = Arc::new(ClaudeService::new());

            // Manage services
            app.manage(file_service);
            app.manage(config_service);
            app.manage(project_service);
            app.manage(claude_service);

            // Open devtools in debug mode
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Config commands
            commands::config::get_app_config,
            commands::config::save_app_config,
            commands::config::get_claude_config,
            commands::config::save_claude_config,

            // Project commands
            commands::projects::get_recent_projects,
            commands::projects::add_project,
            commands::projects::pin_project,
            commands::projects::unpin_project,
            commands::projects::remove_project,
            commands::projects::update_last_opened,
            commands::projects::launch_claude_in_dir,

            // Profile commands
            commands::profiles::get_profiles,
            commands::profiles::create_profile,
            commands::profiles::update_profile,
            commands::profiles::apply_profile,
            commands::profiles::delete_profile,
            commands::profiles::export_profile,
            commands::profiles::import_profile,

            // Skill commands
            commands::skills::get_installed_skills,
            commands::skills::get_available_skills,
            commands::skills::install_skill,
            commands::skills::install_skill_from_url,
            commands::skills::update_skill,
            commands::skills::uninstall_skill,
            commands::skills::toggle_skill,
            commands::skills::get_skill_config,
            commands::skills::update_skill_config,
            commands::skills::check_skill_updates,

            // Permission commands
            commands::permissions::get_permissions,
            commands::permissions::update_permission,
            commands::permissions::update_permission_group,

            // Quick commands
            commands::quick_commands::get_quick_commands,
            commands::quick_commands::execute_quick_command,
            commands::quick_commands::add_custom_command,
            commands::quick_commands::remove_custom_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
