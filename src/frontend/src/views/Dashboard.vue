<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="bg-white shadow rounded-lg p-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">
            Welcome back, {{ authStore.currentUser?.profile?.name || 'User' }}!
          </h1>
          <p class="text-gray-600 mt-1">
            Here's what's happening with your teams and projects.
          </p>
        </div>
        <div class="flex space-x-3">
          <router-link
            to="/teams"
            class="btn btn-primary"
          >
            Create Team
          </router-link>
          <router-link
            to="/projects"
            class="btn btn-secondary"
          >
            Create Project
          </router-link>
        </div>
      </div>
    </div>

    <!-- Stats -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <div class="bg-white shadow rounded-lg p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-primary-100 rounded-lg flex items-center justify-center">
              <svg class="w-5 h-5 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500">Total Teams</p>
            <p class="text-2xl font-semibold text-gray-900">{{ teamsStore.userTeams.length }}</p>
          </div>
        </div>
      </div>

      <div class="bg-white shadow rounded-lg p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-green-100 rounded-lg flex items-center justify-center">
              <svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500">Total Projects</p>
            <p class="text-2xl font-semibold text-gray-900">{{ projectsStore.userProjects.length }}</p>
          </div>
        </div>
      </div>

      <div class="bg-white shadow rounded-lg p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="w-8 h-8 bg-yellow-100 rounded-lg flex items-center justify-center">
              <svg class="w-5 h-5 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-500">Recent Activity</p>
            <p class="text-2xl font-semibold text-gray-900">{{ recentActivityCount }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Teams -->
    <div class="bg-white shadow rounded-lg">
      <div class="px-6 py-4 border-b border-gray-200">
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-medium text-gray-900">Your Teams</h3>
          <router-link
            to="/teams"
            class="text-primary-600 hover:text-primary-500 text-sm font-medium"
          >
            View all
          </router-link>
        </div>
      </div>
      <div class="p-6">
        <div v-if="teamsStore.loading" class="space-y-4">
          <div v-for="i in 3" :key="i" class="loading h-20 rounded-lg"></div>
        </div>
        <div v-else-if="teamsStore.userTeams.length === 0" class="text-center py-8">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
          </svg>
          <h3 class="mt-2 text-sm font-medium text-gray-900">No teams yet</h3>
          <p class="mt-1 text-sm text-gray-500">Get started by creating your first team.</p>
          <div class="mt-6">
            <router-link
              to="/teams"
              class="btn btn-primary"
            >
              Create Team
            </router-link>
          </div>
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div
            v-for="team in teamsStore.userTeams.slice(0, 6)"
            :key="team.id"
            class="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
          >
            <div class="flex items-center justify-between">
              <h4 class="text-lg font-medium text-gray-900">{{ team.name }}</h4>
              <span
                :class="team.is_public ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'"
                class="badge"
              >
                {{ team.is_public ? 'Public' : 'Private' }}
              </span>
            </div>
            <p class="text-sm text-gray-600 mt-1">{{ team.description }}</p>
            <div class="mt-4 flex items-center justify-between">
              <span class="text-xs text-gray-500">
                {{ team.members.length }} member{{ team.members.length !== 1 ? 's' : '' }}
              </span>
              <router-link
                :to="`/teams/${team.id}`"
                class="text-primary-600 hover:text-primary-500 text-sm font-medium"
              >
                View
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Projects -->
    <div class="bg-white shadow rounded-lg">
      <div class="px-6 py-4 border-b border-gray-200">
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-medium text-gray-900">Your Projects</h3>
          <router-link
            to="/projects"
            class="text-primary-600 hover:text-primary-500 text-sm font-medium"
          >
            View all
          </router-link>
        </div>
      </div>
      <div class="p-6">
        <div v-if="projectsStore.loading" class="space-y-4">
          <div v-for="i in 3" :key="i" class="loading h-20 rounded-lg"></div>
        </div>
        <div v-else-if="projectsStore.userProjects.length === 0" class="text-center py-8">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
          </svg>
          <h3 class="mt-2 text-sm font-medium text-gray-900">No projects yet</h3>
          <p class="mt-1 text-sm text-gray-500">Get started by creating your first project.</p>
          <div class="mt-6">
            <router-link
              to="/projects"
              class="btn btn-primary"
            >
              Create Project
            </router-link>
          </div>
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div
            v-for="project in projectsStore.userProjects.slice(0, 6)"
            :key="project.id"
            class="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
          >
            <div class="flex items-center justify-between">
              <h4 class="text-lg font-medium text-gray-900">{{ project.name }}</h4>
              <span
                :class="getOwnerBadgeClass(project.owner)"
                class="badge"
              >
                {{ getOwnerType(project.owner) }}
              </span>
            </div>
            <p class="text-sm text-gray-600 mt-1">{{ project.description }}</p>
            <div class="mt-4 flex items-center justify-between">
              <span class="text-xs text-gray-500">
                {{ project.members.length }} member{{ project.members.length !== 1 ? 's' : '' }}
              </span>
              <router-link
                :to="`/projects/${project.id}`"
                class="text-primary-600 hover:text-primary-500 text-sm font-medium"
              >
                View
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, onMounted } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useTeamsStore } from '../stores/teams'
import { useProjectsStore } from '../stores/projects'

const authStore = useAuthStore()
const teamsStore = useTeamsStore()
const projectsStore = useProjectsStore()

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

const getOwnerBadgeClass = (owner) => {
  if (typeof owner === 'object') {
    if ('User' in owner) return 'bg-blue-100 text-blue-800'
    if ('Team' in owner) return 'bg-purple-100 text-purple-800'
  }
  return 'bg-gray-100 text-gray-800'
}

onMounted(async () => {
  if (authStore.currentUser) {
    await Promise.all([
      teamsStore.fetchUserTeams(authStore.currentUser.id),
      projectsStore.fetchUserProjects(authStore.currentUser.id)
    ])
  }
})
</script> 