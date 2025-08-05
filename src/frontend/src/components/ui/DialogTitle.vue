<template>
  <h2
    :class="cn(
      'text-lg font-semibold leading-none tracking-tight',
      $attrs.class
    )"
    v-bind="$attrs"
  >
    <slot />
  </h2>
</template>

<script setup>
import { cn } from '@/lib/utils'
</script> 