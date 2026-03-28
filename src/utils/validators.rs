use std::path::Path;
use anyhow::{Result, Context};

const MAX_PROJECT_NAME_LENGTH: usize = 100;
const MAX_COMMAND_NAME_LENGTH: usize = 50;
const MAX_PROFILE_NAME_LENGTH: usize = 50;

/// Validates a project name
/// - Must not be empty
/// - Must be <= 100 characters
/// - Must not contain special characters that could be problematic in file paths
pub fn is_valid_project_name(name: &str) -> bool {
    if name.is_empty() || name.len() > MAX_PROJECT_NAME_LENGTH {
        return false;
    }

    // Check for invalid characters
    // Allow: alphanumeric, spaces, hyphens, underscores, dots
    // Disallow: path separators, control characters, etc.
    name.chars().all(|c| {
        c.is_alphanumeric()
            || c.is_whitespace()
            || c == '-'
            || c == '_'
            || c == '.'
    })
}

/// Validates a command name
/// - Must not be empty
/// - Must be <= 50 characters
pub fn is_valid_command_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= MAX_COMMAND_NAME_LENGTH
}

/// Validates a profile name
/// - Must only contain alphanumeric characters, hyphens, and underscores
/// - Must not be empty
/// - Must be <= 50 characters
pub fn is_valid_profile_name(name: &str) -> bool {
    if name.is_empty() || name.len() > MAX_PROFILE_NAME_LENGTH {
        return false;
    }

    name.chars().all(|c| {
        c.is_alphanumeric() || c == '-' || c == '_'
    })
}

/// Validates that a directory path exists and is a directory
pub fn is_valid_directory_path(path: &str) -> Result<bool> {
    let path = Path::new(path);

    if !path.exists() {
        return Ok(false);
    }

    let metadata = path.metadata()
        .context("Failed to read path metadata")?;

    Ok(metadata.is_dir())
}

/// Validates that a file path exists and is a file
pub fn is_valid_file_path(path: &str) -> Result<bool> {
    let path = Path::new(path);

    if !path.exists() {
        return Ok(false);
    }

    let metadata = path.metadata()
        .context("Failed to read path metadata")?;

    Ok(metadata.is_file())
}

/// Sanitizes a filename by replacing invalid characters with underscores
/// Invalid characters on most systems: < > : " / \ | ? *
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| {
            match c {
                '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
                c if c.is_control() => '_',
                c => c,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_project_name() {
        // Valid names
        assert!(is_valid_project_name("my-project"));
        assert!(is_valid_project_name("My Project"));
        assert!(is_valid_project_name("project_123"));
        assert!(is_valid_project_name("a".repeat(MAX_PROJECT_NAME_LENGTH).as_str()));

        // Invalid names
        assert!(!is_valid_project_name(""));
        assert!(!is_valid_project_name("project/name"));
        assert!(!is_valid_project_name("project<name>"));
        assert!(!is_valid_project_name("a".repeat(MAX_PROJECT_NAME_LENGTH + 1).as_str()));
    }

    #[test]
    fn test_is_valid_command_name() {
        assert!(is_valid_command_name("build"));
        assert!(is_valid_command_name("run-tests"));
        assert!(!is_valid_command_name(""));
        assert!(!is_valid_command_name("a".repeat(MAX_COMMAND_NAME_LENGTH + 1).as_str()));
    }

    #[test]
    fn test_is_valid_profile_name() {
        assert!(is_valid_profile_name("default"));
        assert!(is_valid_profile_name("my-profile"));
        assert!(is_valid_profile_name("profile_123"));
        assert!(!is_valid_profile_name(""));
        assert!(!is_valid_profile_name("profile name"));
        assert!(!is_valid_profile_name("profile/name"));
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("file:name"), "file_name");
        assert_eq!(sanitize_filename("file/name"), "file_name");
        assert_eq!(sanitize_filename("file<name>"), "file_name");
        assert_eq!(sanitize_filename("file|name?"), "file_name_");
        assert_eq!(sanitize_filename("valid_name.txt"), "valid_name.txt");
    }
}
