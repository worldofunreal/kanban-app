<template>
  <div
    v-if="dropdown.isOpen"
    ref="dropdownRef"
    class="dropdown-menu"
    :class="cn(
      'absolute right-0 top-full mt-2 z-50 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md animate-in fade-in-0 zoom-in-95',
      $attrs.class
    )"
    v-bind="$attrs"
  >
    <slot />
  </div>
</template>

<script setup>
import { inject, ref } from 'vue'
import { cn } from '@/lib/utils'

const dropdown = inject('dropdown')
const dropdownRef = ref(null)
</script> 