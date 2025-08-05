<template>
  <button
    :class="cn(
      'relative flex w-full cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      $attrs.class
    )"
    v-bind="$attrs"
    @click="dropdown.close"
  >
    <slot />
  </button>
</template>

<script setup>
import { inject } from 'vue'
import { cn } from '@/lib/utils'

const dropdown = inject('dropdown')
</script> 