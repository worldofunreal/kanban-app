<template>
  <Card>
    <CardHeader>
      <CardTitle>Theme Settings</CardTitle>
      <CardDescription>Customize the appearance of your dashboard</CardDescription>
    </CardHeader>
    <CardContent class="space-y-6">
      <!-- Theme Style -->
      <div class="space-y-2">
        <Label>Theme Style</Label>
        <div class="grid grid-cols-2 gap-4">
          <div
            v-for="(theme, key) in themeStore.themes"
            :key="key"
            class="relative cursor-pointer"
            @click="themeStore.setTheme(key)"
          >
            <Card
              :class="cn(
                'transition-all hover:shadow-md',
                themeStore.currentTheme === key ? 'ring-2 ring-primary' : ''
              )"
            >
              <CardContent class="p-4">
                <div class="space-y-2">
                  <div class="flex items-center space-x-2">
                    <div class="w-3 h-3 rounded-full bg-primary"></div>
                    <div class="w-3 h-3 rounded-full bg-muted-foreground/20"></div>
                    <div class="w-3 h-3 rounded-full bg-muted-foreground/20"></div>
                  </div>
                  <div class="space-y-1">
                    <div class="h-2 bg-muted rounded" :style="{ borderRadius: theme.radius }"></div>
                    <div class="h-2 bg-muted rounded w-3/4" :style="{ borderRadius: theme.radius }"></div>
                  </div>
                </div>
                <div class="mt-2">
                  <p class="text-sm font-medium">{{ theme.name }}</p>
                  <p class="text-xs text-muted-foreground">{{ theme.description }}</p>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      </div>

      <!-- Color Scheme -->
      <div class="space-y-2">
        <Label>Color Scheme</Label>
        <div class="grid grid-cols-4 gap-2">
          <button
            v-for="(color, key) in themeStore.colors"
            :key="key"
            class="relative w-12 h-12 rounded-lg border-2 transition-all hover:scale-105"
            :class="themeStore.currentColor === key ? 'border-primary' : 'border-border'"
            :style="{ backgroundColor: `hsl(${color.primary})` }"
            :title="color.name"
            @click="themeStore.setColor(key)"
          >
            <div
              v-if="themeStore.currentColor === key"
              class="absolute inset-0 flex items-center justify-center"
            >
              <svg class="w-4 h-4 text-primary-foreground" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
              </svg>
            </div>
          </button>
        </div>
      </div>

      <!-- Dark Mode Toggle -->
      <div class="flex items-center justify-between">
        <div class="space-y-0.5">
          <Label>Dark Mode</Label>
          <p class="text-sm text-muted-foreground">Switch between light and dark themes</p>
        </div>
        <Button
          variant="outline"
          size="icon"
          @click="themeStore.toggleDarkMode"
        >
          <svg
            v-if="!isDark"
            class="h-4 w-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
          </svg>
          <svg
            v-else
            class="h-4 w-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
          </svg>
        </Button>
      </div>
    </CardContent>
  </Card>
</template>

<script setup>
import { computed } from 'vue'
import { useThemeStore } from '../stores/theme'
import Card from './ui/Card.vue'
import CardHeader from './ui/CardHeader.vue'
import CardTitle from './ui/CardTitle.vue'
import CardDescription from './ui/CardDescription.vue'
import CardContent from './ui/CardContent.vue'
import Label from './ui/Label.vue'
import Button from './ui/Button.vue'
import { cn } from '../lib/utils'

const themeStore = useThemeStore()

const isDark = computed(() => {
  return document.documentElement.classList.contains('dark')
})
</script> 