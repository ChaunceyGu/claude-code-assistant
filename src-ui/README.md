# Claude Code Assistant - Frontend

The frontend for the Claude Code Desktop Assistant built with vanilla JavaScript and Tauri.

## Structure

```
src-ui/
├── index.html          # Main HTML entry point
├── styles.css          # Global styles and CSS variables
├── main.js             # Application entry and utilities
├── README.md           # This file
└── components/         # UI Components
    ├── index.js        # Component exports
    ├── Card.js         # Card component
    ├── Dashboard.js    # Dashboard grid component
    ├── Modal.js        # Modal dialog component
    └── Toast.js        # Toast notification component
```

## Components

### Card
Reusable card component for the dashboard grid.

```javascript
import { Card } from './components/index.js';

const card = new Card({
    id: 'projects',
    icon: '📂',
    title: 'Recent Projects',
    description: 'Quickly access recent project directories',
    color: '#3B82F6',
    onClick: (cardId) => console.log(`Clicked ${cardId}`)
});

card.mount(document.getElementById('dashboard'));
```

### Dashboard
Manages the grid of cards with animations.

```javascript
import { Dashboard } from './components/index.js';

const dashboard = new Dashboard('dashboard-container');
dashboard.init(cardsConfig); // Array of card configurations
```

### Toast
Notification system for user feedback.

```javascript
import { Toast } from './components/index.js';

const toast = new Toast();
toast.success('Project added successfully!');
toast.error('Failed to load configuration');
toast.warning('Please check your settings');
toast.info('New update available');
```

### Modal
Dialog component for user interactions.

```javascript
import { Modal } from './components/index.js';

const modal = new Modal({
    title: 'Confirm Delete',
    content: 'Are you sure you want to delete this project?',
    buttons: [
        { text: 'Cancel', class: 'btn-secondary', onClick: () => modal.close() },
        { text: 'Delete', class: 'btn-danger', onClick: () => { /* delete logic */ } }
    ],
    onClose: () => console.log('Modal closed')
});

modal.open();
```

## Utilities

### Utils
Global utility functions available in `window.Utils`:

```javascript
// Toggle loading overlay
Utils.toggleLoading(true, 'Loading projects...');
Utils.toggleLoading(false);

// Show toast notifications
Utils.showToast('Operation successful', 'success', 3000);

// Invoke Tauri commands
const config = await Utils.invoke('get_app_config');

// Format dates
const formatted = Utils.formatDate('2024-01-15T10:30:00Z');

// Truncate text
const short = Utils.truncate('Very long text here', 20);

// Generate unique IDs
const id = Utils.generateId();

// Debounce function calls
const debounced = Utils.debounce((value) => console.log(value), 300);
```

### AppState
Global state management in `window.AppState`:

```javascript
// Access state
console.log(AppState.currentView);
console.log(AppState.config);

// Update state (reactive updates coming soon)
AppState.recentProjects = await Utils.invoke('get_recent_projects');
```

## Styling

### CSS Variables
All styles use CSS variables for theming. Override in `:root`:

```css
:root {
    /* Primary colors */
    --primary: #D97757;
    --primary-light: #E8956F;
    --primary-dark: #B55E3D;

    /* Background colors */
    --bg-primary: #FAF9F6;
    --bg-secondary: #FFFFFF;

    /* Text colors */
    --text-primary: #2D2D2D;
    --text-secondary: #666666;

    /* Functional colors */
    --success: #10A37F;
    --warning: #F59E0B;
    --error: #EF4444;
}
```

### Dark Mode
To enable dark mode, add `.dark` class to `body`:

```javascript
document.body.classList.add('dark');
```

Define dark mode variables:

```css
.dark {
    --bg-primary: #1A1A1A;
    --bg-secondary: #252525;
    --text-primary: #E5E5E5;
    --text-secondary: #999999;
}
```

## Tauri Integration

The frontend communicates with Rust backend via Tauri's invoke API:

```javascript
// Call Rust command
const result = await window.__TAURI__.invoke('command_name', {
    arg1: 'value1',
    arg2: 42
});

// Using Utils wrapper
const result = await Utils.invoke('command_name', { arg1: 'value1' });
```

## Development

### Running in Browser

For quick UI development without Tauri:

1. Start a local server:
   ```bash
   cd src-ui
   python -m http.server 8080
   # or
   npx serve .
   ```

2. Open `http://localhost:8080`

Note: Tauri-specific features (invoke, fs, etc.) will not work in browser mode. The app includes fallbacks for development.

### Running with Tauri

```bash
# Development mode
cargo tauri dev

# Build for production
cargo tauri build
```

## License

MIT License - See LICENSE file for details.
