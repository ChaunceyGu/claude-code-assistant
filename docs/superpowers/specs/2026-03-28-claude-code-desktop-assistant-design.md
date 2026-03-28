# Claude Code 桌面辅助工具设计文档

## 项目概述

一个轻量级的桌面仪表盘工具，帮助用户可视化管理和快速操作 Claude Code。

## 核心定位

- **目标用户**: 觉得 Claude Code 命令行难用、功能不易发现的普通用户
- **核心价值**: 降低 Claude Code 的使用门槛，提供图形化配置和快捷操作
- **使用方式**: 轻量、即用即走，不常驻后台

## 技术架构

### 技术栈
- **框架**: Tauri (Rust + Web 技术)
- **前端**: HTML + CSS + JavaScript
- **存储**: 本地 JSON 文件
- **平台**: Windows 10/11, macOS 12+

### 选择理由
- 跨平台支持（Windows + macOS）
- 安装包体积小（相比 Electron）
- 启动速度快
- 原生性能
- 现代 Web 技术栈，开发灵活

## 界面设计

### 整体布局

仪表盘卡片风格，主窗口大小约 900x600 像素，可调整大小。

```
┌─────────────────────────────────────────────────────────────┐
│  🔧 Claude Code 助手                          [设置] [×] │
├─────────────────────────────────────────────────────────────┤
│  ┌────────────┐  ┌────────────┐  ┌────────────┐           │
│  │  🛡️ 权限管理 │  │ 📂 最近项目  │  │ ⚡ 快捷命令   │           │
│  │   查看/修改  │  │  快速打开   │  │  一键执行    │           │
│  │   命令权限   │  │  工作区管理  │  │  常用操作    │           │
│  └────────────┘  └────────────┘  └────────────┘           │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐           │
│  │  ⚙️ 配置编辑  │  │ 🔄 配置方案  │  │ 🧩 Skill管理 │           │
│  │  图形化编辑   │  │  保存/切换   │  │  查看/安装   │           │
│  │  settings   │  │  多套配置    │  │  启用/禁用   │           │
│  └────────────┘  └────────────┘  └────────────┘           │
└─────────────────────────────────────────────────────────────┘
```

### 卡片设计规范

每个卡片包含：
- **图标** (24x24px): 区分功能类型
- **标题**: 功能名称
- **描述**: 简短说明（2行）
- **悬停效果**: 轻微阴影 + 边框高亮
- **点击行为**: 展开详细面板或弹窗

### 颜色方案

- **主色调**: #D97757 (暖橙，Claude 品牌色)
- **背景**: #FAF9F6 (暖白)
- **卡片**: #FFFFFF (纯白)
- **文字**: #2D2D2D (深灰)
- **边框**: #E5E5E5 (浅灰)
- **强调**: #10A37F (绿，成功状态)

## 功能模块详解

### 1. 🛡️ 权限管理器

**功能**: 可视化查看和管理 Claude Code 的命令权限

**界面设计**:
```
┌─────────────────────────────────────────────────────┐
│  🛡️ 权限管理                                    [×] │
├─────────────────────────────────────────────────────┤
│  按类别查看命令权限                                  │
│  ┌──────────────────────────────────────────────┐  │
│  │ 🔍 [搜索命令...]                              │  │
│  └──────────────────────────────────────────────┘  │
│                                                     │
│  ▼ 📁 文件操作 (12 个命令)                          │
│    ☑️ Read 文件读取                              ✓  │
│    ☑️ Edit 文件编辑                              ✓  │
│    ☑️ Write 文件写入                             ✓  │
│    ☐ Bash (rm -rf) 危险命令                      ✗  │
│                                                     │
│  ▶ 🌐 网络操作 (5 个命令)                           │
│  ▶ 🔧 系统工具 (8 个命令)                           │
│                                                     │
│  [保存更改]  [重置为默认]                           │
└─────────────────────────────────────────────────────┘
```

**核心功能**:
- 按类别分组展示命令权限
- 一键启用/禁用特定命令
- 危险命令特殊标识
- 搜索过滤功能
- 保存配置到 `permissions.json`

---

### 2. 📂 最近项目/工作区

**功能**: 快速访问最近使用的项目目录

**界面设计**:
```
┌─────────────────────────────────────────────────────┐
│  📂 最近项目                                      [×] │
├─────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────┐  │
│  │ 🔍 [搜索项目...] 或粘贴路径                  │  │
│  └──────────────────────────────────────────────┘  │
│                                                     │
│  ⭐ 固定项目                                        │
│  ┌─────────────────────────────────────────────────┐│
│  │ 📁 my-awesome-project                          ││
│  │    ~/Work/projects/awesome-project            ││
│  │    [打开] [在终端启动Claude] [移除]             ││
│  └─────────────────────────────────────────────────┘│
│                                                     │
│  🕐 最近访问                                        │
│  ┌─────────────────────────────────────────────────┐│
│  │ 📁 another-project      ~/Work/another         ││
│  │ 📁 personal-site        ~/Projects/site        ││
│  └─────────────────────────────────────────────────┘│
│                                                     │
│  [+ 添加新项目]  [清空历史]                          │
└─────────────────────────────────────────────────────┘
```

**核心功能**:
- 记录最近打开的项目（最多 20 个）
- 固定常用项目到顶部
- 一键在指定目录启动 Claude Code
- 支持拖拽文件夹添加
- 搜索过滤功能

---

### 3. ⚡ 快捷命令面板

**功能**: 一键执行常用的 Claude Code 命令和自定义快捷操作

**界面设计**:
```
┌─────────────────────────────────────────────────────┐
│  ⚡ 快捷命令                                      [×] │
├─────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────┐  │
│  │ 🔍 [搜索命令...]                              │  │
│  └──────────────────────────────────────────────┘  │
│                                                     │
│  📌 内置快捷命令                                    │
│  ┌─────────────────────────────────────────────────┐│
│  │ 📝 /commit    自动提交当前更改         [执行]   ││
│  │ 🔍 /review    审查当前代码           [执行]   ││
│  │ 🧪 /test      运行测试               [执行]   ││
│  │ 📚 /doc       生成文档               [执行]   ││
│  │ 🔧 /fix       自动修复问题           [执行]   ││
│  └─────────────────────────────────────────────────┘│
│                                                     │
│  🔧 自定义快捷命令                                  │
│  ┌─────────────────────────────────────────────────┐│
│  │ 🚀 部署到测试环境  [执行] [编辑] [删除]          ││
│  │    命令: claude -p "部署当前项目到测试环境"   ││
│  │ 📊 生成代码统计    [执行] [编辑] [删除]          ││
│  └─────────────────────────────────────────────────┘│
│                                                     │
│  [+ 添加快捷命令]  [编辑内置命令]                     │
└─────────────────────────────────────────────────────┘
```

**核心功能**:
- 内置常用 Claude Code 命令快捷按钮
- 支持自定义快捷命令（自定义提示词）
- 命令参数配置
- 分类管理（内置/自定义）
- 拖拽排序
- 搜索过滤

---

### 4. ⚙️ 配置编辑器

**功能**: 图形化界面编辑 Claude Code 的 settings.json

**界面设计**:
```
┌─────────────────────────────────────────────────────────────┐
│  ⚙️ 配置编辑                                              [×] │
├─────────────────────────────────────────────────────────────┤
│  当前配置文件: ~/.claude/settings.json                      │
│  [保存更改] [撤销修改] [打开文件位置] [重置默认]             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  🔍 [搜索配置项...]                                         │
│                                                             │
│  ▼ 🎨 外观设置                                              │
│    ┌─────────────────────────────────────────────────┐       │
│    │ 主题                                        ▼ [暗色] │       │
│    │                                              [亮色] │       │
│    │ 字体大小                                  14 ▲▼  │       │
│    │ 行高                                      1.5 ▲▼  │       │
│    └─────────────────────────────────────────────────┘       │
│                                                             │
│  ▶ 🤖 AI 设置                                               │
│  ▶ ⌨️ 快捷键设置                                            │
│  ▶ 🔒 安全设置                                              │
│  ▶ 🧩 扩展设置                                              │
│                                                             │
│  [+ 添加自定义配置项]                                        │
└─────────────────────────────────────────────────────────────┘
```

**核心功能**:
- 按类别分组展示配置项
- 表单控件（开关、下拉、数字、文本等）
- 实时验证和错误提示
- 配置变更预览
- 备份和恢复功能
- 原始 JSON 编辑模式（高级用户）

---

### 5. 🔄 配置方案管理

**功能**: 保存和切换多套 Claude Code 配置方案

**界面设计**:
```
┌─────────────────────────────────────────────────────┐
│  🔄 配置方案                                      [×] │
├─────────────────────────────────────────────────────┤
│  快速在不同场景的配置间切换                           │
├─────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────┐  │
│  │ 🔍 [搜索方案...]                              │  │
│  └──────────────────────────────────────────────┘  │
│                                                     │
│  🏠 当前使用: 默认配置                              │
│                                                     │
│  ⭐ 我的方案                                        │
│  ┌─────────────────────────────────────────────────┐│
│  │ 💼 工作模式                                        ││
│  │    严格权限 + 企业规范 + 自动提交审查               ││
│  │    [激活] [编辑] [复制] [导出] [删除]              ││
│  ├─────────────────────────────────────────────────┤│
│  │ 🏠 个人模式                                        ││
│  │    宽松权限 + 完整功能 + 实验性特性                 ││
│  │    [激活] [编辑] [复制] [导出] [删除]              ││
│  ├─────────────────────────────────────────────────┤│
│  │ 🧪 测试模式                                        ││
│  │    沙盒环境 + 隔离测试 + 自动回滚                   ││
│  │    [激活] [编辑] [复制] [导出] [删除]              ││
│  └─────────────────────────────────────────────────┘│
│                                                     │
│  📥 导入方案                                        │
│  ┌─────────────────────────────────────────────────┐│
│  │ my-friend-config.json    [预览] [导入] [删除]    ││
│  └─────────────────────────────────────────────────┘│
│                                                     │
│  [+ 创建新方案]  [从当前配置创建]  [导入配置文件]    │
└─────────────────────────────────────────────────────┘
```

**核心功能**:
- 保存多套命名配置方案
- 一键切换配置方案
- 方案对比功能（查看两个方案的差异）
- 导入/导出配置方案（JSON 格式）
- 方案描述和标签
- 从当前配置快速创建新方案

---

### 6. 🧩 Skill 管理

**功能**: 查看、安装、配置和管理 Claude Code 的 Skills

**界面设计**:
```
┌─────────────────────────────────────────────────────┐
│  🧩 Skill 管理                                    [×] │
├─────────────────────────────────────────────────────┤
│  管理 Claude Code 的扩展 Skills                     │
├─────────────────────────────────────────────────────┤
│  [已安装]  [可安装]  [需要更新]  [已禁用]             │
├─────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────┐  │
│  │ 🔍 [搜索 Skill...]                            │  │
│  └──────────────────────────────────────────────┘  │
│                                                     │
│  ⭐ 核心 Skills（内置）                              │
│  ┌─────────────────────────────────────────────────┐│
│  │ 📝 commit        代码提交助手                   ││
│  │      v1.0.0  ·  内置  ·  已启用                  ││
│  │      自动生成规范的 commit message              ││
│  │                                        [配置]    ││
│  ├─────────────────────────────────────────────────┤│
│  │ 🔍 review-pr     代码审查助手                   ││
│  │      v2.1.0  ·  内置  ·  有更新                  ││
│  │      自动审查 Pull Request                      ││
│  │                              [更新] [配置]       ││
│  └─────────────────────────────────────────────────┘│
│                                                     │
│  📦 已安装 Skills                                   │
│  ┌─────────────────────────────────────────────────┐│
│  │ 🚀 deploy        部署助手         v1.3.0       ││
│  │      自动部署到各种云平台                        ││
│  │      [配置] [禁用] [卸载]                        ││
│  ├─────────────────────────────────────────────────┤│
│  │ 📊 analytics     代码分析助手       v0.9.2       ││
│  │      代码复杂度、依赖分析                        ││
│  │      [配置] [禁用] [卸载]                        ││
│  └─────────────────────────────────────────────────┘│
│                                                     │
│  [+ 从市场安装]  [+ 从 URL 安装]  [检查更新]        │
└─────────────────────────────────────────────────────┘
```

**核心功能**:
- **标签页切换**: 已安装 / 可安装 / 需要更新 / 已禁用
- **Skill 信息展示**: 名称、版本、描述、来源（内置/市场/自定义）
- **状态管理**: 启用/禁用、更新、卸载
- **参数配置**: 点击配置按钮编辑 Skill 参数
- **市场安装**: 浏览官方 Skill 市场，一键安装
- **自定义安装**: 从 URL 或本地文件安装
- **自动更新检查**: 检查并安装 Skill 更新

**技术实现**:
- 读取 Claude Code 的 skill 目录和配置文件
- 调用 CLI 命令进行 skill 操作（安装、更新、卸载等）
- 缓存 skill 信息列表，定期刷新

---

## 数据结构

### 配置方案 Schema
```json
{
  "id": "work-mode",
  "name": "工作模式",
  "description": "严格权限 + 企业规范 + 自动提交审查",
  "tags": ["工作", "企业"],
  "createdAt": "2026-03-28T10:30:00Z",
  "updatedAt": "2026-03-28T10:30:00Z",
  "config": {
    "permissions": {
      "allow": ["Read", "Edit", "Write"],
      "deny": ["Bash:rm -rf"]
    },
    "hooks": {
      "before_commit": ["lint", "test"]
    }
  }
}
```

### 应用设置 Schema
```json
{
  "version": "1.0.0",
  "general": {
    "theme": "light",
    "language": "zh-CN",
    "checkUpdates": true
  },
  "claudeCode": {
    "executablePath": "claude",
    "configPath": "~/.claude/settings.json"
  },
  "features": {
    "recentProjects": {
      "maxCount": 20,
      "pinned": ["/path/to/project1", "/path/to/project2"]
    },
    "quickCommands": {
      "custom": [
        { "name": "部署到测试", "command": "deploy --env=test" }
      ]
    },
    "configProfiles": {
      "active": "default",
      "profiles": ["default", "work-mode", "personal-mode"]
    }
  }
}
```

## 文件结构

```
claude-code-assistant/
├── src/
│   ├── main.rs           # Tauri 主进程入口
│   ├── lib.rs            # 核心库
│   ├── commands/         # Tauri 命令
│   │   ├── config.rs     # 配置相关命令
│   │   ├── claude.rs     # Claude Code 交互
│   │   ├── projects.rs   # 项目管理
│   │   ├── profiles.rs   # 配置方案
│   │   ├── skills.rs     # Skill 管理
│   │   └── permissions.rs # 权限管理
│   ├── models/           # 数据模型
│   │   ├── config.rs
│   │   ├── project.rs
│   │   ├── profile.rs
│   │   ├── skill.rs
│   │   └── permission.rs
│   └── utils/            # 工具函数
│       ├── fs.rs
│       ├── path.rs
│       └── validators.rs
├── src-ui/               # 前端代码
│   ├── index.html
│   ├── main.js
│   ├── styles.css
│   └── components/       # UI 组件
│       ├── Dashboard.js
│       ├── Card.js
│       ├── PermissionManager.js
│       ├── ProjectManager.js
│       ├── QuickCommands.js
│       ├── ConfigEditor.js
│       ├── ProfileManager.js
│       └── SkillManager.js
├── assets/               # 静态资源
│   ├── icons/
│   └── fonts/
├── docs/                 # 文档
├── Cargo.toml            # Rust 依赖
├── package.json          # 前端依赖
├── tauri.conf.json       # Tauri 配置
└── README.md
```

## API 设计

### Tauri 命令

```rust
// 配置相关
#[command]
async fn get_app_config() -> Result<AppConfig, String>;

#[command]
async fn save_app_config(config: AppConfig) -> Result<(), String>;

// Claude Code 交互
#[command]
async fn get_claude_config() -> Result<ClaudeConfig, String>;

#[command]
async fn save_claude_config(config: ClaudeConfig) -> Result<(), String>;

#[command]
async fn launch_claude_in_dir(path: String) -> Result<(), String>;

// 项目管理
#[command]
async fn get_recent_projects() -> Result<Vec<Project>, String>;

#[command]
async fn add_project(path: String) -> Result<(), String>;

#[command]
async fn pin_project(id: String) -> Result<(), String>;

#[command]
async fn remove_project(id: String) -> Result<(), String>;

// 配置方案
#[command]
async fn get_profiles() -> Result<Vec<Profile>, String>;

#[command]
async fn create_profile(name: String, description: String) -> Result<Profile, String>;

#[command]
async fn update_profile(id: String, config: ProfileConfig) -> Result<(), String>;

#[command]
async fn apply_profile(id: String) -> Result<(), String>;

#[command]
async fn delete_profile(id: String) -> Result<(), String>;

#[command]
async fn export_profile(id: String, path: String) -> Result<(), String>;

#[command]
async fn import_profile(path: String) -> Result<Profile, String>;

// Skill 管理
#[command]
async fn get_installed_skills() -> Result<Vec<Skill>, String>;

#[command]
async fn get_available_skills() -> Result<Vec<Skill>, String>;

#[command]
async fn install_skill(name: String) -> Result<(), String>;

#[command]
async fn install_skill_from_url(url: String) -> Result<(), String>;

#[command]
async fn update_skill(name: String) -> Result<(), String>;

#[command]
async fn uninstall_skill(name: String) -> Result<(), String>;

#[command]
async fn toggle_skill(name: String, enabled: bool) -> Result<(), String>;

#[command]
async fn get_skill_config(name: String) -> Result<SkillConfig, String>;

#[command]
async fn update_skill_config(name: String, config: SkillConfig) -> Result<(), String>;

#[command]
async fn check_skill_updates() -> Result<Vec<SkillUpdateInfo>, String>;

// 权限管理
#[command]
async fn get_permissions() -> Result<PermissionGroups, String>;

#[command]
async fn update_permission(command: String, allowed: bool) -> Result<(), String>;

#[command]
async fn update_permission_group(group: String, allowed: bool) -> Result<(), String>;
```

## 开发路线图

### Phase 1: 基础框架 (Week 1-2)
- [ ] 搭建 Tauri 项目框架
- [ ] 设计并实现基础 UI 组件
- [ ] 实现配置读写模块
- [ ] 基础窗口管理

### Phase 2: 核心功能 (Week 3-4)
- [ ] 配置编辑器模块
- [ ] 最近项目管理模块
- [ ] 快捷命令面板模块
- [ ] 配置方案管理模块

### Phase 3: 高级功能 (Week 5-6)
- [ ] 权限管理器模块
- [ ] Skill 管理模块
- [ ] 配置导入/导出功能
- [ ] 数据备份与恢复

### Phase 4: 优化与发布 (Week 7-8)
- [ ] UI/UX 优化
- [ ] 性能优化
- [ ] 跨平台测试
- [ ] 打包发布

## 风险评估

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| Claude Code CLI 接口变动 | 高 | 封装 CLI 调用层，便于适配新接口 |
| 配置文件格式变更 | 中 | 使用版本化的配置 Schema，支持迁移 |
| Tauri 跨平台兼容性问题 | 中 | 早期开始在双平台上测试 |
| 性能问题（大量配置项目） | 低 | 虚拟列表、懒加载等优化手段 |

## 附录

### 参考资料
- [Tauri 官方文档](https://tauri.app/)
- [Claude Code 官方文档](https://docs.anthropic.com/claude-code/)
- [Rust 官方文档](https://doc.rust-lang.org/)

### 术语表
- **Profile（配置方案）**: 一组 Claude Code 配置的集合，用于快速切换不同场景的配置
- **Skill**: Claude Code 的扩展功能，以特定格式编写的提示词和工具集
- **Permission（权限）**: Claude Code 允许或禁止执行的命令类型

---

*文档版本: 1.0*
*最后更新: 2026-03-28*
