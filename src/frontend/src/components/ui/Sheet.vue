<template>
  <div
    v-if="open"
    class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm"
    @click="$emit('update:open', false)"
  >
    <div
      :class="cn(
        'fixed inset-y-0 right-0 z-50 h-full w-3/4 border-l bg-background p-6 shadow-lg transition ease-in-out data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:slide-in-from-right data-[state=open]:slide-out-to-right sm:max-w-sm',
        $attrs.class
      )"
      @click.stop
    >
      <slot />
    </div>
  </div>
</template>

<script setup>
import { cn } from '@/lib/utils'

defineProps({
  open: {
    type: Boolean,
    default: false
  }
})

defineEmits(['update:open'])
</script> 