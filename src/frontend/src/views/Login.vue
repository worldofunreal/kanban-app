<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
          Welcome to Kanban App
        </h2>
        <p class="mt-2 text-center text-sm text-gray-600">
          Sign in to your account or create a new one
        </p>
      </div>
      
      <div class="mt-8 space-y-6">
        <!-- Login Form -->
        <form v-if="!showRegister" class="space-y-6" @submit.prevent="handleLogin">
          <div>
            <label for="name" class="block text-sm font-medium text-gray-700">
              Full Name
            </label>
            <input
              id="name"
              v-model="form.name"
              type="text"
              required
              class="input mt-1"
              placeholder="Enter your full name"
            />
          </div>
          
          <div>
            <label for="email" class="block text-sm font-medium text-gray-700">
              Email Address
            </label>
            <input
              id="email"
              v-model="form.email"
              type="email"
              required
              class="input mt-1"
              placeholder="Enter your email"
            />
          </div>
          
          <div>
            <label for="bio" class="block text-sm font-medium text-gray-700">
              Bio (Optional)
            </label>
            <textarea
              id="bio"
              v-model="form.bio"
              rows="3"
              class="input mt-1"
              placeholder="Tell us about yourself"
            ></textarea>
          </div>
          
          <div>
            <button
              type="submit"
              :disabled="authStore.loading"
              class="btn btn-primary w-full"
            >
              <span v-if="authStore.loading" class="flex items-center justify-center">
                <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Creating Account...
              </span>
              <span v-else>Create Account</span>
            </button>
          </div>
        </form>
        
        <!-- Register Form -->
        <form v-else class="space-y-6" @submit.prevent="handleRegister">
          <div>
            <label for="reg-name" class="block text-sm font-medium text-gray-700">
              Full Name
            </label>
            <input
              id="reg-name"
              v-model="form.name"
              type="text"
              required
              class="input mt-1"
              placeholder="Enter your full name"
            />
          </div>
          
          <div>
            <label for="reg-email" class="block text-sm font-medium text-gray-700">
              Email Address
            </label>
            <input
              id="reg-email"
              v-model="form.email"
              type="email"
              required
              class="input mt-1"
              placeholder="Enter your email"
            />
          </div>
          
          <div>
            <label for="reg-bio" class="block text-sm font-medium text-gray-700">
              Bio (Optional)
            </label>
            <textarea
              id="reg-bio"
              v-model="form.bio"
              rows="3"
              class="input mt-1"
              placeholder="Tell us about yourself"
            ></textarea>
          </div>
          
          <div>
            <button
              type="submit"
              :disabled="authStore.isLoading"
              class="btn btn-primary w-full"
            >
              <span v-if="authStore.isLoading" class="flex items-center justify-center">
                <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Creating Account...
              </span>
              <span v-else>Create Account</span>
            </button>
          </div>
        </form>
        
        <!-- Toggle between login and register -->
        <div class="text-center">
          <button
            class="text-primary-600 hover:text-primary-500 text-sm font-medium"
            @click="showRegister = !showRegister"
          >
            {{ showRegister ? 'Already have an account? Sign in' : "Don't have an account? Create one" }}
          </button>
        </div>
        
        <!-- Error Message -->
        <div v-if="authStore.error" class="bg-red-50 border border-red-200 rounded-md p-4">
          <div class="flex">
            <div class="flex-shrink-0">
              <svg class="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
              </svg>
            </div>
            <div class="ml-3">
              <h3 class="text-sm font-medium text-red-800">
                {{ authStore.error }}
              </h3>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const showRegister = ref(false)

const form = reactive({
  name: '',
  email: '',
  bio: '',
  avatar_url: null
})

const handleLogin = async () => {
  const profile = {
    name: form.name,
    email: form.email,
    bio: form.bio || null,
    avatar_url: form.avatar_url
  }
  
  const result = await authStore.login(profile)
  if (result.success) {
    router.push('/')
  }
}

const handleRegister = async () => {
  const profile = {
    name: form.name,
    email: form.email,
    bio: form.bio || null,
    avatar_url: form.avatar_url
  }
  
  const result = await authStore.login(profile)
  if (result.success) {
    router.push('/')
  }
}
</script> 