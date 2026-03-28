use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub last_opened: DateTime<Local>,
    #[serde(default)]
    pub is_pinned: bool,
    #[serde(default)]
    pub color: Option<String>,
}

impl Project {
    pub fn new(name: impl Into<String>, path: impl Into<String>) -> Self {
        let name_str = name.into();
        let path_str = path.into();
        let id = format!("proj_{}", uuid::Uuid::new_v4().simple());

        Self {
            id,
            name: name_str,
            path: path_str,
            description: None,
            last_opened: Local::now(),
            is_pinned: false,
            color: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn pin(mut self) -> Self {
        self.is_pinned = true;
        self
    }

    pub fn update_last_opened(&mut self) {
        self.last_opened = Local::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInput {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub color: Option<String>,
    #[serde(default)]
    pub is_pinned: bool,
}

impl ProjectInput {
    pub fn new(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            description: None,
            color: None,
            is_pinned: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUpdateInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub is_pinned: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectList {
    pub projects: Vec<Project>,
    pub total: usize,
    pub pinned: Vec<Project>,
    pub recent: Vec<Project>,
}

impl ProjectList {
    pub fn new(projects: Vec<Project>) -> Self {
        let total = projects.len();
        let pinned: Vec<Project> = projects
            .iter()
            .filter(|p| p.is_pinned)
            .cloned()
            .collect();

        let mut recent = projects.clone();
        recent.sort_by(|a, b| b.last_opened.cmp(&a.last_opened));
        recent.truncate(10);

        Self {
            projects,
            total,
            pinned,
            recent,
        }
    }
}
