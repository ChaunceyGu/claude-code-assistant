/**
 * Claude Code 桌面助手 - 前端主框架
 * 版本: 1.0.0
 */

// ==========================================================================
// 全局状态管理 AppState
// ==========================================================================
const AppState = {
    currentView: 'dashboard',
    isLoading: false,
    config: null,
    recentProjects: [],
    profiles: [],
    skills: [],
    permissions: [],
    quickCommands: [],
    isTauriAvailable: typeof window.__TAURI__ !== 'undefined',
};

// ==========================================================================
// 工具函数 Utils
// ==========================================================================
const Utils = {
    /**
     * 显示/隐藏加载遮罩
     * @param {boolean} show - 是否显示
     * @param {string} message - 加载消息
     */
    toggleLoading(show, message = '加载中...') {
        const overlay = document.getElementById('loadingOverlay');
        const msgEl = document.getElementById('loadingMessage');

        if (!overlay) return;

        AppState.isLoading = show;

        if (msgEl) msgEl.textContent = message;

        if (show) {
            overlay.classList.add('active');
        } else {
            overlay.classList.remove('active');
        }
    },

    /**
     * 显示 Toast 提示
     * @param {string} message - 消息内容
     * @param {string} type - 类型: success, error, warning, info
     * @param {number} duration - 显示时长(毫秒)
     */
    showToast(message, type = 'info', duration = 3000) {
        const container = document.getElementById('toastContainer');
        if (!container) return;

        // 创建 Toast 元素
        const toast = document.createElement('div');
        toast.className = `toast toast-${type}`;

        // 图标映射
        const icons = {
            success: '✓',
            error: '✕',
            warning: '!',
            info: 'i'
        };

        toast.innerHTML = `
            <div class="toast-icon">${icons[type] || icons.info}</div>
            <div class="toast-content">
                <p class="toast-message">${message}</p>
            </div>
            <button class="toast-close" aria-label="关闭">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
            </button>
        `;

        // 添加关闭事件
        const closeBtn = toast.querySelector('.toast-close');
        closeBtn.addEventListener('click', () => {
            toast.classList.add('hiding');
            setTimeout(() => toast.remove(), 250);
        });

        container.appendChild(toast);

        // 自动关闭
        if (duration > 0) {
            setTimeout(() => {
                if (toast.parentElement) {
                    toast.classList.add('hiding');
                    setTimeout(() => toast.remove(), 250);
                }
            }, duration);
        }
    },

    /**
     * 调用 Tauri 命令
     * @param {string} command - 命令名
     * @param {object} args - 参数
     * @returns {Promise}
     */
    async invoke(command, args = {}) {
        if (!AppState.isTauriAvailable) {
            console.warn(`Tauri 不可用，命令 "${command}" 无法执行`);
            throw new Error('Tauri API 不可用');
        }

        try {
            return await window.__TAURI__.invoke(command, args);
        } catch (error) {
            console.error(`命令 "${command}" 执行失败:`, error);
            throw error;
        }
    },

    /**
     * 格式化日期
     * @param {string|Date} dateStr - 日期字符串或对象
     * @param {string} format - 格式类型
     * @returns {string}
     */
    formatDate(dateStr, format = 'short') {
        const date = new Date(dateStr);
        if (isNaN(date.getTime())) return '-';

        const formats = {
            short: { month: 'short', day: 'numeric' },
            long: { year: 'numeric', month: 'long', day: 'numeric' },
            full: { year: 'numeric', month: 'long', day: 'numeric', hour: '2-digit', minute: '2-digit' }
        };

        return date.toLocaleDateString('zh-CN', formats[format] || formats.short);
    },

    /**
     * 截断文本
     * @param {string} str - 原始字符串
     * @param {number} maxLength - 最大长度
     * @param {string} suffix - 后缀
     * @returns {string}
     */
    truncate(str, maxLength = 50, suffix = '...') {
        if (!str || str.length <= maxLength) return str;
        return str.substring(0, maxLength - suffix.length) + suffix;
    },

    /**
     * 生成唯一 ID
     * @returns {string}
     */
    generateId() {
        return `${Date.now().toString(36)}_${Math.random().toString(36).substr(2, 9)}`;
    },

    /**
     * 防抖函数
     * @param {Function} func - 目标函数
     * @param {number} wait - 等待时间
     * @returns {Function}
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

    /**
     * 节流函数
     * @param {Function} func - 目标函数
     * @param {number} limit - 限制时间
     * @returns {Function}
     */
    throttle(func, limit = 300) {
        let inThrottle;
        return function executedFunction(...args) {
            if (!inThrottle) {
                func(...args);
                inThrottle = true;
                setTimeout(() => inThrottle = false, limit);
            }
        };
    }
};

// ==========================================================================
// 卡片配置 CardsConfig
// ==========================================================================
const CardsConfig = [
    {
        id: 'permissions',
        icon: '🛡️',
        iconColor: '#EF4444',
        title: '权限管理',
        description: '管理文件系统访问权限、网络权限和其他安全设置',
        badge: '已配置',
        meta: '12 项权限',
        action: '管理权限',
    },
    {
        id: 'projects',
        icon: '📂',
        iconColor: '#3B82F6',
        title: '最近项目',
        description: '快速访问最近使用的工作区和项目目录',
        badge: null,
        meta: '8 个项目',
        action: '查看全部',
    },
    {
        id: 'quick-commands',
        icon: '⚡',
        iconColor: '#F59E0B',
        title: '快捷命令',
        description: '一键执行常用命令和自定义快捷操作',
        badge: '新',
        meta: '15 条命令',
        action: '添加命令',
    },
    {
        id: 'config',
        icon: '⚙️',
        iconColor: '#10A37F',
        title: '配置编辑',
        description: '编辑 Claude Code 的配置文件和设置项',
        badge: null,
        meta: '上次编辑: 2小时前',
        action: '编辑配置',
    },
    {
        id: 'profiles',
        icon: '🔄',
        iconColor: '#8B5CF6',
        title: '配置方案',
        description: '切换和管理不同的配置预设方案',
        badge: '3 个方案',
        meta: '当前: 默认方案',
        action: '切换方案',
    },
    {
        id: 'skills',
        icon: '🧩',
        iconColor: '#EC4899',
        title: 'Skill 管理',
        description: '管理和配置 Claude Code 的扩展技能',
        badge: '已安装',
        meta: '6 个技能',
        action: '管理技能',
    },
];

// ==========================================================================
// 仪表盘管理器 Dashboard
// ==========================================================================
const Dashboard = {
    container: null,

    /**
     * 初始化仪表盘
     */
    init() {
        this.container = document.getElementById('dashboardGrid');
        if (!this.container) {
            console.error('找不到仪表盘容器元素 #dashboardGrid');
            return;
        }
        this.render();
        this.attachEvents();
        console.log('仪表盘已初始化');
    },

    /**
     * 渲染卡片网格
     */
    render() {
        if (!this.container) return;

        const html = CardsConfig.map(card => this.createCardHTML(card)).join('');
        this.container.innerHTML = html;

        // 添加入场动画
        const cards = this.container.querySelectorAll('.card');
        cards.forEach((card, index) => {
            card.style.opacity = '0';
            card.style.transform = 'translateY(20px)';
            setTimeout(() => {
                card.style.transition = 'all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1)';
                card.style.opacity = '1';
                card.style.transform = 'translateY(0)';
            }, index * 80);
        });
    },

    /**
     * 创建卡片 HTML
     * @param {object} card - 卡片配置
     * @returns {string} HTML 字符串
     */
    createCardHTML(card) {
        const badgeHTML = card.badge
            ? `<span class="card-badge">${card.badge}</span>`
            : '';

        return `
            <article class="card" data-card-id="${card.id}" role="button" tabindex="0">
                <div class="card-header">
                    <div class="card-icon ${card.id}" style="color: ${card.iconColor}">
                        ${card.icon}
                    </div>
                    ${badgeHTML}
                </div>
                <div class="card-content">
                    <h2 class="card-title">${card.title}</h2>
                    <p class="card-description">${card.description}</p>
                </div>
                <div class="card-footer">
                    <span class="card-meta">${card.meta}</span>
                    <span class="card-action">${card.action} →</span>
                </div>
            </article>
        `;
    },

    /**
     * 绑定事件
     */
    attachEvents() {
        if (!this.container) return;

        // 点击卡片
        this.container.addEventListener('click', (e) => {
            const card = e.target.closest('.card');
            if (!card) return;

            const cardId = card.dataset.cardId;
            if (cardId) {
                this.openCard(cardId);
            }
        });

        // 键盘导航
        this.container.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                const card = e.target.closest('.card');
                if (card) {
                    const cardId = card.dataset.cardId;
                    if (cardId) this.openCard(cardId);
                }
            }
        });
    },

    /**
     * 打开卡片详情
     * @param {string} cardId - 卡片 ID
     */
    openCard(cardId) {
        const card = CardsConfig.find(c => c.id === cardId);
        if (!card) return;

        console.log(`打开卡片: ${card.title} (${cardId})`);

        // 显示提示
        Utils.showToast(`正在打开 ${card.title}...`, 'info', 2000);

        // 这里可以根据不同卡片执行不同操作
        switch (cardId) {
            case 'permissions':
                this.openPermissionsManager();
                break;
            case 'projects':
                this.openProjectsView();
                break;
            case 'quick-commands':
                this.openQuickCommands();
                break;
            case 'config':
                this.openConfigEditor();
                break;
            case 'profiles':
                this.openProfilesManager();
                break;
            case 'skills':
                this.openSkillsManager();
                break;
            default:
                console.warn(`未知的卡片 ID: ${cardId}`);
        }
    },

    // 各个模块的打开方法
    openPermissionsManager() {
        // TODO: 实现权限管理器
        console.log('打开权限管理器...');
    },

    openProjectsView() {
        // TODO: 实现项目视图
        console.log('打开项目视图...');
    },

    openQuickCommands() {
        // TODO: 实现快捷命令
        console.log('打开快捷命令...');
    },

    openConfigEditor() {
        // TODO: 实现配置编辑器
        console.log('打开配置编辑器...');
    },

    openProfilesManager() {
        // TODO: 实现配置方案管理器
        console.log('打开配置方案管理器...');
    },

    openSkillsManager() {
        // TODO: 实现 Skill 管理器
        console.log('打开 Skill 管理器...');
    },
};

// ==========================================================================
// 应用初始化 App
// ==========================================================================
const App = {
    /**
     * 初始化应用
     */
    async init() {
        console.log('Claude Code 桌面助手 - 正在初始化...');

        try {
            // 绑定标题栏按钮
            this.bindTitleBar();

            // 初始化仪表盘
            Dashboard.init();

            // 加载初始数据
            await this.loadInitialData();

            // 检查 Tauri 可用性
            if (AppState.isTauriAvailable) {
                console.log('Tauri API 已加载');
                Utils.showToast('欢迎使用 Claude Code 桌面助手！', 'success', 3000);
            } else {
                console.warn('Tauri API 不可用，运行在 Web 模式');
                Utils.showToast('运行在 Web 模式，部分功能可能不可用', 'warning', 5000);
            }

            console.log('应用初始化完成');
        } catch (error) {
            console.error('应用初始化失败:', error);
            Utils.showToast('应用初始化失败，请刷新重试', 'error', 5000);
        }
    },

    /**
     * 绑定标题栏按钮
     */
    bindTitleBar() {
        const settingsBtn = document.getElementById('settingsBtn');
        const minimizeBtn = document.getElementById('minimizeBtn');
        const closeBtn = document.getElementById('closeBtn');

        if (settingsBtn) {
            settingsBtn.addEventListener('click', () => {
                console.log('打开设置...');
                Utils.showToast('设置功能开发中...', 'info', 2000);
                // TODO: 打开设置面板
            });
        }

        if (minimizeBtn) {
            minimizeBtn.addEventListener('click', async () => {
                if (AppState.isTauriAvailable) {
                    try {
                        const { appWindow } = window.__TAURI__.window;
                        await appWindow.minimize();
                    } catch (error) {
                        console.error('最小化窗口失败:', error);
                    }
                } else {
                    console.log('Web 模式下无法最小化窗口');
                }
            });
        }

        if (closeBtn) {
            closeBtn.addEventListener('click', async () => {
                if (AppState.isTauriAvailable) {
                    try {
                        const { appWindow } = window.__TAURI__.window;
                        await appWindow.close();
                    } catch (error) {
                        console.error('关闭窗口失败:', error);
                    }
                } else {
                    console.log('Web 模式下无法关闭窗口');
                    if (confirm('确定要关闭应用吗？')) {
                        window.close();
                    }
                }
            });
        }
    },

    /**
     * 加载初始数据
     */
    async loadInitialData() {
        Utils.toggleLoading(true, '正在加载数据...');

        try {
            // 模拟加载延迟
            await new Promise(resolve => setTimeout(resolve, 500));

            // TODO: 从 Tauri 后端加载实际数据
            // const config = await Utils.invoke('get_config');
            // AppState.config = config;

            // 模拟数据
            AppState.config = {
                theme: 'light',
                autoStart: true,
                minimizeToTray: true,
            };

            AppState.recentProjects = [
                { id: 1, name: 'claude-code', path: '/Users/dev/projects/claude-code', lastOpened: '2024-01-15T10:30:00Z' },
                { id: 2, name: 'my-app', path: '/Users/dev/projects/my-app', lastOpened: '2024-01-14T16:45:00Z' },
            ];

            console.log('初始数据加载完成');
        } catch (error) {
            console.error('加载初始数据失败:', error);
            Utils.showToast('数据加载失败', 'error');
        } finally {
            Utils.toggleLoading(false);
        }
    },
};

// ==========================================================================
// 启动应用
// ==========================================================================
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        App.init();
    });
} else {
    App.init();
}

// 导出全局对象供调试使用
window.ClaudeApp = { AppState, Utils, Dashboard, App };
