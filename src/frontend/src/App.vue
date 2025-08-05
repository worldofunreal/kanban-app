<script setup>
import { ref, onMounted, watch, computed } from 'vue'
import { useAuthStore } from './stores/auth'
import LoginForm from './components/user/LoginForm.vue'
import Button from './components/ui/Button.vue'
import Avatar from './components/ui/Avatar.vue'
import AvatarFallback from './components/ui/AvatarFallback.vue'
import Toast from './components/ui/Toast.vue'

const authStore = useAuthStore()

const globalError = ref(null)
const isInitializing = ref(true)
const showUserMenu = ref(false)

// Computed properties
const isAuthenticated = computed(() => authStore.isAuthenticated)
const currentUser = computed(() => authStore.getCurrentUser)
const isLoading = computed(() => authStore.isLoading || isInitializing.value)

// Watch for errors in stores and display them globally
watch(() => authStore.error, (error) => {
  if (error) {
    globalError.value = error
    authStore.clearError()
  }
})

const logout = async () => {
  try {
    await authStore.logout()
  } catch (error) {
    console.error('Logout failed:', error)
  }
}

onMounted(async () => {
  try {
    // Initialize authentication (creates guest account if needed)
    await authStore.initializeAuth()
  } catch (error) {
    console.error('Failed to initialize auth:', error)
    globalError.value = 'Failed to initialize application. Please refresh the page.'
  } finally {
    isInitializing.value = false
  }
})
</script>

<template>
  <div id="app" class="min-h-screen bg-background">
    <!-- Loading Screen -->
    <div v-if="isLoading" class="fixed inset-0 bg-background flex items-center justify-center z-50">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
        <p class="text-muted-foreground">Initializing...</p>
      </div>
    </div>

    <!-- Login Screen -->
    <LoginForm v-else-if="!isAuthenticated" />

    <!-- Main App -->
    <div v-else class="flex h-screen bg-background">
      <!-- Sidebar -->
      <div class="w-64 bg-background border-r flex flex-col">
        <!-- Logo -->
        <div class="p-4 border-b">
          <router-link to="/" class="flex items-center space-x-2">
            <span class="text-xl font-bold">Kanban App</span>
          </router-link>
        </div>
        
        <!-- Navigation -->
        <nav class="flex-1 p-4 space-y-2">
          <router-link
            to="/"
            class="flex items-center space-x-3 px-3 py-2 rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground"
            active-class="bg-primary text-primary-foreground"
          >
            <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h4a2 2 0 012 2v2H8V5z" />
            </svg>
            <span>Dashboard</span>
          </router-link>
          
          <router-link
            to="/teams"
            class="flex items-center space-x-3 px-3 py-2 rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground"
            active-class="bg-primary text-primary-foreground"
          >
            <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z" />
            </svg>
            <span>Teams</span>
          </router-link>
          
          <router-link
            to="/projects"
            class="flex items-center space-x-3 px-3 py-2 rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground"
            active-class="bg-primary text-primary-foreground"
          >
            <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            <span>Projects</span>
          </router-link>
        </nav>
        
        <!-- User Avatar -->
        <div class="p-4 border-t">
          <Button 
            variant="ghost" 
            class="w-full justify-start text-left"
            @click="showUserMenu = true"
          >
            <Avatar class="h-8 w-8 mr-3 flex-shrink-0">
              <AvatarFallback>
                {{ currentUser?.profile?.name?.charAt(0) || 'U' }}
              </AvatarFallback>
            </Avatar>
            <div class="flex flex-col items-start text-left min-w-0 flex-1">
              <span class="text-sm font-medium truncate">{{ currentUser?.profile?.name || 'User' }}</span>
              <span class="text-xs text-muted-foreground truncate">{{ currentUser?.profile?.email || 'No email' }}</span>
            </div>
          </Button>
        </div>
      </div>
      
      <!-- Main Content -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- Top Bar -->
        <div class="h-16 border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 flex items-center justify-between px-6">
          <div class="flex items-center space-x-4">
            <h1 class="text-lg font-semibold">
              {{ $route.name === 'Dashboard' ? 'Dashboard' : 
                 $route.name === 'Teams' ? 'Teams' : 
                 $route.name === 'Projects' ? 'Projects' : 
                 $route.name === 'Profile' ? 'Profile' : 'Kanban App' }}
            </h1>
          </div>
          
          <div class="flex items-center space-x-4">
            <Button>
              <svg class="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              Create
            </Button>
          </div>
        </div>
        
        <!-- Page Content -->
        <div class="flex-1 overflow-auto">
          <div class="p-6">
            <router-view />
          </div>
        </div>
      </div>
        

    </div>

    <!-- User Menu Modal -->
    <div
      v-if="showUserMenu"
      class="fixed inset-0 z-50"
      @click="showUserMenu = false"
    >
      <!-- Backdrop -->
      <div class="absolute inset-0 bg-black/20"></div>
      
      <!-- Menu positioned to bottom left -->
      <div class="absolute bottom-4 left-4">
        <div 
          class="bg-background border rounded-lg shadow-xl w-80 max-w-full"
          @click.stop
        >
        <!-- User Info Header -->
        <div class="p-4 border-b">
          <div class="flex items-center space-x-3">
            <Avatar class="h-12 w-12">
              <AvatarFallback class="text-lg">
                {{ currentUser?.profile?.name?.charAt(0) || 'U' }}
              </AvatarFallback>
            </Avatar>
            <div class="flex-1 min-w-0">
              <h3 class="text-sm font-medium truncate">
                {{ currentUser?.profile?.name || 'User' }}
              </h3>
              <p class="text-xs text-muted-foreground truncate">
                {{ currentUser?.profile?.email || 'No email' }}
              </p>
            </div>
          </div>
        </div>

        <!-- Menu Options -->
        <div class="p-2">
          <Button
            variant="ghost"
            class="w-full justify-start h-12"
            @click="() => { $router.push('/profile'); showUserMenu = false; }"
          >
            <svg class="mr-3 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
            Your Profile
          </Button>
          
          <Button
            variant="ghost"
            class="w-full justify-start h-12 text-destructive hover:text-destructive"
            @click="() => { logout(); showUserMenu = false; }"
          >
            <svg class="mr-3 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
            </svg>
            Sign out
          </Button>
        </div>
      </div>
    </div>
  </div>

    <!-- Global Error Display -->
    <div
      v-if="globalError"
      class="fixed bottom-4 right-4 z-50"
    >
      <Toast class="bg-destructive text-destructive-foreground">
        <div class="flex items-center justify-between">
          <span>{{ globalError }}</span>
          <Button
            variant="ghost"
            size="icon"
            class="h-6 w-6 text-destructive-foreground hover:text-destructive-foreground/80"
            @click="globalError = null"
          >
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path
                fill-rule="evenodd"
                d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                clip-rule="evenodd"
              />
            </svg>
          </Button>
        </div>
      </Toast>
    </div>
  </div>
</template>

<style scoped>
/* Add any additional styles here */
</style>
