<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
          Create your account
        </h2>
        <p class="mt-2 text-center text-sm text-gray-600">
          Join Kanban App and start collaborating
        </p>
      </div>
      
      <div class="mt-8 space-y-6">
        <form class="space-y-6" @submit.prevent="handleRegister">
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
        
        <!-- Link to login -->
        <div class="text-center">
          <router-link
            to="/login"
            class="text-primary-600 hover:text-primary-500 text-sm font-medium"
          >
            Already have an account? Sign in
          </router-link>
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
import { reactive } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const form = reactive({
  name: '',
  email: '',
  bio: '',
  avatar_url: null
})

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