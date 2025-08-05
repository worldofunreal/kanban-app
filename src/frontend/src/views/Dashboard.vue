<template>
  <div class="space-y-6">
    <!-- Header -->
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold">
              Welcome back, {{ authStore.currentUser?.profile?.name || 'User' }}!
            </h1>
            <p class="text-muted-foreground mt-1">
              Here's what's happening with your teams and projects.
            </p>
          </div>
          <div class="flex space-x-3">
            <Button as-child>
              <router-link to="/teams">
                Create Team
              </router-link>
            </Button>
            <Button variant="outline" as-child>
              <router-link to="/projects">
                Create Project
              </router-link>
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Stats -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <Card>
        <CardContent class="p-6">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <div class="w-8 h-8 bg-primary/10 rounded-lg flex items-center justify-center">
                <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                </svg>
              </div>
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium text-muted-foreground">Total Teams</p>
              <p class="text-2xl font-semibold">{{ teamsStore.userTeams.length }}</p>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="p-6">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <div class="w-8 h-8 bg-green-500/10 rounded-lg flex items-center justify-center">
                <svg class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                </svg>
              </div>
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium text-muted-foreground">Total Projects</p>
              <p class="text-2xl font-semibold">{{ projectsStore.userProjects.length }}</p>
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="p-6">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <div class="w-8 h-8 bg-yellow-500/10 rounded-lg flex items-center justify-center">
                <svg class="w-5 h-5 text-yellow-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </div>
            </div>
            <div class="ml-4">
              <p class="text-sm font-medium text-muted-foreground">Recent Activity</p>
              <p class="text-2xl font-semibold">{{ recentActivityCount }}</p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- Recent Teams -->
    <Card>
      <CardHeader>
        <div class="flex items-center justify-between">
          <CardTitle>Your Teams</CardTitle>
          <Button variant="ghost" as-child>
            <router-link to="/teams">
              View all
            </router-link>
          </Button>
        </div>
      </CardHeader>
      <CardContent>
        <div v-if="teamsStore.loading" class="space-y-4">
          <div v-for="i in 3" :key="i" class="h-20 rounded-lg bg-muted animate-pulse"></div>
        </div>
        <div v-else-if="teamsStore.userTeams.length === 0" class="text-center py-8">
          <svg class="mx-auto h-12 w-12 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
          </svg>
          <h3 class="mt-2 text-sm font-medium">No teams yet</h3>
          <p class="mt-1 text-sm text-muted-foreground">Get started by creating your first team.</p>
          <div class="mt-6">
            <Button as-child>
              <router-link to="/teams">
                Create Team
              </router-link>
            </Button>
          </div>
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <Card
            v-for="team in teamsStore.userTeams.slice(0, 6)"
            :key="team.id"
            class="hover:shadow-md transition-shadow"
          >
            <CardContent class="p-4">
              <div class="flex items-center justify-between">
                <h4 class="text-lg font-medium">{{ team.name }}</h4>
                <Badge :variant="team.is_public ? 'default' : 'secondary'">
                  {{ team.is_public ? 'Public' : 'Private' }}
                </Badge>
              </div>
              <p class="text-sm text-muted-foreground mt-1">{{ team.description }}</p>
              <div class="mt-4 flex items-center justify-between">
                <span class="text-xs text-muted-foreground">
                  {{ team.members.length }} member{{ team.members.length !== 1 ? 's' : '' }}
                </span>
                <Button variant="ghost" size="sm" as-child>
                  <router-link :to="`/teams/${team.id}`">
                    View
                  </router-link>
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </CardContent>
    </Card>

    <!-- Recent Projects -->
    <Card>
      <CardHeader>
        <div class="flex items-center justify-between">
          <CardTitle>Your Projects</CardTitle>
          <Button variant="ghost" as-child>
            <router-link to="/projects">
              View all
            </router-link>
          </Button>
        </div>
      </CardHeader>
      <CardContent>
        <div v-if="projectsStore.loading" class="space-y-4">
          <div v-for="i in 3" :key="i" class="h-20 rounded-lg bg-muted animate-pulse"></div>
        </div>
        <div v-else-if="projectsStore.userProjects.length === 0" class="text-center py-8">
          <svg class="mx-auto h-12 w-12 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
          </svg>
          <h3 class="mt-2 text-sm font-medium">No projects yet</h3>
          <p class="mt-1 text-sm text-muted-foreground">Get started by creating your first project.</p>
          <div class="mt-6">
            <Button as-child>
              <router-link to="/projects">
                Create Project
              </router-link>
            </Button>
          </div>
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <Card
            v-for="project in projectsStore.userProjects.slice(0, 6)"
            :key="project.id"
            class="hover:shadow-md transition-shadow"
          >
            <CardContent class="p-4">
              <div class="flex items-center justify-between">
                <h4 class="text-lg font-medium">{{ project.name }}</h4>
                <Badge :variant="getOwnerBadgeVariant(project.owner)">
                  {{ getOwnerType(project.owner) }}
                </Badge>
              </div>
              <p class="text-sm text-muted-foreground mt-1">{{ project.description }}</p>
              <div class="mt-4 flex items-center justify-between">
                <span class="text-xs text-muted-foreground">
                  {{ project.members.length }} member{{ project.members.length !== 1 ? 's' : '' }}
                </span>
                <Button variant="ghost" size="sm" as-child>
                  <router-link :to="`/projects/${project.id}`">
                    View
                  </router-link>
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </CardContent>
    </Card>

    <!-- Theme Switcher -->
    <ThemeSwitcher />
  </div>
</template>

<script setup>
import { computed, onMounted } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useTeamsStore } from '../stores/teams'
import { useProjectsStore } from '../stores/projects'
import { useThemeStore } from '../stores/theme'
import Button from '../components/ui/Button.vue'
import Card from '../components/ui/Card.vue'
import CardHeader from '../components/ui/CardHeader.vue'
import CardTitle from '../components/ui/CardTitle.vue'
import CardContent from '../components/ui/CardContent.vue'
import Badge from '../components/ui/Badge.vue'
import ThemeSwitcher from '../components/ThemeSwitcher.vue'

const authStore = useAuthStore()
const teamsStore = useTeamsStore()
const projectsStore = useProjectsStore()
const themeStore = useThemeStore()

const recentActivityCount = computed(() => {
  // This would be calculated based on recent activity
  // For now, return a placeholder
  return 0
})

const getOwnerType = (owner) => {
  if (typeof owner === 'object') {
    if ('User' in owner) return 'Personal'
    if ('Team' in owner) return 'Team'
  }
  return 'Unknown'
}

const getOwnerBadgeVariant = (owner) => {
  if (typeof owner === 'object') {
    if ('User' in owner) return 'default'
    if ('Team' in owner) return 'secondary'
  }
  return 'outline'
}

onMounted(async () => {
  // Initialize theme
  themeStore.initTheme()
  
  if (authStore.currentUser) {
    await Promise.all([
      teamsStore.fetchUserTeams(authStore.currentUser.id),
      projectsStore.fetchUserProjects(authStore.currentUser.id)
    ])
  }
})
</script> 