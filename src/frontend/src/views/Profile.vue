<template>
  <div class="profile-page">
    <UserProfile />
  </div>
</template>

<script setup>
import UserProfile from '@/components/user/UserProfile.vue'
</script>

<style scoped>
.profile-page {
  min-height: calc(100vh - 4rem);
}
</style> 