import Card from './Card.js';

/**
 * Dashboard Component
 * 仪表盘组件，管理和渲染所有卡片
 */
class Dashboard {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.cards = [];
        this.cardInstances = [];
        this.gridElement = null;
    }

    /**
     * 初始化仪表盘
     * @param {Array} cardsConfig 卡片配置数组
     */
    init(cardsConfig) {
        this.cards = cardsConfig || [];
        this.createGridContainer();
        this.render();
        this.animateCards();
    }

    /**
     * 创建网格容器
     */
    createGridContainer() {
        this.gridElement = document.createElement('div');
        this.gridElement.className = 'dashboard-grid';
        this.container.appendChild(this.gridElement);
    }

    /**
     * 渲染所有卡片
     */
    render() {
        this.gridElement.innerHTML = '';
        this.cardInstances = [];

        this.cards.forEach((cardConfig, index) => {
            const card = new Card({
                ...cardConfig,
                onClick: (id, instance) => {
                    this.handleCardClick(id, instance);
                }
            });

            const cardWrapper = document.createElement('div');
            cardWrapper.style.opacity = '0';
            cardWrapper.style.transform = 'translateY(20px)';
            card.mount(cardWrapper);
            this.gridElement.appendChild(cardWrapper);

            this.cardInstances.push({
                id: cardConfig.id,
                instance: card,
                element: cardWrapper,
                index: index
            });

            setTimeout(() => {
                cardWrapper.style.transition = 'opacity 0.4s ease, transform 0.4s ease';
                cardWrapper.style.opacity = '1';
                cardWrapper.style.transform = 'translateY(0)';
            }, index * 80);
        });
    }

    addCard(cardConfig) {
        this.cards.push(cardConfig);
        
        const card = new Card({
            ...cardConfig,
            onClick: (id, instance) => {
                this.handleCardClick(id, instance);
            }
        });

        const cardWrapper = document.createElement('div');
        cardWrapper.style.opacity = '0';
        cardWrapper.style.transform = 'translateY(20px)';
        card.mount(cardWrapper);
        this.gridElement.appendChild(cardWrapper);

        const index = this.cardInstances.length;
        this.cardInstances.push({
            id: cardConfig.id,
            instance: card,
            element: cardWrapper,
            index: index
        });

        setTimeout(() => {
            cardWrapper.style.transition = 'opacity 0.4s ease, transform 0.4s ease';
            cardWrapper.style.opacity = '1';
            cardWrapper.style.transform = 'translateY(0)';
        }, 50);
    }

    removeCard(cardId) {
        const cardIndex = this.cardInstances.findIndex(c => c.id === cardId);
        if (cardIndex === -1) return;

        const cardData = this.cardInstances[cardIndex];
        
        cardData.element.style.transition = 'opacity 0.3s ease, transform 0.3s ease';
        cardData.element.style.opacity = '0';
        cardData.element.style.transform = 'scale(0.9)';

        setTimeout(() => {
            cardData.instance.destroy();
            this.cardInstances.splice(cardIndex, 1);
            
            const configIndex = this.cards.findIndex(c => c.id === cardId);
            if (configIndex !== -1) {
                this.cards.splice(configIndex, 1);
            }
        }, 300);
    }

    handleCardClick(cardId, cardInstance) {
        const event = new CustomEvent('dashboard:cardClick', {
            detail: { cardId, cardInstance, dashboard: this }
        });
        document.dispatchEvent(event);

        const cardConfig = this.cards.find(c => c.id === cardId);
        if (cardConfig && typeof cardConfig.onClick === 'function') {
            cardConfig.onClick(cardId, cardInstance);
        }
    }

    animateCards() {
        this.cardInstances.forEach((cardData, index) => {
            cardData.element.style.opacity = '0';
            cardData.element.style.transform = 'translateY(30px)';
            
            setTimeout(() => {
                cardData.element.style.transition = 'opacity 0.5s ease, transform 0.5s ease';
                cardData.element.style.opacity = '1';
                cardData.element.style.transform = 'translateY(0)';
            }, index * 100);
        });
    }

    getCard(cardId) {
        const cardData = this.cardInstances.find(c => c.id === cardId);
        return cardData ? cardData.instance : null;
    }

    getAllCards() {
        return this.cardInstances.map(c => c.instance);
    }

    destroy() {
        this.cardInstances.forEach(cardData => {
            cardData.instance.destroy();
        });
        this.cardInstances = [];
        this.cards = [];
        
        if (this.gridElement && this.gridElement.parentNode) {
            this.gridElement.parentNode.removeChild(this.gridElement);
        }
        this.gridElement = null;
    }
}

export default Dashboard;
