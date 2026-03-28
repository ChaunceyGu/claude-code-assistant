# Claude Code 桌面助手

[![Build Status](https://github.com/ChaunceyGu/claude-code-assistant/actions/workflows/build.yml/badge.svg)](https://github.com/ChaunceyGu/claude-code-assistant/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个基于 Tauri (Rust + Web) 构建的跨平台桌面应用，帮助用户更方便地管理和使用 Claude Code CLI 工具。

## ✨ 功能特性

- 🛡️ **权限管理** - 可视化查看和修改 Claude Code 的命令权限
- 📂 **最近项目** - 快速访问最近使用的项目目录
- ⚡ **快捷命令** - 一键执行常用的 Claude Code 命令
- ⚙️ **配置编辑** - 图形化界面编辑 settings.json
- 🔄 **配置方案** - 保存多套配置方案，一键切换
- 🧩 **Skill 管理** - 管理和配置 Claude Code 的扩展技能

## 🚀 快速开始

### 下载安装

1. 前往 [Releases](https://github.com/ChaunceyGu/claude-code-assistant/releases) 页面
2. 下载最新版本的 `.msi` 安装包或 `.exe` 便携版
3. 运行安装程序或直接运行 exe 文件

### 从源码编译

#### 环境要求

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- [Node.js](https://nodejs.org/) (20+)
- [Visual Studio 2022 Build Tools](https://visualstudio.microsoft.com/downloads/) (Windows)

#### 编译步骤

```bash
# 克隆仓库
git clone https://github.com/ChaunceyGu/claude-code-assistant.git
cd claude-code-assistant

# 安装依赖
cargo install tauri-cli --locked

# 编译
cargo tauri build

# 编译后的文件在：
# target/release/Claude Code Assistant.exe
# target/release/bundle/msi/*.msi
```

## 🛠️ 技术栈

- **后端**: Rust + Tauri
- **前端**: HTML5 + CSS3 + JavaScript (ES6+)
- **构建工具**: Cargo (Rust), Tauri CLI

## 📁 项目结构

```
claude-code-assistant/
├── src/                    # Rust 后端代码
│   ├── commands/          # Tauri 命令处理器
│   ├── models/            # 数据模型
│   ├── services/          # 业务逻辑服务
│   └── utils/             # 工具函数
├── src-ui/                 # 前端代码
│   ├── components/        # UI 组件
│   ├── index.html         # 主页面
│   ├── styles.css         # 样式文件
│   └── main.js            # 入口脚本
├── .github/workflows/    # GitHub Actions 配置
├── Cargo.toml            # Rust 项目配置
├── package.json          # 前端依赖配置
├── tauri.conf.json       # Tauri 配置
└── README.md             # 项目说明
```

## 🤝 贡献指南

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 [MIT](LICENSE) 许可证。

## 🙏 致谢

- [Tauri](https://tauri.app/) - 构建跨平台桌面应用的框架
- [Claude Code](https://github.com/anthropics/claude-code) - Anthropic 的 Claude 命令行工具

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/ChaunceyGu">ChaunceyGu</a>
</p>
