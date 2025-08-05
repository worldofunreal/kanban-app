<template>
  <div class="relative">
    <slot />
  </div>
</template>

<script setup>
import { provide, ref, onMounted, onUnmounted } from 'vue'

const isOpen = ref(false)

const open = () => {
  isOpen.value = true
}

const close = () => {
  isOpen.value = false
}

const toggle = () => {
  isOpen.value = !isOpen.value
}

// Ensure dropdown is closed when component unmounts
onUnmounted(() => {
  isOpen.value = false
})

// Close dropdown when clicking outside
const handleClickOutside = (event) => {
  if (isOpen.value) {
    const dropdownElement = event.target.closest('.dropdown-menu')
    const triggerElement = event.target.closest('[data-dropdown-trigger]')
    
    if (!dropdownElement && !triggerElement) {
      close()
    }
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  isOpen.value = false
})

provide('dropdown', {
  isOpen,
  open,
  close,
  toggle
})
</script> 