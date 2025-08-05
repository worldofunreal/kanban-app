<template>
  <div
    :class="cn(
      'grid gap-6',
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