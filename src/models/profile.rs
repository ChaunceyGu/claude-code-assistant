use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Local>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated_at: DateTime<Local>,
    pub config: ProfileConfig,
}

impl Profile {
    pub fn new(name: impl Into<String>) -> Self {
        let name_str = name.into();
        let id = format!("profile_{}", uuid::Uuid::new_v4().simple());
        let now = Local::now();

        Self {
            id,
            name: name_str,
            description: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            config: ProfileConfig::default(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_config(mut self, config: ProfileConfig) -> Self {
        self.config = config;
        self
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = Local::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    #[serde(default)]
    pub permissions: PermissionConfig,
    #[serde(default)]
    pub hooks: HookConfig,
    #[serde(default)]
    pub settings: serde_json::Value,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            permissions: PermissionConfig::default(),
            hooks: HookConfig::default(),
            settings: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionConfig {
    #[serde(default)]
    pub allowed_commands: Vec<String>,
    #[serde(default)]
    pub blocked_commands: Vec<String>,
    #[serde(default)]
    pub require_confirmation: Vec<String>,
    #[serde(default)]
    pub allow_dangerous_operations: bool,
    #[serde(default)]
    pub allow_network_access: bool,
    #[serde(default)]
    pub allow_file_system_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HookConfig {
    #[serde(default)]
    pub pre_command: Option<String>,
    #[serde(default)]
    pub post_command: Option<String>,
    #[serde(default)]
    pub on_error: Option<String>,
    #[serde(default)]
    pub on_project_open: Option<String>,
    #[serde(default)]
    pub on_project_close: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileInput {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub config: Option<ProfileConfig>,
}

impl ProfileInput {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            tags: Vec::new(),
            config: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUpdateInput {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub config: Option<ProfileConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileList {
    pub profiles: Vec<Profile>,
    pub total: usize,
    pub active_profile: Option<String>,
}

impl ProfileList {
    pub fn new(profiles: Vec<Profile>) -> Self {
        let total = profiles.len();
        Self {
            profiles,
            total,
            active_profile: None,
        }
    }

    pub fn with_active_profile(mut self, active_id: impl Into<String>) -> Self {
        self.active_profile = Some(active_id.into());
        self
    }
}
