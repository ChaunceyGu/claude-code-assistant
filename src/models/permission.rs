use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGroups {
    #[serde(default)]
    pub groups: Vec<PermissionGroup>,
    #[serde(default)]
    pub global_settings: GlobalPermissionSettings,
}

impl PermissionGroups {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            global_settings: GlobalPermissionSettings::default(),
        }
    }

    pub fn get_command(&self, command_id: &str) -> Option<&CommandPermission> {
        for group in &self.groups {
            if let Some(cmd) = group.commands.iter().find(|c| c.id == command_id) {
                return Some(cmd);
            }
        }
        None
    }

    pub fn get_command_mut(&mut self, command_id: &str) -> Option<&mut CommandPermission> {
        for group in &mut self.groups {
            if let Some(cmd) = group.commands.iter_mut().find(|c| c.id == command_id) {
                return Some(cmd);
            }
        }
        None
    }

    pub fn get_dangerous_commands(&self) -> Vec<&CommandPermission> {
        let mut dangerous = Vec::new();
        for group in &self.groups {
            for cmd in &group.commands {
                if cmd.is_dangerous || group.is_dangerous {
                    dangerous.push(cmd);
                }
            }
        }
        dangerous
    }

    pub fn add_group(&mut self, group: PermissionGroup) {
        self.groups.push(group);
    }

    pub fn remove_group(&mut self, group_id: &str) -> Option<PermissionGroup> {
        if let Some(index) = self.groups.iter().position(|g| g.id == group_id) {
            Some(self.groups.remove(index))
        } else {
            None
        }
    }
}

impl Default for PermissionGroups {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGroup {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub commands: Vec<CommandPermission>,
    #[serde(default)]
    pub is_dangerous: bool,
}

impl PermissionGroup {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            icon: None,
            commands: Vec::new(),
            is_dangerous: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_dangerous(mut self, dangerous: bool) -> Self {
        self.is_dangerous = dangerous;
        self
    }

    pub fn add_command(&mut self, command: CommandPermission) {
        self.commands.push(command);
    }

    pub fn remove_command(&mut self, command_id: &str) -> Option<CommandPermission> {
        if let Some(index) = self.commands.iter().position(|c| c.id == command_id) {
            Some(self.commands.remove(index))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPermission {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub allowed: bool,
    #[serde(default)]
    pub is_dangerous: bool,
    #[serde(default)]
    pub requires_confirmation: bool,
    #[serde(default)]
    pub parameters: Vec<CommandParameter>,
}

impl CommandPermission {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            allowed: true,
            is_dangerous: false,
            requires_confirmation: false,
            parameters: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_allowed(mut self, allowed: bool) -> Self {
        self.allowed = allowed;
        self
    }

    pub fn with_dangerous(mut self, dangerous: bool) -> Self {
        self.is_dangerous = dangerous;
        self
    }

    pub fn with_confirmation(mut self, requires: bool) -> Self {
        self.requires_confirmation = requires;
        self
    }

    pub fn add_parameter(&mut self, param: CommandParameter) {
        self.parameters.push(param);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandParameter {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub parameter_type: ParameterType,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub default_value: Option<serde_json::Value>,
}

impl CommandParameter {
    pub fn new(name: impl Into<String>, parameter_type: ParameterType) -> Self {
        Self {
            name: name.into(),
            description: None,
            parameter_type,
            required: false,
            default_value: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn with_default_value(mut self, value: serde_json::Value) -> Self {
        self.default_value = Some(value);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

impl Default for ParameterType {
    fn default() -> Self {
        Self::String
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalPermissionSettings {
    #[serde(default)]
    pub default_allow_all: bool,
    #[serde(default)]
    pub require_confirmation_for_dangerous: bool,
    #[serde(default)]
    pub log_all_permissions: bool,
    #[serde(default)]
    pub permission_timeout: u64,
    #[serde(default)]
    pub custom_rules: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionUpdateRequest {
    pub command_id: String,
    pub allowed: bool,
    #[serde(default)]
    pub requires_confirmation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGroupUpdateRequest {
    pub group_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub commands: Option<Vec<CommandPermission>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheckResult {
    pub allowed: bool,
    pub requires_confirmation: bool,
    pub reason: Option<String>,
    pub command: Option<CommandPermission>,
}

impl PermissionCheckResult {
    pub fn allowed() -> Self {
        Self {
            allowed: true,
            requires_confirmation: false,
            reason: None,
            command: None,
        }
    }

    pub fn denied(reason: impl Into<String>) -> Self {
        Self {
            allowed: false,
            requires_confirmation: false,
            reason: Some(reason.into()),
            command: None,
        }
    }

    pub fn with_confirmation(mut self) -> Self {
        self.requires_confirmation = true;
        self
    }

    pub fn with_command(mut self, command: CommandPermission) -> Self {
        self.command = Some(command);
        self
    }
}
