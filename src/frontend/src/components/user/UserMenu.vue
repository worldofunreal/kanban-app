<template>
  <div class="relative">
    <!-- Avatar Button -->
    <Button 
      variant="ghost" 
      class="w-full justify-start text-left p-0 h-auto"
      @click="isOpen = !isOpen"
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

    <!-- Dropdown Menu -->
    <div
      v-if="isOpen"
      class="absolute bottom-full left-0 mb-2 w-80 bg-background border rounded-lg shadow-xl z-50"
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
          @click="goToProfile"
        >
          <svg class="mr-3 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
          Your Profile
        </Button>
        
        <Button
          variant="ghost"
          class="w-full justify-start h-12 text-destructive hover:text-destructive"
          @click="handleLogout"
        >
          <svg class="mr-3 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
          </svg>
          Sign out
        </Button>
      </div>
    </div>

    <!-- Backdrop to close menu -->
    <div
      v-if="isOpen"
      class="fixed inset-0 z-40"
      @click="isOpen = false"
    ></div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import Button from '../ui/Button.vue'
import Avatar from '../ui/Avatar.vue'
import AvatarFallback from '../ui/AvatarFallback.vue'

const router = useRouter()
const authStore = useAuthStore()

const isOpen = ref(false)

const currentUser = computed(() => authStore.getCurrentUser)

const goToProfile = () => {
  isOpen.value = false
  router.push('/profile')
}

const handleLogout = async () => {
  if (confirm('Are you sure you want to logout? You will need your seed phrase to recover your account.')) {
    try {
      await authStore.logout()
      isOpen.value = false
    } catch (error) {
      console.error('Logout failed:', error)
      alert('Logout failed. Please try again.')
    }
  }
}
</script> 