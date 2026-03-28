use std::sync::Arc;
use anyhow::{Result, Context};

use crate::models::config::{AppConfig, ClaudeConfig, default_app_config};
use crate::services::file_service::FileService;
use crate::utils::paths::{get_config_file_path, get_claude_config_path};

/// Service for managing application and Claude Code configuration
pub struct ConfigService {
    file_service: Arc<FileService>,
}

impl ConfigService {
    /// Create a new ConfigService instance
    pub fn new(file_service: Arc<FileService>) -> Self {
        Self { file_service }
    }

    /// Load the application configuration
    /// Creates default config if it doesn't exist
    pub fn load_app_config(&self) -> Result<AppConfig> {
        let config_path = get_config_file_path()?;

        if self.file_service.exists(&config_path) {
            self.file_service.read_json(&config_path)
        } else {
            // Create default config
            let config = default_app_config();
            self.save_app_config(&config)?;
            Ok(config)
        }
    }

    /// Save the application configuration
    pub fn save_app_config(&self, config: &AppConfig) -> Result<()> {
        let config_path = get_config_file_path()?;
        self.file_service.write_json(&config_path, config)
    }

    /// Load the Claude Code configuration from ~/.claude/settings.json
    pub fn load_claude_config(&self) -> Result<ClaudeConfig> {
        let config_path = get_claude_config_path()?;

        if self.file_service.exists(&config_path) {
            self.file_service.read_json(&config_path)
        } else {
            // Return empty/default config
            Ok(ClaudeConfig::default())
        }
    }

    /// Save the Claude Code configuration to ~/.claude/settings.json
    pub fn save_claude_config(&self, config: &ClaudeConfig) -> Result<()> {
        let config_path = get_claude_config_path()?;

        // Ensure the .claude directory exists
        if let Some(parent) = config_path.parent() {
            self.file_service.ensure_dir(parent)?;
        }

        self.file_service.write_json(&config_path, config)
    }

    /// Ensure default configuration exists
    /// Creates default config files if they don't exist
    pub fn ensure_default_config(&self) -> Result<()> {
        // Ensure app data directory exists
        let app_data_dir = crate::utils::paths::get_app_data_dir()?;
        self.file_service.ensure_dir(&app_data_dir)?;

        // Create default app config if needed
        let config_path = get_config_file_path()?;
        if !self.file_service.exists(&config_path) {
            let config = default_app_config();
            self.save_app_config(&config)?;
        }

        Ok(())
    }

    /// Reset app configuration to defaults
    pub fn reset_app_config(&self) -> Result<AppConfig> {
        let config = default_app_config();
        self.save_app_config(&config)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::temp_dir;

    // Note: These tests would need to be adapted to use a mock FileService
    // for proper unit testing without affecting real config files

    #[test]
    fn test_config_service_creation() {
        let file_service = Arc::new(FileService::new());
        let _service = ConfigService::new(file_service);
    }
}
