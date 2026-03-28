/**
 * Modal Component
 * 模态框组件，用于显示对话框和表单
 */
class Modal {
    constructor(options = {}) {
        this.title = options.title || '';
        this.content = options.content || '';
        this.buttons = options.buttons || [];
        this.onClose = options.onClose;
        this.onOpen = options.onOpen;
        this.closeOnOverlay = options.closeOnOverlay !== false;
        this.closeOnEscape = options.closeOnEscape !== false;
        this.size = options.size || 'medium'; // small, medium, large, full
        this.element = null;
        this.overlay = null;
        this.isOpen = false;
        this.keydownHandler = null;
    }

    /**
     * 渲染模态框 HTML
     * @returns {string} HTML 字符串
     */
    render() {
        const sizeClass = `modal-${this.size}`;
        const buttonsHtml = this.buttons.map((btn, index) => {
            const type = btn.type || 'secondary';
            const className = `modal-btn modal-btn-${type}`;
            return `<button type="button" class="${className}" data-index="${index}">
                ${btn.icon ? `<span class="btn-icon">${btn.icon}</span>` : ''}
                ${btn.text}
            </button>`;
        }).join('');

        return `
            <div class="modal-overlay ${sizeClass}" id="modal-${this.generateId()}">
                <div class="modal-container" role="dialog" aria-modal="true" aria-labelledby="modal-title">
                    <div class="modal-header">
                        <h2 class="modal-title" id="modal-title">${this.title}</h2>
                        <button type="button" class="modal-close" aria-label="关闭">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <line x1="18" y1="6" x2="6" y2="18"></line>
                                <line x1="6" y1="6" x2="18" y2="18"></line>
                            </svg>
                        </button>
                    </div>
                    <div class="modal-body">
                        ${this.content}
                    </div>
                    ${buttonsHtml ? `
                    <div class="modal-footer">
                        ${buttonsHtml}
                    </div>
                    ` : ''}
                </div>
            </div>
        `;
    }

    /**
     * 打开模态框
     */
    open() {
        if (this.isOpen) return;

        // 渲染模态框
        const wrapper = document.createElement('div');
        wrapper.innerHTML = this.render();
        this.overlay = wrapper.firstElementChild;
        document.body.appendChild(this.overlay);

        // 绑定事件
        this.bindEvents();

        // 添加动画类
        requestAnimationFrame(() => {
            this.overlay.classList.add('modal-open');
            this.overlay.querySelector('.modal-container').classList.add('modal-container-open');
        });

        // 禁止背景滚动
        document.body.style.overflow = 'hidden';

        this.isOpen = true;

        // 触发打开回调
        if (typeof this.onOpen === 'function') {
            this.onOpen(this);
        }

        // 触发自定义事件
        document.dispatchEvent(new CustomEvent('modal:open', { detail: { modal: this } }));
    }

    /**
     * 关闭模态框
     */
    close() {
        if (!this.isOpen) return;

        // 添加关闭动画
        this.overlay.classList.remove('modal-open');
        this.overlay.classList.add('modal-closing');
        const container = this.overlay.querySelector('.modal-container');
        container.classList.remove('modal-container-open');
        container.classList.add('modal-container-closing');

        // 动画结束后移除 DOM
        setTimeout(() => {
            this.destroy();
        }, 300);

        // 恢复背景滚动
        document.body.style.overflow = '';

        this.isOpen = false;

        // 触发关闭回调
        if (typeof this.onClose === 'function') {
            this.onClose(this);
        }

        // 触发自定义事件
        document.dispatchEvent(new CustomEvent('modal:close', { detail: { modal: this } }));
    }

    /**
     * 绑定事件
     */
    bindEvents() {
        // 关闭按钮
        const closeBtn = this.overlay.querySelector('.modal-close');
        if (closeBtn) {
            closeBtn.addEventListener('click', () => this.close());
        }

        // 点击遮罩关闭
        if (this.closeOnOverlay) {
            this.overlay.addEventListener('click', (e) => {
                if (e.target === this.overlay) {
                    this.close();
                }
            });
        }

        // 按钮点击
        const buttons = this.overlay.querySelectorAll('.modal-btn');
        buttons.forEach(btn => {
            btn.addEventListener('click', (e) => {
                const index = parseInt(e.currentTarget.dataset.index);
                const buttonConfig = this.buttons[index];
                if (buttonConfig && typeof buttonConfig.onClick === 'function') {
                    buttonConfig.onClick(this, buttonConfig);
                }
            });
        });

        // ESC 键关闭
        if (this.closeOnEscape) {
            this.keydownHandler = (e) => {
                if (e.key === 'Escape' && this.isOpen) {
                    this.close();
                }
            };
            document.addEventListener('keydown', this.keydownHandler);
        }
    }

    /**
     * 销毁模态框
     */
    destroy() {
        if (this.overlay && this.overlay.parentNode) {
            this.overlay.parentNode.removeChild(this.overlay);
        }
        
        if (this.keydownHandler) {
            document.removeEventListener('keydown', this.keydownHandler);
            this.keydownHandler = null;
        }

        this.overlay = null;
        this.element = null;
        this.isOpen = false;
    }

    /**
     * 设置内容
     * @param {string} content HTML 内容
     */
    setContent(content) {
        this.content = content;
        if (this.overlay) {
            const body = this.overlay.querySelector('.modal-body');
            if (body) {
                body.innerHTML = content;
            }
        }
    }

    /**
     * 设置标题
     * @param {string} title 标题
     */
    setTitle(title) {
        this.title = title;
        if (this.overlay) {
            const titleEl = this.overlay.querySelector('.modal-title');
            if (titleEl) {
                titleEl.textContent = title;
            }
        }
    }

    /**
     * 生成唯一 ID
     * @returns {string} 唯一 ID
     */
    generateId() {
        return Math.random().toString(36).substr(2, 9);
    }
}

export default Modal;
DASHBOARDEOF
