/**
 * Card Component
 * 仪表盘卡片组件，用于显示功能入口
 */
class Card {
    constructor(config) {
        this.id = config.id;
        this.icon = config.icon;
        this.title = config.title;
        this.description = config.description;
        this.color = config.color || '#3B82F6';
        this.onClick = config.onClick;
        this.element = null;
    }

    /**
     * 渲染卡片 HTML
     * @returns {string} HTML 字符串
     */
    render() {
        return `
            <div class="dashboard-card" id="card-${this.id}" style="--card-color: ${this.color}">
                <div class="card-icon" style="background: ${this.hexToRgba(this.color, 0.1)}; color: ${this.color}">
                    <span class="icon">${this.icon}</span>
                </div>
                <div class="card-content">
                    <h3 class="card-title">${this.title}</h3>
                    <p class="card-description">${this.description}</p>
                </div>
                <div class="card-arrow">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <polyline points="9 18 15 12 9 6"></polyline>
                    </svg>
                </div>
            </div>
        `;
    }

    /**
     * 将卡片挂载到容器
     * @param {HTMLElement} container 容器元素
     */
    mount(container) {
        const wrapper = document.createElement('div');
        wrapper.innerHTML = this.render();
        this.element = wrapper.firstElementChild;
        container.appendChild(this.element);
        this.bindEvents(this.element);
    }

    /**
     * 绑定事件
     * @param {HTMLElement} element 卡片元素
     */
    bindEvents(element) {
        element.addEventListener('click', (e) => {
            e.preventDefault();
            this.handleClick();
        });

        // 添加悬停效果
        element.addEventListener('mouseenter', () => {
            element.style.transform = 'translateY(-4px)';
            element.style.boxShadow = `0 12px 24px -8px ${this.hexToRgba(this.color, 0.25)}`;
        });

        element.addEventListener('mouseleave', () => {
            element.style.transform = 'translateY(0)';
            element.style.boxShadow = '';
        });
    }

    /**
     * 处理点击事件
     */
    handleClick() {
        // 添加点击动画
        this.element.style.transform = 'scale(0.98)';
        setTimeout(() => {
            this.element.style.transform = '';
        }, 150);

        // 执行回调
        if (typeof this.onClick === 'function') {
            this.onClick(this.id, this);
        }
    }

    /**
     * 销毁卡片
     */
    destroy() {
        if (this.element && this.element.parentNode) {
            this.element.parentNode.removeChild(this.element);
        }
        this.element = null;
    }

    /**
     * 更新卡片配置
     * @param {Object} newConfig 新配置
     */
    update(newConfig) {
        Object.assign(this, newConfig);
        if (this.element) {
            const newElement = document.createElement('div');
            newElement.innerHTML = this.render();
            this.element.replaceWith(newElement.firstElementChild);
            this.element = newElement.firstElementChild;
            this.bindEvents(this.element);
        }
    }

    /**
     * 工具方法：将 hex 颜色转换为 rgba
     * @param {string} hex hex 颜色值
     * @param {number} alpha 透明度
     * @returns {string} rgba 颜色值
     */
    hexToRgba(hex, alpha) {
        const r = parseInt(hex.slice(1, 3), 16);
        const g = parseInt(hex.slice(3, 5), 16);
        const b = parseInt(hex.slice(5, 7), 16);
        return `rgba(${r}, ${g}, ${b}, ${alpha})`;
    }
}

export default Card;
