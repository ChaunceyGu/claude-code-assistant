use anyhow::Result;
use std::path::PathBuf;

/// File system service for common operations
pub struct FileService;

impl FileService {
    /// Get the application data directory
    pub fn get_app_dir() -> Result<PathBuf> {
        let app_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("claude-code-assistant");

        if !app_dir.exists() {
            std::fs::create_dir_all(&app_dir)?;
        }

        Ok(app_dir)
    }

    /// Get the logs directory
    pub fn get_logs_dir() -> Result<PathBuf> {
        let logs_dir = Self::get_app_dir()?.join("logs");
        if !logs_dir.exists() {
            std::fs::create_dir_all(&logs_dir)?;
        }
        Ok(logs_dir)
    }
}

/// Configuration service
pub struct ConfigService;

impl ConfigService {
    /// Get config file path
    pub fn get_config_path() -> Result<PathBuf> {
        Ok(FileService::get_app_dir()?.join("config.json"))
    }
}
