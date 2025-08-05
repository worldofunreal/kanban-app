<template>
  <button
    data-dropdown-trigger
    :class="cn(
      'inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none disabled:opacity-50',
      $attrs.class
    )"
    v-bind="$attrs"
    @click="dropdown.toggle"
  >
    <slot />
  </button>
</template>

<script setup>
import { inject } from 'vue'
import { cn } from '@/lib/utils'

const dropdown = inject('dropdown')
</script> 