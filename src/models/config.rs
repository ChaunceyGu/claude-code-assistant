use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub general: GeneralSettings,
    #[serde(default)]
    pub claude_code: ClaudeCodeSettings,
    #[serde(default)]
    pub features: FeatureSettings,
    #[serde(default)]
    pub recent_projects: RecentProjectSettings,
    #[serde(default)]
    pub quick_commands: QuickCommandSettings,
    #[serde(default)]
    pub config_profiles: ConfigProfileSettings,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            claude_code: ClaudeCodeSettings::default(),
            features: FeatureSettings::default(),
            recent_projects: RecentProjectSettings::default(),
            quick_commands: QuickCommandSettings::default(),
            config_profiles: ConfigProfileSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default)]
    pub language: String,
    #[serde(default = "default_font_size")]
    pub font_size: u32,
    #[serde(default)]
    pub auto_save: bool,
    #[serde(default = "default_auto_save_interval")]
    pub auto_save_interval: u32,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            language: String::new(),
            font_size: default_font_size(),
            auto_save: false,
            auto_save_interval: default_auto_save_interval(),
        }
    }
}

fn default_theme() -> String {
    "system".to_string()
}

fn default_font_size() -> u32 {
    14
}

fn default_auto_save_interval() -> u32 {
    300
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodeSettings {
    #[serde(default)]
    pub default_model: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub timeout: u64,
    #[serde(default)]
    pub max_tokens: u32,
    #[serde(default)]
    pub temperature: f32,
}

impl Default for ClaudeCodeSettings {
    fn default() -> Self {
        Self {
            default_model: String::new(),
            api_key: None,
            base_url: None,
            timeout: 60,
            max_tokens: 4096,
            temperature: 0.7,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSettings {
    #[serde(default = "default_true")]
    pub enable_git_integration: bool,
    #[serde(default = "default_true")]
    pub enable_auto_complete: bool,
    #[serde(default = "default_true")]
    pub enable_syntax_highlighting: bool,
    #[serde(default)]
    pub enable_telemetry: bool,
    #[serde(default)]
    pub experimental_features: Vec<String>,
}

impl Default for FeatureSettings {
    fn default() -> Self {
        Self {
            enable_git_integration: true,
            enable_auto_complete: true,
            enable_syntax_highlighting: true,
            enable_telemetry: false,
            experimental_features: Vec::new(),
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentProjectSettings {
    #[serde(default)]
    pub projects: Vec<String>,
    #[serde(default = "default_max_recent")]
    pub max_recent: usize,
}

impl Default for RecentProjectSettings {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            max_recent: default_max_recent(),
        }
    }
}

fn default_max_recent() -> usize {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickCommandSettings {
    #[serde(default)]
    pub commands: Vec<CustomQuickCommand>,
    #[serde(default = "default_true")]
    pub show_in_toolbar: bool,
    #[serde(default)]
    pub custom_shortcuts: HashMap<String, String>,
}

impl Default for QuickCommandSettings {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
            show_in_toolbar: true,
            custom_shortcuts: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomQuickCommand {
    pub id: String,
    pub name: String,
    pub description: String,
    pub command: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub shortcut: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProfileSettings {
    #[serde(default)]
    pub profiles: Vec<String>,
    #[serde(default)]
    pub active_profile: Option<String>,
}

impl Default for ConfigProfileSettings {
    fn default() -> Self {
        Self {
            profiles: Vec::new(),
            active_profile: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfig {
    #[serde(default)]
    pub default_model: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub organization_id: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub custom_headers: HashMap<String, String>,
}

impl Default for ClaudeConfig {
    fn default() -> Self {
        Self {
            default_model: String::new(),
            api_key: None,
            base_url: None,
            organization_id: None,
            timeout: default_timeout(),
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
            top_p: None,
            custom_headers: HashMap::new(),
        }
    }
}

fn default_timeout() -> u64 {
    60
}

fn default_max_tokens() -> u32 {
    4096
}

fn default_temperature() -> f32 {
    0.7
}
