use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

/// Application name for directory naming
const APP_NAME: &str = "claude-code-assistant";
const CLAUDE_CONFIG_DIR: &str = ".claude";
const CLAUDE_SETTINGS_FILE: &str = "settings.json";

/// Get the application data directory
/// On Linux: ~/.local/share/claude-code-assistant/
/// On Windows: %APPDATA%/claude-code-assistant/
/// On macOS: ~/Library/Application Support/claude-code-assistant/
pub fn get_app_data_dir() -> Result<PathBuf> {
    let data_dir = dirs::data_dir()
        .context("Failed to get data directory")?;
    let app_dir = data_dir.join(APP_NAME);
    Ok(app_dir)
}

/// Get the configuration file path
pub fn get_config_file_path() -> Result<PathBuf> {
    let app_dir = get_app_data_dir()?;
    Ok(app_dir.join("config.json"))
}

/// Get the projects file path
pub fn get_projects_file_path() -> Result<PathBuf> {
    let app_dir = get_app_data_dir()?;
    Ok(app_dir.join("projects.json"))
}

/// Get the profiles directory path
pub fn get_profiles_dir() -> Result<PathBuf> {
    let app_dir = get_app_data_dir()?;
    let profiles_dir = app_dir.join("profiles");
    Ok(profiles_dir)
}

/// Get the Claude Code configuration path (~/.claude/settings.json)
pub fn get_claude_config_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir()
        .context("Failed to get home directory")?;
    let claude_config = home_dir
        .join(CLAUDE_CONFIG_DIR)
        .join(CLAUDE_SETTINGS_FILE);
    Ok(claude_config)
}

/// Expand ~ in a path to the user's home directory
pub fn expand_tilde(path: &str) -> Result<PathBuf> {
    if path.starts_with("~/") || path == "~" {
        let home_dir = dirs::home_dir()
            .context("Failed to get home directory")?;
        let without_tilde = if path == "~" {
            ""
        } else {
            &path[2..]
        };
        Ok(home_dir.join(without_tilde))
    } else {
        Ok(PathBuf::from(path))
    }
}

/// Normalize a path: expand ~ and convert to absolute path
pub fn normalize_path(path: &str) -> Result<PathBuf> {
    let expanded = expand_tilde(path)?;
    let absolute = if expanded.is_absolute() {
        expanded
    } else {
        std::env::current_dir()
            .context("Failed to get current directory")?
            .join(expanded)
    };
    Ok(absolute)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let home = dirs::home_dir().unwrap();

        // Test ~/path
        let result = expand_tilde("~/test/path").unwrap();
        assert_eq!(result, home.join("test/path"));

        // Test just ~
        let result = expand_tilde("~").unwrap();
        assert_eq!(result, home);

        // Test regular path
        let result = expand_tilde("/usr/local").unwrap();
        assert_eq!(result, PathBuf::from("/usr/local"));
    }
}
