<script setup>
import { ref, onMounted, watch, computed } from 'vue'
import { useAuthStore } from './stores/auth'
import LoginForm from './components/user/LoginForm.vue'

const authStore = useAuthStore()

const userMenuOpen = ref(false)
const mobileMenuOpen = ref(false)
const globalError = ref(null)
const isInitializing = ref(true)

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
    userMenuOpen.value = false
    mobileMenuOpen.value = false
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
  <div id="app" class="min-h-screen bg-gray-50">
    <!-- Loading Screen -->
    <div v-if="isLoading" class="fixed inset-0 bg-white flex items-center justify-center z-50">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600 mx-auto mb-4"></div>
        <p class="text-gray-600">Initializing...</p>
      </div>
    </div>

    <!-- Login Screen -->
    <LoginForm v-else-if="!isAuthenticated" />

    <!-- Main App -->
    <div v-else>
      <!-- Navigation -->
      <nav class="bg-white shadow-sm border-b border-gray-200">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div class="flex justify-between h-16">
            <div class="flex">
              <!-- Logo -->
              <div class="flex-shrink-0 flex items-center">
                <router-link to="/" class="text-xl font-bold text-primary-600">
                  Kanban App
                </router-link>
              </div>
              
              <!-- Navigation Links -->
              <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                <router-link
                  to="/"
                  class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                  active-class="border-primary-500 text-gray-900"
                >
                  Dashboard
                </router-link>
                <router-link
                  to="/teams"
                  class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                  active-class="border-primary-500 text-gray-900"
                >
                  Teams
                </router-link>
                <router-link
                  to="/projects"
                  class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                  active-class="border-primary-500 text-gray-900"
                >
                  Projects
                </router-link>
                <router-link
                  to="/profile"
                  class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
                  active-class="border-primary-500 text-gray-900"
                >
                  Profile
                </router-link>
              </div>
            </div>
            
            <!-- User Menu -->
            <div class="hidden sm:ml-6 sm:flex sm:items-center">
              <div class="ml-3 relative">
                <div>
                  <button
                    class="max-w-xs bg-white flex items-center text-sm rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                    @click="userMenuOpen = !userMenuOpen"
                  >
                    <div class="h-8 w-8 rounded-full bg-primary-100 flex items-center justify-center">
                      <span class="text-primary-600 font-medium">
                        {{ currentUser?.username?.charAt(0) || 'U' }}
                      </span>
                    </div>
                    <span class="ml-2 text-gray-700">{{ currentUser?.username || 'User' }}</span>
                  </button>
                </div>
                
                <!-- User dropdown menu -->
                <div
                  v-if="userMenuOpen"
                  class="origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg py-1 bg-white ring-1 ring-black ring-opacity-5 focus:outline-none z-10"
                >
                  <router-link
                    to="/profile"
                    class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                    @click="userMenuOpen = false"
                  >
                    Your Profile
                  </router-link>
                  <button
                    class="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                    @click="logout"
                  >
                    Sign out
                  </button>
                </div>
              </div>
            </div>
            
            <!-- Mobile menu button -->
            <div class="flex items-center sm:hidden">
              <button
                class="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-primary-500"
                @click="mobileMenuOpen = !mobileMenuOpen"
              >
                <svg
                  class="h-6 w-6"
                  :class="{ 'hidden': mobileMenuOpen, 'block': !mobileMenuOpen }"
                  stroke="currentColor"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 6h16M4 12h16M4 18h16"
                  />
                </svg>
                <svg
                  class="h-6 w-6"
                  :class="{ 'block': mobileMenuOpen, 'hidden': !mobileMenuOpen }"
                  stroke="currentColor"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </button>
            </div>
          </div>
        </div>
        
        <!-- Mobile menu -->
        <div v-if="mobileMenuOpen" class="sm:hidden">
          <div class="pt-2 pb-3 space-y-1">
            <router-link
              to="/"
              class="border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
              active-class="bg-primary-50 border-primary-500 text-primary-700"
              @click="mobileMenuOpen = false"
            >
              Dashboard
            </router-link>
            <router-link
              to="/teams"
              class="border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
              active-class="bg-primary-50 border-primary-500 text-primary-700"
              @click="mobileMenuOpen = false"
            >
              Teams
            </router-link>
            <router-link
              to="/projects"
              class="border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
              active-class="bg-primary-50 border-primary-500 text-primary-700"
              @click="mobileMenuOpen = false"
            >
              Projects
            </router-link>
            <router-link
              to="/profile"
              class="border-transparent text-gray-500 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium"
              active-class="bg-primary-50 border-primary-500 text-primary-700"
              @click="mobileMenuOpen = false"
            >
              Profile
            </router-link>
          </div>
          <div class="pt-4 pb-3 border-t border-gray-200">
            <div class="flex items-center px-4">
              <div class="flex-shrink-0">
                <div class="h-10 w-10 rounded-full bg-primary-100 flex items-center justify-center">
                  <span class="text-primary-600 font-medium">
                    {{ currentUser?.username?.charAt(0) || 'U' }}
                  </span>
                </div>
              </div>
              <div class="ml-3">
                <div class="text-base font-medium text-gray-800">
                  {{ currentUser?.username || 'User' }}
                </div>
                <div class="text-sm font-medium text-gray-500">
                  {{ currentUser?.email || 'No email' }}
                </div>
              </div>
            </div>
            <div class="mt-3 space-y-1">
              <router-link
                to="/profile"
                class="block px-4 py-2 text-base font-medium text-gray-500 hover:text-gray-800 hover:bg-gray-100"
                @click="mobileMenuOpen = false"
              >
                Your Profile
              </router-link>
              <button
                class="block w-full text-left px-4 py-2 text-base font-medium text-gray-500 hover:text-gray-800 hover:bg-gray-100"
                @click="logout"
              >
                Sign out
              </button>
            </div>
          </div>
        </div>
      </nav>

      <!-- Main Content -->
      <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <router-view />
  </main>
    </div>

    <!-- Global Error Display -->
    <div
      v-if="globalError"
      class="fixed bottom-4 right-4 bg-red-500 text-white px-6 py-3 rounded-lg shadow-lg z-50"
    >
      <div class="flex items-center justify-between">
        <span>{{ globalError }}</span>
        <button
          class="ml-4 text-white hover:text-gray-200"
          @click="globalError = null"
        >
          <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
            <path
              fill-rule="evenodd"
              d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
              clip-rule="evenodd"
            />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Add any additional styles here */
</style>
