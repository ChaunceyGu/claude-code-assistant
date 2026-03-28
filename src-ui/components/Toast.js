/**
 * Toast Component
 * Toast 提示组件类（增强版）- 用于显示临时通知消息
 */

class Toast {
    /**
     * 创建 Toast 实例
     * @param {string} containerId - Toast 容器 ID（默认为 'toast-container'）
     */
    constructor(containerId = 'toast-container') {
        this.containerId = containerId;
        this.container = document.getElementById(containerId);

        // 如果容器不存在，创建默认容器
        if (!this.container) {
            this.container = this.createContainer();
        }

        this.toasts = []; // Toast 实例数组
        this.maxToasts = 5; // 最大同时显示的 Toast 数量
        this.defaultDuration = 3000; // 默认显示时长（毫秒）
    }

    /**
     * 创建 Toast 容器
     * @returns {HTMLElement} 容器元素
     */
    createContainer() {
        const container = document.createElement('div');
        container.id = this.containerId;
        container.className = 'toast-container';
        container.setAttribute('aria-live', 'polite');
        container.setAttribute('aria-atomic', 'true');

        // 添加到 body
        document.body.appendChild(container);

        return container;
    }

    /**
     * 显示 Toast
     * @param {string} message - 显示的消息内容
     * @param {string} type - Toast 类型：'info' | 'success' | 'error' | 'warning'
     * @param {number} duration - 显示时长（毫秒），0 表示不自动关闭
     * @returns {string} Toast ID
     */
    show(message, type = 'info', duration = this.defaultDuration) {
        // 检查最大数量限制
        if (this.toasts.length >= this.maxToasts) {
            // 移除最旧的 Toast
            const oldestToast = this.toasts[0];
            this.removeToast(oldestToast.id);
        }

        // 生成唯一 ID
        const toastId = `toast-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

        // 创建 Toast 元素
        const toastElement = this.createToastElement(message, type, toastId);

        // 添加到容器
        this.container.appendChild(toastElement);

        // 触发动画
        requestAnimationFrame(() => {
            toastElement.classList.add('toast-visible');
        });

        // 保存 Toast 信息
        const toastInfo = {
            id: toastId,
            element: toastElement,
            type,
            message,
            timer: null
        };

        this.toasts.push(toastInfo);

        // 设置自动关闭定时器
        if (duration > 0) {
            toastInfo.timer = setTimeout(() => {
                this.removeToast(toastId);
            }, duration);
        }

        // 触发自定义事件
        this.dispatchEvent('toast:show', { toast: toastInfo });

        return toastId;
    }

    /**
     * 显示成功 Toast
     * @param {string} message - 消息内容
     * @param {number} duration - 显示时长
     * @returns {string} Toast ID
     */
    success(message, duration) {
        return this.show(message, 'success', duration);
    }

    /**
     * 显示错误 Toast
     * @param {string} message - 消息内容
     * @param {number} duration - 显示时长
     * @returns {string} Toast ID
     */
    error(message, duration) {
        return this.show(message, 'error', duration || 5000);
    }

    /**
     * 显示警告 Toast
     * @param {string} message - 消息内容
     * @param {number} duration - 显示时长
     * @returns {string} Toast ID
     */
    warning(message, duration) {
        return this.show(message, 'warning', duration);
    }

    /**
     * 显示信息 Toast
     * @param {string} message - 消息内容
     * @param {number} duration - 显示时长
     * @returns {string} Toast ID
     */
    info(message, duration) {
        return this.show(message, 'info', duration);
    }

    /**
     * 创建 Toast 元素
     * @param {string} message - 消息内容
     * @param {string} type - Toast 类型
     * @param {string} toastId - Toast ID
     * @returns {HTMLElement} Toast 元素
     */
    createToastElement(message, type, toastId) {
        const toast = document.createElement('div');
        toast.id = toastId;
        toast.className = `toast toast-${type}`;
        toast.setAttribute('role', 'alert');
        toast.setAttribute('aria-live', 'polite');

        // 图标映射
        const icons = {
            success: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>',
            error: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>',
            warning: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>',
            info: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>'
        };

        toast.innerHTML = `
            <div class="toast-icon">${icons[type] || icons.info}</div>
            <div class="toast-content">
                <div class="toast-message">${this.escapeHtml(message)}</div>
            </div>
            <button class="toast-close" aria-label="关闭">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
            </button>
        `;

        // 绑定关闭事件
        const closeBtn = toast.querySelector('.toast-close');
        if (closeBtn) {
            closeBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                this.removeToast(toastId);
            });
        }

        // 点击 Toast 本身不关闭，除非是链接
        toast.addEventListener('click', (e) => {
            if (e.target.tagName === 'A' || e.target.closest('a')) {
                this.removeToast(toastId);
            }
        });

        return toast;
    }

    /**
     * 移除 Toast
     * @param {string} toastId - Toast ID
     */
    removeToast(toastId) {
        const toastIndex = this.toasts.findIndex(t => t.id === toastId);
        if (toastIndex === -1) return;

        const toast = this.toasts[toastIndex];

        // 清除定时器
        if (toast.timer) {
            clearTimeout(toast.timer);
        }

        // 添加退出动画
        if (toast.element) {
            toast.element.classList.remove('toast-visible');
            toast.element.classList.add('toast-hiding');

            // 动画结束后移除DOM
            setTimeout(() => {
                if (toast.element && toast.element.parentNode) {
                    toast.element.parentNode.removeChild(toast.element);
                }
            }, 300);
        }

        // 从数组中移除
        this.toasts.splice(toastIndex, 1);

        // 触发自定义事件
        this.dispatchEvent('toast:remove', { toast });
    }

    /**
     * 清除所有 Toast
     */
    clearAll() {
        // 复制数组以避免在迭代时修改
        const toastsCopy = [...this.toasts];
        toastsCopy.forEach(toast => {
            this.removeToast(toast.id);
        });
    }

    /**
     * 更新配置
     * @param {Object} options - 新的配置选项
     */
    configure(options) {
        if (options.maxToasts !== undefined) {
            this.maxToasts = options.maxToasts;
        }
        if (options.defaultDuration !== undefined) {
            this.defaultDuration = options.defaultDuration;
        }
    }

    /**
     * 触发自定义事件
     * @param {string} eventName - 事件名称
     * @param {Object} detail - 事件详情
     */
    dispatchEvent(eventName, detail) {
        const event = new CustomEvent(eventName, { detail });
        document.dispatchEvent(event);
    }

    /**
     * HTML 转义
     * @param {string} text - 原始文本
     * @returns {string} 转义后的文本
     */
    escapeHtml(text) {
        if (!text) return '';
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    /**
     * 销毁 Toast 实例
     */
    destroy() {
        this.clearAll();
        if (this.container && this.container.parentNode) {
            this.container.parentNode.removeChild(this.container);
        }
        this.container = null;
        this.toasts = [];
    }
}

// 创建全局单例实例
let globalToastInstance = null;

/**
 * 获取全局 Toast 实例
 * @returns {Toast} Toast 实例
 */
Toast.getInstance = function() {
    if (!globalToastInstance) {
        globalToastInstance = new Toast();
    }
    return globalToastInstance;
};

/**
 * 显示 Toast 的静态方法
 * @param {string} message - 消息内容
 * @param {string} type - 类型
 * @param {number} duration - 时长
 * @returns {string} Toast ID
 */
Toast.show = function(message, type, duration) {
    return Toast.getInstance().show(message, type, duration);
};

export default Toast;
