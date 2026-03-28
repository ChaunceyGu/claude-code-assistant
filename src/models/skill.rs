use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub version: String,
    #[serde(default)]
    pub author: Option<String>,
    pub source: SkillSource,
    #[serde(default)]
    pub status: SkillStatus,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub installed_at: DateTime<Local>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated_at: DateTime<Local>,
    #[serde(default)]
    pub config: SkillConfig,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Skill {
    pub fn new(
        name: impl Into<String>,
        display_name: impl Into<String>,
        version: impl Into<String>,
        source: SkillSource,
    ) -> Self {
        let name_str = name.into();
        let id = format!("skill_{}", uuid::Uuid::new_v4().simple());
        let now = Local::now();

        Self {
            id,
            name: name_str.clone(),
            display_name: display_name.into(),
            description: None,
            version: version.into(),
            author: None,
            source,
            status: SkillStatus::Active,
            installed_at: now,
            updated_at: now,
            config: SkillConfig::default(),
            icon: None,
            tags: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_config(mut self, config: SkillConfig) -> Self {
        self.config = config;
        self
    }

    pub fn is_updatable(&self) -> bool {
        matches!(
            self.source,
            SkillSource::Marketplace | SkillSource::Local | SkillSource::Custom
        ) && matches!(self.status, SkillStatus::Active | SkillStatus::Disabled)
    }

    pub fn is_configurable(&self) -> bool {
        !self.config.schema.is_null()
    }

    pub fn update_status(&mut self, status: SkillStatus) {
        self.status = status;
        self.updated_at = Local::now();
    }

    pub fn update_version(&mut self, version: impl Into<String>) {
        self.version = version.into();
        self.updated_at = Local::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SkillSource {
    BuiltIn,
    Marketplace,
    Custom,
    Local,
}

impl Default for SkillSource {
    fn default() -> Self {
        Self::Custom
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SkillStatus {
    Active,
    Disabled,
    Error,
    Updating,
}

impl Default for SkillStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConfig {
    #[serde(default)]
    pub schema: serde_json::Value,
    #[serde(default)]
    pub values: serde_json::Value,
}

impl Default for SkillConfig {
    fn default() -> Self {
        Self {
            schema: serde_json::Value::Null,
            values: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillUpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub release_notes: Option<String>,
    pub download_url: Option<String>,
    #[serde(default)]
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInput {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub version: String,
    pub author: Option<String>,
    pub source: SkillSource,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub config: SkillConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillList {
    pub skills: Vec<Skill>,
    pub total: usize,
    pub active: Vec<Skill>,
    pub disabled: Vec<Skill>,
    pub builtin: Vec<Skill>,
}

impl SkillList {
    pub fn new(skills: Vec<Skill>) -> Self {
        let total = skills.len();
        let active: Vec<Skill> = skills
            .iter()
            .filter(|s| s.status == SkillStatus::Active)
            .cloned()
            .collect();
        let disabled: Vec<Skill> = skills
            .iter()
            .filter(|s| s.status == SkillStatus::Disabled)
            .cloned()
            .collect();
        let builtin: Vec<Skill> = skills
            .iter()
            .filter(|s| s.source == SkillSource::BuiltIn)
            .cloned()
            .collect();

        Self {
            skills,
            total,
            active,
            disabled,
            builtin,
        }
    }
}
