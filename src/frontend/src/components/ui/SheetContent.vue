<template>
  <div
    :class="cn(
      'flex h-full flex-col overflow-y-auto bg-background p-6',
      $attrs.class
    )"
    v-bind="$attrs"
  >
    <slot />
  </div>
</template>

<script setup>
import { cn } from '@/lib/utils'
</script> 