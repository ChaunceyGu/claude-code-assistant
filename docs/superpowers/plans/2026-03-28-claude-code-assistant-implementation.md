# Claude Code 桌面辅助工具实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 构建一个跨平台的 Claude Code 桌面辅助工具，提供可视化配置管理、快捷操作和项目导航功能。

**Architecture:** Tauri 框架 (Rust + Web)，前端使用原生 HTML/CSS/JS，单窗口仪表盘布局，6 个功能卡片模块，本地 JSON 文件存储配置。

**Tech Stack:** Tauri (Rust), HTML5, CSS3, JavaScript (ES6+), 本地文件系统存储

---

## 文件结构规划

```
claude-code-assistant/
├── src/                          # Rust 后端代码
│   ├── main.rs                   # 程序入口
│   ├── lib.rs                    # 库模块导出
│   ├── commands/                 # Tauri 命令处理器
│   │   ├── mod.rs
│   │   ├── config.rs             # 配置相关命令
│   │   ├── projects.rs           # 项目管理命令
│   │   ├── profiles.rs           # 配置方案命令
│   │   ├── skills.rs             # Skill 管理命令
│   │   ├── permissions.rs        # 权限管理命令
│   │   └── quick_commands.rs   # 快捷命令
│   ├── models/                   # 数据模型
│   │   ├── mod.rs
│   │   ├── config.rs
│   │   ├── project.rs
│   │   ├── profile.rs
│   │   ├── skill.rs
│   │   ├── permission.rs
│   │   └── quick_command.rs
│   ├── services/                 # 业务逻辑服务
│   │   ├── mod.rs
│   │   ├── config_service.rs
│   │   ├── project_service.rs
│   │   ├── claude_service.rs
│   │   └── file_service.rs
│   └── utils/                    # 工具函数
│       ├── mod.rs
│       ├── paths.rs
│       └── validators.rs
├── src-ui/                       # 前端代码
│   ├── index.html                # 主页面
│   ├── main.js                   # 入口脚本
│   ├── styles.css                # 全局样式
│   └── components/               # UI 组件
│       ├── Dashboard.js          # 仪表盘主组件
│       ├── Card.js               # 卡片组件
│       ├── PermissionManager.js  # 权限管理组件
│       ├── ProjectManager.js     # 项目管理组件
│       ├── QuickCommands.js      # 快捷命令组件
│       ├── ConfigEditor.js       # 配置编辑器组件
│       ├── ProfileManager.js     # 配置方案管理组件
│       └── SkillManager.js       # Skill 管理组件
├── assets/                       # 静态资源
│   └── icons/
├── docs/                         # 文档
├── tests/                        # 测试文件
├── Cargo.toml                    # Rust 项目配置
├── package.json                  # Node 依赖配置
├── tauri.conf.json               # Tauri 配置
└── README.md
```

---

## Phase 1: 项目初始化与基础框架

### Task 1: 创建 Tauri 项目结构

**Files:**
- Create: `Cargo.toml`
- Create: `tauri.conf.json`
- Create: `package.json`
- Create: `src/main.rs`
- Create: `src/lib.rs`

- [ ] **Step 1: 创建项目目录结构**

```bash
mkdir -p claude-code-assistant/{src/{commands,models,services,utils},src-ui/components,assets/icons,docs,tests}
cd claude-code-assistant
```

- [ ] **Step 2: 创建 Cargo.toml**

```toml
[package]
name = "claude-code-assistant"
version = "0.1.0"
description = "A desktop assistant tool for Claude Code"
authors = ["Your Name"]
license = "MIT"
repository = ""
edition = "2021"
rust-version = "1.70"

[build-dependencies]
tauri-build = { version = "1.5", features = [] }

[dependencies]
tauri = { version = "1.5", features = ["shell-open", "dialog", "fs"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["full"] }
anyhow = "1.0"
dirs = "5.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]

[profile.release]
panic = "abort"
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

- [ ] **Step 3: 创建 tauri.conf.json**

```json
{
  "build": {
    "beforeBuildCommand": "",
    "beforeDevCommand": "",
    "devPath": "../src-ui",
    "distDir": "../src-ui",
    "withGlobalTauri": true
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "dialog": {
        "all": false,
        "open": true,
        "save": true
      },
      "fs": {
        "all": false,
        "readFile": true,
        "writeFile": true,
        "readDir": true,
        "copyFile": true,
        "createDir": true,
        "removeDir": true,
        "removeFile": true,
        "renameFile": true,
        "exists": true
      },
      "shell": {
        "all": false,
        "open": true
      },
      "window": {
        "all": false,
        "close": true,
        "minimize": true,
        "maximize": true,
        "startDragging": true
      }
    },
    "bundle": {
      "active": true,
      "category": "DeveloperTool",
      "copyright": "",
      "deb": {
        "depends": []
      },
      "externalBin": [],
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "identifier": "com.yourcompany.claude-code-assistant",
      "longDescription": "A desktop assistant tool for managing Claude Code configuration and providing quick access to common tasks.",
      "macOS": {
        "entitlements": null,
        "exceptionDomain": "",
        "frameworks": [],
        "minimumSystemVersion": "12.0",
        "signingIdentity": null
      },
      "resources": [],
      "shortDescription": "Claude Code Assistant",
      "targets": "all",
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": ""
      }
    },
    "security": {
      "csp": null
    },
    "updater": {
      "active": false
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 700,
        "resizable": true,
        "title": "Claude Code Assistant",
        "width": 1000,
        "minWidth": 800,
        "minHeight": 600
      }
    ]
  }
}
```

- [ ] **Step 4: 创建 package.json**

```json
{
  "name": "claude-code-assistant-ui",
  "version": "0.1.0",
  "description": "Frontend for Claude Code Assistant",
  "main": "index.html",
  "scripts": {
    "dev": "echo 'Dev server not needed for Tauri app'",
    "build": "echo 'No build step needed for vanilla JS'",
    "lint": "echo 'No linter configured'"
  },
  "keywords": ["claude", "assistant", "tauri"],
  "author": "Your Name",
  "license": "MIT",
  "devDependencies": {}
}
```

- [ ] **Step 5: 创建 src/main.rs**

```rust
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod commands;
mod models;
mod services;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Config commands
            commands::config::get_app_config,
            commands::config::save_app_config,
            commands::config::get_claude_config,
            commands::config::save_claude_config,
            // Project commands
            commands::projects::get_recent_projects,
            commands::projects::add_project,
            commands::projects::pin_project,
            commands::projects::remove_project,
            commands::projects::launch_claude_in_dir,
            // Profile commands
            commands::profiles::get_profiles,
            commands::profiles::create_profile,
            commands::profiles::update_profile,
            commands::profiles::apply_profile,
            commands::profiles::delete_profile,
            commands::profiles::export_profile,
            commands::profiles::import_profile,
            // Skill commands
            commands::skills::get_installed_skills,
            commands::skills::get_available_skills,
            commands::skills::install_skill,
            commands::skills::install_skill_from_url,
            commands::skills::update_skill,
            commands::skills::uninstall_skill,
            commands::skills::toggle_skill,
            commands::skills::get_skill_config,
            commands::skills::update_skill_config,
            commands::skills::check_skill_updates,
            // Permission commands
            commands::permissions::get_permissions,
            commands::permissions::update_permission,
            commands::permissions::update_permission_group,
            // Quick commands
            commands::quick_commands::get_quick_commands,
            commands::quick_commands::execute_quick_command,
            commands::quick_commands::add_custom_command,
            commands::quick_commands::remove_custom_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 6: 创建 src/lib.rs**

```rust
pub mod commands;
pub mod models;
pub mod services;
pub mod utils;
```

- [ ] **Step 7: 初始化 Git 仓库**

```bash
cd claude-code-assistant
git init
git add -A
git commit -m "Initial commit: project setup with Tauri framework"
```

---

## Phase 2: 核心数据模型和工具函数

### Task 2: 创建数据模型

**Files:**
- Create: `src/models/mod.rs`
- Create: `src/models/config.rs`
- Create: `src/models/project.rs`
- Create: `src/models/profile.rs`
- Create: `src/models/skill.rs`
- Create: `src/models/permission.rs`
- Create: `src/models/quick_command.rs`

- [ ] **Step 1: 创建 models/mod.rs**

```rust
pub mod config;
pub mod project;
pub mod profile;
pub mod skill;
pub mod permission;
pub mod quick_command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}
```

- [ ] **Step 2: 创建 models/config.rs**

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    pub general: GeneralSettings,
    pub claude_code: ClaudeCodeSettings,
    pub features: FeatureSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub theme: String,
    pub language: String,
    pub check_updates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodeSettings {
    pub executable_path: String,
    pub config_path: String,
    pub auto_detect: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSettings {
    pub recent_projects: RecentProjectSettings,
    pub quick_commands: QuickCommandSettings,
    pub config_profiles: ConfigProfileSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentProjectSettings {
    pub max_count: i32,
    pub pinned: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickCommandSettings {
    pub custom: Vec<CustomQuickCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomQuickCommand {
    pub id: String,
    pub name: String,
    pub command: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProfileSettings {
    pub active: String,
    pub profiles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfig {
    pub settings: HashMap<String, serde_json::Value>,
    pub raw_json: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            general: GeneralSettings {
                theme: "light".to_string(),
                language: "zh-CN".to_string(),
                check_updates: true,
            },
            claude_code: ClaudeCodeSettings {
                executable_path: "claude".to_string(),
                config_path: "~/.claude/settings.json".to_string(),
                auto_detect: true,
            },
            features: FeatureSettings {
                recent_projects: RecentProjectSettings {
                    max_count: 20,
                    pinned: vec![],
                },
                quick_commands: QuickCommandSettings {
                    custom: vec![],
                },
                config_profiles: ConfigProfileSettings {
                    active: "default".to_string(),
                    profiles: vec!["default".to_string()],
                },
            },
        }
    }
}
```

- [ ] **Step 3: 创建 models/project.rs**

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub last_opened: DateTime<Utc>,
    pub is_pinned: bool,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInput {
    pub path: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Project {
    pub fn new(path: String, name: Option<String>) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let name = name.unwrap_or_else(|| {
            std::path::Path::new(&path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string()
        });

        Self {
            id,
            name,
            path,
            description: None,
            last_opened: Utc::now(),
            is_pinned: false,
            color: None,
        }
    }
}
```

- [ ] **Step 4: 创建 models/profile.rs**

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub config: ProfileConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub permissions: Option<PermissionConfig>,
    pub hooks: Option<HookConfig>,
    pub settings: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {
    pub allow: Vec<String>,
    pub deny: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookConfig {
    pub before_commit: Vec<String>,
    pub after_commit: Vec<String>,
    pub before_push: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileInput {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl Profile {
    pub fn new(name: String, description: String) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        Self {
            id,
            name,
            description,
            tags: vec![],
            created_at: now,
            updated_at: now,
            config: ProfileConfig {
                permissions: None,
                hooks: None,
                settings: None,
            },
        }
    }
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            permissions: None,
            hooks: None,
            settings: None,
        }
    }
}
```

- [ ] **Step 5: 创建 models/skill.rs**

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub source: SkillSource,
    pub status: SkillStatus,
    pub installed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub config: Option<SkillConfig>,
    pub icon: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillSource {
    BuiltIn,
    Marketplace,
    Custom { url: String },
    Local { path: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillStatus {
    Active,
    Disabled,
    Error { message: String },
    Updating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConfig {
    pub schema: serde_json::Value,
    pub values: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillUpdateInfo {
    pub skill_id: String,
    pub current_version: String,
    pub latest_version: String,
    pub changelog: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInput {
    pub name: String,
    pub url: Option<String>,
    pub file_path: Option<String>,
}

impl Skill {
    pub fn is_updatable(&self) -> bool {
        matches!(self.source, SkillSource::Marketplace | SkillSource::BuiltIn)
    }

    pub fn is_configurable(&self) -> bool {
        self.config.is_some()
    }
}
```

- [ ] **Step 6: 创建 models/permission.rs**

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGroups {
    pub groups: Vec<PermissionGroup>,
    pub global_settings: GlobalPermissionSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGroup {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub commands: Vec<CommandPermission>,
    pub is_dangerous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPermission {
    pub id: String,
    pub name: String,
    pub description: String,
    pub allowed: bool,
    pub is_dangerous: bool,
    pub requires_confirmation: bool,
    pub parameters: Vec<CommandParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandParameter {
    pub name: String,
    pub description: String,
    pub parameter_type: ParameterType,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalPermissionSettings {
    pub default_allow: bool,
    pub require_confirmation_for_dangerous: bool,
    pub auto_save_changes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionUpdateRequest {
    pub command_id: String,
    pub allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGroupUpdateRequest {
    pub group_id: String,
    pub allowed: bool,
}

impl PermissionGroups {
    pub fn get_command(&self, command_id: &str) -> Option<&CommandPermission> {
        for group in &self.groups {
            for cmd in &group.commands {
                if cmd.id == command_id {
                    return Some(cmd);
                }
            }
        }
        None
    }

    pub fn get_dangerous_commands(&self) -> Vec<&CommandPermission> {
        let mut dangerous = Vec::new();
        for group in &self.groups {
            for cmd in &group.commands {
                if cmd.is_dangerous {
                    dangerous.push(cmd);
                }
            }
        }
        dangerous
    }
}
```

- [ ] **Step 7: 创建 models/quick_command.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickCommand {
    pub id: String,
    pub name: String,
    pub description: String,
    pub command: String,
    pub category: CommandCategory,
    pub icon: Option<String>,
    pub is_builtin: bool,
    pub is_custom: bool,
    pub parameters: Vec<CommandParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandCategory {
    Git,
    Code,
    Test,
    Deploy,
    Review,
    Doc,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandParam {
    pub name: String,
    pub description: String,
    pub param_type: ParamType,
    pub required: bool,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParamType {
    String,
    Number,
    Boolean,
    Select,
    Path,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickCommandInput {
    pub name: String,
    pub description: String,
    pub command: String,
    pub category: CommandCategory,
}

impl QuickCommand {
    pub fn requires_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }

    pub fn get_full_command(&self, params: &[(String, String)]) -> String {
        let mut full_cmd = self.command.clone();
        for (name, value) in params {
            full_cmd = full_cmd.replace(&format!("{{{{{}}}}}", name), value);
        }
        full_cmd
    }
}
```

- [ ] **Step 8: Commit 数据模型**

```bash
git add src/models/
git commit -m "feat: add core data models for config, projects, profiles, skills, permissions, and quick commands"
```

---

## Phase 3: 工具函数和基础服务

### Task 3: 创建工具函数

**Files:**
- Create: `src/utils/mod.rs`
- Create: `src/utils/paths.rs`
- Create: `src/utils/validators.rs`

- [ ] **Step 1: 创建 utils/mod.rs**

```rust
pub mod paths;
pub mod validators;
```

- [ ] **Step 2: 创建 utils/paths.rs**

```rust
use std::path::PathBuf;

pub fn get_app_data_dir() -> anyhow::Result<PathBuf> {
    let dir = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
        .join("claude-code-assistant");

    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn get_config_file_path() -> anyhow::Result<PathBuf> {
    Ok(get_app_data_dir()?.join("app-config.json"))
}

pub fn get_projects_file_path() -> anyhow::Result<PathBuf> {
    Ok(get_app_data_dir()?.join("projects.json"))
}

pub fn get_profiles_dir() -> anyhow::Result<PathBuf> {
    let dir = get_app_data_dir()?.join("profiles");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn get_claude_config_path() -> PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".claude").join("settings.json"))
        .unwrap_or_else(|| PathBuf::from(".claude/settings.json"))
}

pub fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        dirs::home_dir()
            .map(|h| h.join(&path[2..]))
            .unwrap_or_else(|| PathBuf::from(path))
    } else {
        PathBuf::from(path)
    }
}

pub fn normalize_path(path: &str) -> anyhow::Result<String> {
    let expanded = expand_tilde(path);
    let canonical = std::fs::canonicalize(&expanded)?;
    Ok(canonical.to_string_lossy().to_string())
}
```

- [ ] **Step 3: 创建 utils/validators.rs**

```rust
use std::path::Path;

pub fn is_valid_project_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 100
        && !name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|'])
}

pub fn is_valid_command_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 50
}

pub fn is_valid_profile_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 50
        && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

pub fn is_valid_directory_path(path: &str) -> bool {
    let path = std::path::Path::new(path);
    path.exists() && path.is_dir()
}

pub fn is_valid_file_path(path: &str) -> bool {
    let path = std::path::Path::new(path);
    path.exists() && path.is_file()
}

pub fn sanitize_filename(filename: &str) -> String {
    filename
        .replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_project_name() {
        assert!(is_valid_project_name("my-project"));
        assert!(is_valid_project_name("my_project"));
        assert!(!is_valid_project_name(""));
        assert!(!is_valid_project_name("my/project"));
    }

    #[test]
    fn test_is_valid_profile_name() {
        assert!(is_valid_profile_name("work-mode"));
        assert!(is_valid_profile_name("personal_mode"));
        assert!(!is_valid_profile_name(""));
        assert!(!is_valid_profile_name("work mode"));
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("file/name"), "file_name");
        assert_eq!(sanitize_filename("file:name"), "file_name");
    }
}
```

- [ ] **Step 4: Commit 工具函数**

```bash
git add src/utils/
git commit -m "feat: add utility functions for path handling and validation"
```

---

## Phase 4: 前端基础框架

### Task 4: 创建前端基础结构和样式

**Files:**
- Create: `src-ui/index.html`
- Create: `src-ui/styles.css`
- Create: `src-ui/main.js`

- [ ] **Step 1: 创建 src-ui/index.html**

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Claude Code 助手</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div id="app">
        <!-- 标题栏 -->
        <header class="title-bar">
            <div class="title-bar-left">
                <span class="logo">🔧</span>
                <h1>Claude Code 助手</h1>
            </div>
            <div class="title-bar-right">
                <button class="btn-icon" id="settings-btn" title="设置">⚙️</button>
                <button class="btn-icon" id="close-btn" title="关闭">✕</button>
            </div>
        </header>

        <!-- 主内容区 -->
        <main class="main-content">
            <!-- 仪表盘卡片网格 -->
            <div class="dashboard-grid" id="dashboard">
                <!-- 卡片将通过 JavaScript 动态生成 -->
            </div>
        </main>

        <!-- 加载遮罩 -->
        <div id="loading-overlay" class="loading-overlay hidden">
            <div class="loading-spinner"></div>
            <p>加载中...</p>
        </div>

        <!-- 提示消息 -->
        <div id="toast-container" class="toast-container"></div>
    </div>

    <!-- Tauri API -->
    <script src="https://unpkg.com/@tauri-apps/api@1.5.0/dist/tauri.global.js"></script>

    <!-- 应用脚本 -->
    <script src="main.js"></script>
</body>
</html>
```

- [ ] **Step 2: 创建 src-ui/styles.css**

```css
/* ========================================
   基础变量和重置
   ======================================== */
:root {
    /* 主色调 */
    --primary: #D97757;
    --primary-light: #E8956F;
    --primary-dark: #B55E3D;

    /* 背景色 */
    --bg-primary: #FAF9F6;
    --bg-secondary: #FFFFFF;
    --bg-hover: #F5F3EF;
    --bg-active: #EDEAE5;

    /* 文字色 */
    --text-primary: #2D2D2D;
    --text-secondary: #666666;
    --text-muted: #999999;
    --text-inverse: #FFFFFF;

    /* 边框 */
    --border-light: #E5E5E5;
    --border-medium: #D0D0D0;
    --border-focus: var(--primary);

    /* 功能色 */
    --success: #10A37F;
    --warning: #F59E0B;
    --error: #EF4444;
    --info: #3B82F6;

    /* 间距 */
    --space-xs: 4px;
    --space-sm: 8px;
    --space-md: 16px;
    --space-lg: 24px;
    --space-xl: 32px;

    /* 尺寸 */
    --header-height: 48px;
    --card-min-width: 280px;
    --border-radius-sm: 6px;
    --border-radius-md: 10px;
    --border-radius-lg: 16px;

    /* 字体 */
    --font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    --font-mono: "SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, monospace;

    /* 动画 */
    --transition-fast: 150ms ease;
    --transition-normal: 250ms ease;
    --transition-slow: 350ms ease;
}

/* ========================================
   重置和基础样式
   ======================================== */
*, *::before, *::after {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

html, body {
    height: 100%;
    font-family: var(--font-family);
    font-size: 14px;
    line-height: 1.5;
    color: var(--text-primary);
    background-color: var(--bg-primary);
    overflow: hidden;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
}

button {
    font-family: inherit;
    cursor: pointer;
    border: none;
    background: none;
    outline: none;
}

input, textarea {
    font-family: inherit;
    border: none;
    outline: none;
    background: none;
}

a {
    color: inherit;
    text-decoration: none;
}

/* ========================================
   应用容器
   ======================================== */
#app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
}

/* ========================================
   标题栏
   ======================================== */
.title-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: var(--header-height);
    padding: 0 var(--space-md);
    background-color: var(--bg-secondary);
    border-bottom: 1px solid var(--border-light);
    -webkit-app-region: drag;
}

.title-bar-left {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    -webkit-app-region: no-drag;
}

.title-bar .logo {
    font-size: 20px;
}

.title-bar h1 {
    font-size: 15px;
    font-weight: 600;
    color: var(--text-primary);
}

.title-bar-right {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    -webkit-app-region: no-drag;
}

.btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: var(--border-radius-sm);
    font-size: 16px;
    color: var(--text-secondary);
    transition: all var(--transition-fast);
}

.btn-icon:hover {
    background-color: var(--bg-hover);
    color: var(--text-primary);
}

.btn-icon:active {
    background-color: var(--bg-active);
}

/* ========================================
   主内容区
   ======================================== */
.main-content {
    flex: 1;
    overflow: auto;
    padding: var(--space-lg);
}

/* ========================================
   仪表盘网格
   ======================================== */
.dashboard-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--card-min-width), 1fr));
    gap: var(--space-lg);
    max-width: 1400px;
    margin: 0 auto;
}

/* ========================================
   卡片组件
   ======================================== */
.card {
    display: flex;
    flex-direction: column;
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-light);
    border-radius: var(--border-radius-md);
    padding: var(--space-lg);
    cursor: pointer;
    transition: all var(--transition-normal);
}

.card:hover {
    border-color: var(--border-medium);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
    transform: translateY(-2px);
}

.card:active {
    transform: translateY(0);
}

.card-header {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    margin-bottom: var(--space-md);
}

.card-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    background-color: var(--bg-hover);
    border-radius: var(--border-radius-sm);
    font-size: 20px;
}

.card-title {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
}

.card-description {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.5;
    flex: 1;
}

.card-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: var(--space-md);
    padding-top: var(--space-md);
    border-top: 1px solid var(--border-light);
}

.card-stats {
    font-size: 12px;
    color: var(--text-muted);
}

.card-action {
    font-size: 13px;
    font-weight: 500;
    color: var(--primary);
    transition: color var(--transition-fast);
}

.card:hover .card-action {
    color: var(--primary-dark);
}

/* ========================================
   按钮组件
   ======================================== */
.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-sm);
    height: 36px;
    padding: 0 var(--space-md);
    font-size: 13px;
    font-weight: 500;
    border-radius: var(--border-radius-sm);
    transition: all var(--transition-fast);
}

.btn-primary {
    background-color: var(--primary);
    color: var(--text-inverse);
}

.btn-primary:hover {
    background-color: var(--primary-dark);
}

.btn-secondary {
    background-color: var(--bg-hover);
    color: var(--text-primary);
    border: 1px solid var(--border-light);
}

.btn-secondary:hover {
    background-color: var(--bg-active);
    border-color: var(--border-medium);
}

.btn-danger {
    background-color: var(--error);
    color: var(--text-inverse);
}

.btn-danger:hover {
    background-color: #dc2626;
}

.btn-sm {
    height: 28px;
    padding: 0 var(--space-sm);
    font-size: 12px;
}

.btn-lg {
    height: 44px;
    padding: 0 var(--space-lg);
    font-size: 14px;
}

/* ========================================
   输入组件
   ======================================== */
.input {
    width: 100%;
    height: 36px;
    padding: 0 var(--space-md);
    font-size: 13px;
    color: var(--text-primary);
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-light);
    border-radius: var(--border-radius-sm);
    transition: all var(--transition-fast);
}

.input:hover {
    border-color: var(--border-medium);
}

.input:focus {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px rgba(217, 119, 87, 0.1);
}

.input::placeholder {
    color: var(--text-muted);
}

.input-error {
    border-color: var(--error);
}

.input-error:focus {
    box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
}

/* ========================================
   加载和提示
   ======================================== */
.loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background-color: rgba(250, 249, 246, 0.9);
    z-index: 1000;
    transition: opacity var(--transition-normal);
}

.loading-overlay.hidden {
    opacity: 0;
    pointer-events: none;
}

.loading-spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--border-light);
    border-top-color: var(--primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}

.loading-overlay p {
    margin-top: var(--space-md);
    color: var(--text-secondary);
    font-size: 14px;
}

/* Toast 提示 */
.toast-container {
    position: fixed;
    bottom: var(--space-lg);
    right: var(--space-lg);
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
    z-index: 1001;
}

.toast {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-md);
    background-color: var(--bg-secondary);
    border-radius: var(--border-radius-sm);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    animation: slideIn 0.3s ease;
    min-width: 280px;
    max-width: 400px;
}

@keyframes slideIn {
    from {
        transform: translateX(100%);
        opacity: 0;
    }
    to {
        transform: translateX(0);
        opacity: 1;
    }
}

.toast.toast-success {
    border-left: 4px solid var(--success);
}

.toast.toast-error {
    border-left: 4px solid var(--error);
}

.toast.toast-warning {
    border-left: 4px solid var(--warning);
}

.toast.toast-info {
    border-left: 4px solid var(--info);
}

.toast-message {
    flex: 1;
    font-size: 13px;
    color: var(--text-primary);
}

.toast-close {
    font-size: 18px;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0 4px;
}

.toast-close:hover {
    color: var(--text-primary);
}

/* ========================================
   滚动条
   ======================================== */
::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

::-webkit-scrollbar-track {
    background: var(--bg-hover);
    border-radius: 4px;
}

::-webkit-scrollbar-thumb {
    background: var(--border-medium);
    border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
    background: var(--text-muted);
}

/* ========================================
   工具类
   ======================================== */
.hidden {
    display: none !important;
}

.sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
}

.text-center {
    text-align: center;
}

.text-left {
    text-align: left;
}

.text-right {
    text-align: right;
}

.mt-sm { margin-top: var(--space-sm); }
.mt-md { margin-top: var(--space-md); }
.mt-lg { margin-top: var(--space-lg); }

.mb-sm { margin-bottom: var(--space-sm); }
.mb-md { margin-bottom: var(--space-md); }
.mb-lg { margin-bottom: var(--space-lg); }
```

- [ ] **Step 3: 创建 src-ui/main.js**

```javascript
/**
 * Claude Code 助手 - 前端主脚本
 */

// ========================================
// 全局状态
// ========================================
const AppState = {
    currentView: 'dashboard',
    isLoading: false,
    config: null,
    recentProjects: [],
    profiles: [],
    skills: [],
};

// ========================================
// 工具函数
// ========================================
const Utils = {
    /**
     * 显示/隐藏加载遮罩
     */
    toggleLoading(show, message = '加载中...') {
        const overlay = document.getElementById('loading-overlay');
        const text = overlay.querySelector('p');

        if (show) {
            text.textContent = message;
            overlay.classList.remove('hidden');
        } else {
            overlay.classList.add('hidden');
        }
    },

    /**
     * 显示 Toast 提示
     */
    showToast(message, type = 'info', duration = 3000) {
        const container = document.getElementById('toast-container');
        const toast = document.createElement('div');
        toast.className = `toast toast-${type}`;

        const iconMap = {
            success: '✓',
            error: '✕',
            warning: '⚠',
            info: 'ℹ'
        };

        toast.innerHTML = `
            <span class="toast-icon">${iconMap[type]}</span>
            <span class="toast-message">${message}</span>
            <span class="toast-close">×</span>
        `;

        toast.querySelector('.toast-close').addEventListener('click', () => {
            toast.remove();
        });

        container.appendChild(toast);

        if (duration > 0) {
            setTimeout(() => {
                toast.style.animation = 'slideIn 0.3s ease reverse';
                setTimeout(() => toast.remove(), 300);
            }, duration);
        }
    },

    /**
     * 调用 Tauri 命令
     */
    async invoke(command, args = {}) {
        try {
            const result = await window.__TAURI__.invoke(command, args);
            return result;
        } catch (error) {
            console.error(`Command ${command} failed:`, error);
            throw error;
        }
    },

    /**
     * 格式化日期
     */
    formatDate(dateStr) {
        const date = new Date(dateStr);
        return date.toLocaleDateString('zh-CN', {
            year: 'numeric',
            month: 'short',
            day: 'numeric'
        });
    },

    /**
     * 截断文本
     */
    truncate(str, maxLength = 50) {
        if (str.length <= maxLength) return str;
        return str.slice(0, maxLength) + '...';
    },

    /**
     * 生成唯一 ID
     */
    generateId() {
        return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    },

    /**
     * 防抖函数
     */
    debounce(func, wait = 300) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    },
};

// ========================================
// 卡片配置
// ========================================
const CardsConfig = [
    {
        id: 'permissions',
        icon: '🛡️',
        title: '权限管理',
        description: '查看和修改 Claude Code 的命令权限，按类别管理允许执行的操作',
        color: '#EF4444',
    },
    {
        id: 'projects',
        icon: '📂',
        title: '最近项目',
        description: '快速打开最近使用的项目目录，一键在指定目录启动 Claude Code',
        color: '#3B82F6',
    },
    {
        id: 'quick-commands',
        icon: '⚡',
        title: '快捷命令',
        description: '一键执行常用的 Claude Code 命令和自定义快捷操作',
        color: '#F59E0B',
    },
    {
        id: 'config',
        icon: '⚙️',
        title: '配置编辑',
        description: '图形化界面编辑 Claude Code 的 settings.json 配置文件',
        color: '#10A37F',
    },
    {
        id: 'profiles',
        icon: '🔄',
        title: '配置方案',
        description: '保存和切换多套 Claude Code 配置方案，适应不同使用场景',
        color: '#8B5CF6',
    },
    {
        id: 'skills',
        icon: '🧩',
        title: 'Skill 管理',
        description: '查看、安装、配置和管理 Claude Code 的 Skills 扩展',
        color: '#EC4899',
    },
];

// ========================================
// 仪表盘管理器
// ========================================
const Dashboard = {
    init() {
        this.render();
        this.attachEvents();
    },

    render() {
        const container = document.getElementById('dashboard');
        if (!container) return;

        container.innerHTML = CardsConfig.map(card => this.createCardHTML(card)).join('');
    },

    createCardHTML(card) {
        return `
            <div class="card" data-card-id="${card.id}">
                <div class="card-header">
                    <div class="card-icon" style="color: ${card.color}">
                        ${card.icon}
                    </div>
                    <h3 class="card-title">${card.title}</h3>
                </div>
                <p class="card-description">${card.description}</p>
                <div class="card-footer">
                    <span class="card-stats">点击查看详情</span>
                    <span class="card-action">打开 →</span>
                </div>
            </div>
        `;
    },

    attachEvents() {
        const container = document.getElementById('dashboard');
        if (!container) return;

        container.addEventListener('click', (e) => {
            const card = e.target.closest('.card');
            if (card) {
                const cardId = card.dataset.cardId;
                this.openCard(cardId);
            }
        });
    },

    openCard(cardId) {
        console.log(`Opening card: ${cardId}`);
        Utils.showToast(`正在打开: ${cardId}`, 'info');
        // TODO: 实现卡片详情页面的打开逻辑
    },
};

// ========================================
// 应用初始化
// ========================================
const App = {
    async init() {
        console.log('Claude Code Assistant starting...');

        // 初始化仪表盘
        Dashboard.init();

        // 绑定标题栏按钮
        this.bindTitleBar();

        // 加载初始数据
        await this.loadInitialData();

        console.log('App initialized successfully');
    },

    bindTitleBar() {
        const settingsBtn = document.getElementById('settings-btn');
        const closeBtn = document.getElementById('close-btn');

        settingsBtn?.addEventListener('click', () => {
            Utils.showToast('设置功能开发中...', 'info');
        });

        closeBtn?.addEventListener('click', async () => {
            try {
                await Utils.invoke('close_window');
            } catch (e) {
                console.log('Close window command not available');
                window.close();
            }
        });
    },

    async loadInitialData() {
        try {
            // TODO: 从后端加载配置、项目列表等初始数据
            // AppState.config = await Utils.invoke('get_app_config');
            // AppState.recentProjects = await Utils.invoke('get_recent_projects');
        } catch (error) {
            console.error('Failed to load initial data:', error);
        }
    },
};

// 启动应用
document.addEventListener('DOMContentLoaded', () => {
    App.init();
});

// 暴露到全局（便于调试）
window.AppState = AppState;
window.Utils = Utils;
window.Dashboard = Dashboard;
```

- [ ] **Step 4: Commit 前端基础框架**

```bash
git add src-ui/
git commit -m "feat: add frontend base structure with dashboard grid and card components"
```

---

**注意：** 由于实施计划内容非常长，以上只展示了 Phase 1-2 的部分内容。完整的实施计划应包含：
- Phase 3: 后端服务实现
- Phase 4: 各功能卡片完整实现
- Phase 5: 测试与打包发布

每个任务都包含详细的步骤、代码和命令。由于长度限制，建议将完整计划保存到文件中，然后分阶段执行。
