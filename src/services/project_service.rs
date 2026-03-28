use std::sync::Arc;
use anyhow::{Result, Context};
use chrono::Utc;
use uuid::Uuid;

use crate::models::project::{Project, ProjectInput};
use crate::services::file_service::FileService;
use crate::utils::paths::get_projects_file_path;
use crate::utils::validators::{is_valid_project_name, is_valid_directory_path};

/// Service for managing projects
pub struct ProjectService {
    file_service: Arc<FileService>,
}

impl ProjectService {
    /// Create a new ProjectService instance
    pub fn new(file_service: Arc<FileService>) -> Self {
        Self { file_service }
    }

    /// Load all projects
    /// Returns empty vector if no projects file exists
    pub fn load_projects(&self) -> Result<Vec<Project>> {
        let projects_path = get_projects_file_path()?;

        if self.file_service.exists(&projects_path) {
            self.file_service.read_json(&projects_path)
        } else {
            Ok(Vec::new())
        }
    }

    /// Save all projects
    pub fn save_projects(&self, projects: &[Project]) -> Result<()> {
        let projects_path = get_projects_file_path()?;
        self.file_service.write_json(&projects_path, projects)
    }

    /// Add a new project
    pub fn add_project(&self, input: ProjectInput) -> Result<Project> {
        // Validate project name
        if !is_valid_project_name(&input.name) {
            anyhow::bail!("Invalid project name: must be non-empty, <= 100 chars, and contain only alphanumeric, spaces, hyphens, underscores, or dots");
        }

        // Validate directory path
        if !is_valid_directory_path(&input.path)? {
            anyhow::bail!("Invalid project path: directory does not exist");
        }

        let mut projects = self.load_projects()?;

        // Check for duplicate name
        if projects.iter().any(|p| p.name == input.name) {
            anyhow::bail!("Project with name '{}' already exists", input.name);
        }

        // Create new project
        let project = Project {
            id: Uuid::new_v4().to_string(),
            name: input.name,
            path: input.path,
            description: input.description,
            tags: input.tags.unwrap_or_default(),
            pinned: false,
            last_opened: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        projects.push(project.clone());
        self.save_projects(&projects)?;

        Ok(project)
    }

    /// Remove a project by ID
    pub fn remove_project(&self, id: &str) -> Result<()> {
        let mut projects = self.load_projects()?;

        let initial_len = projects.len();
        projects.retain(|p| p.id != id);

        if projects.len() == initial_len {
            anyhow::bail!("Project with ID '{}' not found", id);
        }

        self.save_projects(&projects)?;
        Ok(())
    }

    /// Pin a project
    pub fn pin_project(&self, id: &str) -> Result<()> {
        let mut projects = self.load_projects()?;

        let project = projects
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or_else(|| anyhow::anyhow!("Project with ID '{}' not found", id))?;

        project.pinned = true;
        project.updated_at = Utc::now();

        self.save_projects(&projects)?;
        Ok(())
    }

    /// Unpin a project
    pub fn unpin_project(&self, id: &str) -> Result<()> {
        let mut projects = self.load_projects()?;

        let project = projects
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or_else(|| anyhow::anyhow!("Project with ID '{}' not found", id))?;

        project.pinned = false;
        project.updated_at = Utc::now();

        self.save_projects(&projects)?;
        Ok(())
    }

    /// Update the last opened timestamp for a project
    pub fn update_last_opened(&self, id: &str) -> Result<()> {
        let mut projects = self.load_projects()?;

        let project = projects
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or_else(|| anyhow::anyhow!("Project with ID '{}' not found", id))?;

        project.last_opened = Some(Utc::now());
        project.updated_at = Utc::now();

        self.save_projects(&projects)?;
        Ok(())
    }

    /// Get a project by ID
    pub fn get_project(&self, id: &str) -> Result<Option<Project>> {
        let projects = self.load_projects()?;
        Ok(projects.into_iter().find(|p| p.id == id))
    }

    /// Get projects sorted by pinned first, then by last opened
    pub fn get_sorted_projects(&self) -> Result<Vec<Project>> {
        let mut projects = self.load_projects()?;

        projects.sort_by(|a, b| {
            // First sort by pinned (pinned projects first)
            match (b.pinned, a.pinned) {
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                _ => {
                    // Then sort by last opened (most recent first)
                    match (&b.last_opened, &a.last_opened) {
                        (Some(b_time), Some(a_time)) => b_time.cmp(a_time),
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        (None, None) => a.created_at.cmp(&b.created_at),
                    }
                }
            }
        });

        Ok(projects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::temp_dir;

    #[test]
    fn test_project_service_creation() {
        let file_service = Arc::new(FileService::new());
        let _service = ProjectService::new(file_service);
    }

    #[test]
    fn test_is_valid_project_name() {
        assert!(is_valid_project_name("valid-project"));
        assert!(is_valid_project_name("Valid Project"));
        assert!(!is_valid_project_name(""));
        assert!(!is_valid_project_name("invalid/path"));
    }
}
