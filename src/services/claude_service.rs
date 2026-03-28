use std::process::{Command, Stdio};
use std::path::Path;
use anyhow::{Result, Context};

const CLAUDE_COMMAND: &str = "claude";

/// Service for interacting with Claude Code CLI
pub struct ClaudeService;

impl ClaudeService {
    /// Create a new ClaudeService instance
    pub fn new() -> Self {
        Self
    }

    /// Check if Claude Code is installed and available in PATH
    pub fn check_claude_installed(&self) -> Result<bool> {
        match Command::new(CLAUDE_COMMAND)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
        {
            Ok(status) => Ok(status.success()),
            Err(_) => Ok(false),
        }
    }

    /// Get the Claude Code version
    pub fn get_claude_version(&self) -> Result<String> {
        let output = Command::new(CLAUDE_COMMAND)
            .arg("--version")
            .output()
            .context("Failed to execute 'claude --version'. Is Claude Code installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to get Claude version: {}", stderr);
        }

        let version = String::from_utf8_lossy(&output.stdout);
        Ok(version.trim().to_string())
    }

    /// Launch Claude Code in the specified directory
    /// Note: This spawns the process but doesn't wait for it
    pub fn launch_in_directory(&self, path: &str) -> Result<()> {
        let path_obj = Path::new(path);

        if !path_obj.exists() {
            anyhow::bail!("Directory does not exist: {}", path);
        }

        if !path_obj.is_dir() {
            anyhow::bail!("Path is not a directory: {}", path);
        }

        // Launch Claude Code in the specified directory
        // Using 'spawn' to avoid blocking
        let _child = Command::new(CLAUDE_COMMAND)
            .current_dir(path_obj)
            .spawn()
            .context("Failed to launch Claude Code. Is it installed?")?;

        Ok(())
    }

    /// Execute a Claude Code command and return the output
    ///
    /// # Arguments
    /// * `command` - The command to execute (e.g., "claude --version")
    /// * `path` - Optional working directory for the command
    pub fn execute_command(&self, command: &str, path: Option<&str>) -> Result<String> {
        let args: Vec<&str> = command.split_whitespace().collect();

        if args.is_empty() {
            anyhow::bail!("Empty command");
        }

        let mut cmd = Command::new(CLAUDE_COMMAND);

        // Add all arguments
        for arg in &args[1..] {
            cmd.arg(arg);
        }

        // Set working directory if provided
        if let Some(p) = path {
            cmd.current_dir(p);
        }

        // Execute and capture output
        let output = cmd.output()
            .context("Failed to execute Claude command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Command failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim().to_string())
    }

    /// Check if Claude Code is available and get its version
    /// Returns None if Claude is not installed
    pub fn check_claude_status(&self) -> Option<String> {
        match self.get_claude_version() {
            Ok(version) => Some(version),
            Err(_) => None,
        }
    }
}

impl Default for ClaudeService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_service_creation() {
        let _service = ClaudeService::new();
    }

    // Note: Tests that actually run Claude commands would require
    // Claude Code to be installed, so they're skipped in unit tests
}
