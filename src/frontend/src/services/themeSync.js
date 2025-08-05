import { useAuthStore } from '@/stores/auth';
import { useThemeStore } from '@/stores/theme';

class ThemeSyncService {
  constructor() {
    this.syncTimeout = null;
    this.isAuthenticated = false;
    this.authStore = null;
    this.themeStore = null;
  }

  init() {
    this.authStore = useAuthStore();
    this.themeStore = useThemeStore();
    this.isAuthenticated = this.authStore.isAuthenticated;
    
    // Watch for authentication changes
    this.authStore.$subscribe((mutation, state) => {
      if (state.isAuthenticated !== this.isAuthenticated) {
        this.isAuthenticated = state.isAuthenticated;
        if (this.isAuthenticated) {
          this.loadThemeFromBackend();
        }
      }
    });
  }

  // Load theme preferences from backend when user logs in
  async loadThemeFromBackend() {
    try {
      const currentUser = this.authStore.getCurrentUser;
      if (currentUser?.profile?.theme_preferences) {
        const prefs = currentUser.profile.theme_preferences;
        
        // Apply theme preferences
        this.themeStore.setTheme(prefs.theme);
        this.themeStore.setColor(prefs.color);
        
        // Apply dark mode
        const isDark = prefs.dark_mode;
        const currentIsDark = document.documentElement.classList.contains('dark');
        
        if (isDark !== currentIsDark) {
          this.themeStore.toggleDarkMode();
        }
      }
    } catch (error) {
      console.error('Failed to load theme preferences from backend:', error);
    }
  }

  // Schedule theme sync to backend (debounced)
  scheduleThemeSync() {
    if (!this.isAuthenticated) return;

    // Clear existing timeout
    if (this.syncTimeout) {
      clearTimeout(this.syncTimeout);
    }

    // Set new timeout (10 seconds)
    this.syncTimeout = setTimeout(() => {
      this.syncThemeToBackend();
    }, 10000);
  }

  // Sync current theme preferences to backend
  async syncThemeToBackend() {
    try {
      if (!this.isAuthenticated) return;

      const themePreferences = {
        theme: this.themeStore.currentTheme,
        color: this.themeStore.currentColor,
        dark_mode: document.documentElement.classList.contains('dark')
      };

      // Import backend canister
      const { backend } = await import('../../../declarations/backend');
      
      // Update theme preferences in backend
      const result = await backend.update_theme_preferences(themePreferences);
      
      if ('Ok' in result) {
        console.log('Theme preferences synced to backend');
      } else {
        console.error('Failed to sync theme preferences:', result.Err);
      }
    } catch (error) {
      console.error('Error syncing theme preferences:', error);
    }
  }

  // Force immediate sync (for logout, etc.)
  async forceSync() {
    if (this.syncTimeout) {
      clearTimeout(this.syncTimeout);
      this.syncTimeout = null;
    }
    await this.syncThemeToBackend();
  }

  // Cleanup
  destroy() {
    if (this.syncTimeout) {
      clearTimeout(this.syncTimeout);
      this.syncTimeout = null;
    }
  }
}

// Create singleton instance
const themeSyncService = new ThemeSyncService();

export default themeSyncService; 