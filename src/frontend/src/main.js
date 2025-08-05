import { createApp } from 'vue';
import { createPinia } from 'pinia';
import router from './router';
import App from './App.vue';
import './index.css';
import themeSyncService from './services/themeSync';
import { useThemeStore } from './stores/theme';
import { Buffer } from 'buffer';
window.Buffer = Buffer;

const app = createApp(App);

app.use(createPinia());
app.use(router);

// Initialize theme sync service
window.themeSyncService = themeSyncService;
themeSyncService.init();

// Initialize theme store
const themeStore = useThemeStore();
themeStore.initTheme();

app.mount('#app');
