use std::path::Path;
use std::fs;
use anyhow::{Result, Context};
use serde::{de::DeserializeOwned, Serialize};

/// File service for handling file operations
pub struct FileService;

impl FileService {
    /// Create a new FileService instance
    pub fn new() -> Self {
        Self
    }

    /// Read file contents as string
    pub fn read_file(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))
    }

    /// Write string content to file
    pub fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            self.ensure_dir(parent)?;
        }

        fs::write(path, content)
            .with_context(|| format!("Failed to write file: {}", path.display()))
    }

    /// Read and deserialize JSON file
    pub fn read_json<T: DeserializeOwned>(&self, path: &Path) -> Result<T> {
        let content = self.read_file(path)?;
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON from: {}", path.display()))
    }

    /// Serialize and write JSON file
    pub fn write_json<T: Serialize>(&self, path: &Path, data: &T) -> Result<()> {
        let content = serde_json::to_string_pretty(data)
            .context("Failed to serialize to JSON")?;
        self.write_file(path, &content)
    }

    /// Serialize and write JSON file with compact formatting
    pub fn write_json_compact<T: Serialize>(&self, path: &Path, data: &T) -> Result<()> {
        let content = serde_json::to_string(data)
            .context("Failed to serialize to JSON")?;
        self.write_file(path, &content)
    }

    /// Ensure directory exists, creating it if necessary
    pub fn ensure_dir(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)
                .with_context(|| format!("Failed to create directory: {}", path.display()))?;
        }
        Ok(())
    }

    /// Check if file exists
    pub fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    /// Delete a file
    pub fn delete_file(&self, path: &Path) -> Result<()> {
        fs::remove_file(path)
            .with_context(|| format!("Failed to delete file: {}", path.display()))
    }

    /// Delete a directory recursively
    pub fn delete_dir(&self, path: &Path) -> Result<()> {
        fs::remove_dir_all(path)
            .with_context(|| format!("Failed to delete directory: {}", path.display()))
    }
}

impl Default for FileService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::temp_dir;

    #[test]
    fn test_read_write_file() {
        let service = FileService::new();
        let test_dir = temp_dir().join("file_service_test");
        let test_file = test_dir.join("test.txt");

        // Clean up if exists
        let _ = service.delete_dir(&test_dir);

        // Write and read
        service.write_file(&test_file, "Hello, World!").unwrap();
        let content = service.read_file(&test_file).unwrap();
        assert_eq!(content, "Hello, World!");

        // Clean up
        service.delete_dir(&test_dir).unwrap();
    }

    #[test]
    fn test_json_operations() {
        let service = FileService::new();
        let test_dir = temp_dir().join("json_test");
        let test_file = test_dir.join("test.json");

        // Clean up if exists
        let _ = service.delete_dir(&test_dir);

        // Write and read JSON
        let data = serde_json::json!({
            "name": "test",
            "value": 42
        });
        service.write_json(&test_file, &data).unwrap();

        let read_data: serde_json::Value = service.read_json(&test_file).unwrap();
        assert_eq!(read_data["name"], "test");
        assert_eq!(read_data["value"], 42);

        // Clean up
        service.delete_dir(&test_dir).unwrap();
    }
}
