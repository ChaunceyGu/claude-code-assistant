use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickCommand {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub command: String,
    #[serde(default)]
    pub category: CommandCategory,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub is_builtin: bool,
    #[serde(default)]
    pub is_custom: bool,
    #[serde(default)]
    pub parameters: Vec<CommandParam>,
}

impl QuickCommand {
    pub fn new(
        name: impl Into<String>,
        command: impl Into<String>,
        category: CommandCategory,
    ) -> Self {
        let name_str = name.into();
        let id = format!("cmd_{}", uuid::Uuid::new_v4().simple());

        Self {
            id,
            name: name_str,
            description: None,
            command: command.into(),
            category,
            icon: None,
            is_builtin: false,
            is_custom: true,
            parameters: Vec::new(),
        }
    }

    pub fn builtin(
        id: impl Into<String>,
        name: impl Into<String>,
        command: impl Into<String>,
        category: CommandCategory,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            command: command.into(),
            category,
            icon: None,
            is_builtin: true,
            is_custom: false,
            parameters: Vec::new(),
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

    pub fn with_parameters(mut self, params: Vec<CommandParam>) -> Self {
        self.parameters = params;
        self
    }

    pub fn add_parameter(&mut self, param: CommandParam) {
        self.parameters.push(param);
    }

    pub fn requires_parameters(&self) -> bool {
        !self.parameters.is_empty()
            && self
                .parameters
                .iter()
                .any(|p| p.required && p.default.is_none())
    }

    pub fn get_full_command(&self, param_values: &HashMap<String, String>) -> String {
        let mut command = self.command.clone();

        for param in &self.parameters {
            let value = param_values
                .get(&param.name)
                .cloned()
                .or_else(|| param.default.clone())
                .unwrap_or_default();

            command = command.replace(&format!("{{{{{}}}}}", param.name), &value);
        }

        command
    }

    pub fn validate_parameters(&self, values: &HashMap<String, String>) -> Vec<String> {
        let mut errors = Vec::new();

        for param in &self.parameters {
            if param.required && !values.contains_key(&param.name) && param.default.is_none() {
                errors.push(format!("Missing required parameter: {}", param.name));
            }
        }

        errors
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl Default for CommandCategory {
    fn default() -> Self {
        Self::Custom
    }
}

impl CommandCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Git => "git",
            Self::Code => "code",
            Self::Test => "test",
            Self::Deploy => "deploy",
            Self::Review => "review",
            Self::Doc => "doc",
            Self::Custom => "custom",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Git => "git-branch",
            Self::Code => "code",
            Self::Test => "check-circle",
            Self::Deploy => "rocket",
            Self::Review => "eye",
            Self::Doc => "file-text",
            Self::Custom => "command",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandParam {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub param_type: ParamType,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
}

impl CommandParam {
    pub fn new(name: impl Into<String>, param_type: ParamType) -> Self {
        Self {
            name: name.into(),
            description: None,
            param_type,
            required: false,
            default: None,
            options: Vec::new(),
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

    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }

    pub fn with_options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ParamType {
    String,
    Number,
    Boolean,
    Select,
    Path,
}

impl Default for ParamType {
    fn default() -> Self {
        Self::String
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickCommandInput {
    pub name: String,
    pub description: Option<String>,
    pub command: String,
    #[serde(default)]
    pub category: CommandCategory,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub parameters: Vec<CommandParam>,
}

impl QuickCommandInput {
    pub fn new(name: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            command: command.into(),
            category: CommandCategory::Custom,
            icon: None,
            parameters: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickCommandUpdateInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub command: Option<String>,
    #[serde(default)]
    pub category: Option<CommandCategory>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub parameters: Option<Vec<CommandParam>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickCommandList {
    pub commands: Vec<QuickCommand>,
    pub total: usize,
    pub builtin: Vec<QuickCommand>,
    pub custom: Vec<QuickCommand>,
}

impl QuickCommandList {
    pub fn new(commands: Vec<QuickCommand>) -> Self {
        let total = commands.len();
        let builtin: Vec<QuickCommand> = commands
            .iter()
            .filter(|c| c.is_builtin)
            .cloned()
            .collect();
        let custom: Vec<QuickCommand> = commands
            .iter()
            .filter(|c| c.is_custom)
            .cloned()
            .collect();

        Self {
            commands,
            total,
            builtin,
            custom,
        }
    }

    pub fn by_category(&self, category: &CommandCategory) -> Vec<&QuickCommand> {
        self.commands
            .iter()
            .filter(|c| &c.category == category)
            .collect()
    }
}
