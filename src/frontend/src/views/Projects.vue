<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="bg-white shadow rounded-lg p-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">Projects</h1>
          <p class="text-gray-600 mt-1">
            Manage your projects and track progress.
          </p>
        </div>
        <button
          class="btn btn-primary"
          @click="showCreateModal = true"
        >
          Create Project
        </button>
      </div>
    </div>

    <!-- Filters -->
    <div class="bg-white shadow rounded-lg p-6">
      <div class="flex items-center space-x-4">
        <div class="flex-1">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search projects..."
            class="input"
          />
        </div>
        <select
          v-model="filterType"
          class="input w-48"
        >
          <option value="all">All Projects</option>
          <option value="personal">Personal Projects</option>
          <option value="team">Team Projects</option>
        </select>
      </div>
    </div>

    <!-- Projects Grid -->
    <div class="bg-white shadow rounded-lg">
      <div class="p-6">
        <div v-if="projectsStore.loading" class="space-y-4">
          <div v-for="i in 6" :key="i" class="loading h-32 rounded-lg"></div>
        </div>
        <div v-else-if="filteredProjects.length === 0" class="text-center py-12">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
          </svg>
          <h3 class="mt-2 text-sm font-medium text-gray-900">No projects found</h3>
          <p class="mt-1 text-sm text-gray-500">
            {{ filterType === 'all' ? 'Get started by creating your first project.' : 'No projects match your current filters.' }}
          </p>
          <div v-if="filterType === 'all'" class="mt-6">
            <button
              class="btn btn-primary"
              @click="showCreateModal = true"
            >
              Create Project
            </button>
          </div>
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div
            v-for="project in filteredProjects"
            :key="project.id"
            class="border border-gray-200 rounded-lg p-6 hover:shadow-lg transition-shadow"
          >
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <h3 class="text-lg font-medium text-gray-900">{{ project.name }}</h3>
                <p class="text-sm text-gray-600 mt-1">{{ project.description }}</p>
              </div>
              <div class="ml-4">
                <span
                  :class="getOwnerBadgeClass(project.owner)"
                  class="badge"
                >
                  {{ getOwnerType(project.owner) }}
                </span>
              </div>
            </div>
            
            <div class="mt-4 flex items-center justify-between">
              <div class="flex items-center space-x-2">
                <span class="text-sm text-gray-500">
                  {{ project.members.length }} member{{ project.members.length !== 1 ? 's' : '' }}
                </span>
                <span class="text-sm text-gray-500">•</span>
                <span class="text-sm text-gray-500">
                  {{ formatDate(project.created_at) }}
                </span>
              </div>
              <div class="flex items-center space-x-2">
                <router-link
                  :to="`/projects/${project.id}`"
                  class="text-primary-600 hover:text-primary-500 text-sm font-medium"
                >
                  View
                </router-link>
                <button
                  v-if="canManageProject(project)"
                  class="text-gray-600 hover:text-gray-800 text-sm font-medium"
                  @click="editProject(project)"
                >
                  Edit
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Project Modal -->
    <div
      v-if="showCreateModal"
      class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50"
      @click="showCreateModal = false"
    >
      <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white" @click.stop>
        <div class="mt-3">
          <h3 class="text-lg font-medium text-gray-900 mb-4">Create New Project</h3>
          <form @submit.prevent="createProject">
            <div class="space-y-4">
              <div>
                <label for="project-name" class="block text-sm font-medium text-gray-700">
                  Project Name
                </label>
                <input
                  id="project-name"
                  v-model="newProject.name"
                  type="text"
                  required
                  class="input mt-1"
                  placeholder="Enter project name"
                />
              </div>
              
              <div>
                <label for="project-description" class="block text-sm font-medium text-gray-700">
                  Description
                </label>
                <textarea
                  id="project-description"
                  v-model="newProject.description"
                  rows="3"
                  class="input mt-1"
                  placeholder="Describe your project"
                ></textarea>
              </div>
              
              <div>
                <label for="project-owner" class="block text-sm font-medium text-gray-700">
                  Owner
                </label>
                <select
                  id="project-owner"
                  v-model="newProject.ownerType"
                  class="input mt-1"
                >
                  <option value="user">Personal Project</option>
                  <option value="team">Team Project</option>
                </select>
              </div>
              
              <div v-if="newProject.ownerType === 'team'">
                <label for="project-team" class="block text-sm font-medium text-gray-700">
                  Select Team
                </label>
                <select
                  id="project-team"
                  v-model="newProject.teamId"
                  class="input mt-1"
                  required
                >
                  <option value="">Select a team</option>
                  <option
                    v-for="team in teamsStore.userTeams"
                    :key="team.id"
                    :value="team.id"
                  >
                    {{ team.name }}
                  </option>
                </select>
              </div>
            </div>
            
            <div class="mt-6 flex justify-end space-x-3">
              <button
                type="button"
                class="btn btn-secondary"
                @click="showCreateModal = false"
              >
                Cancel
              </button>
              <button
                type="submit"
                :disabled="projectsStore.loading"
                class="btn btn-primary"
              >
                <span v-if="projectsStore.loading">Creating...</span>
                <span v-else>Create Project</span>
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, reactive } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useProjectsStore } from '../stores/projects'
import { useTeamsStore } from '../stores/teams'

const authStore = useAuthStore()
const projectsStore = useProjectsStore()
const teamsStore = useTeamsStore()

const showCreateModal = ref(false)
const searchQuery = ref('')
const filterType = ref('all')

const newProject = reactive({
  name: '',
  description: '',
  ownerType: 'user',
  teamId: ''
})

const filteredProjects = computed(() => {
  let projects = projectsStore.userProjects

  // Apply filter
  if (filterType.value === 'personal') {
    projects = projects.filter(project => 
      project.owner && typeof project.owner === 'object' && 'User' in project.owner
    )
  } else if (filterType.value === 'team') {
    projects = projects.filter(project => 
      project.owner && typeof project.owner === 'object' && 'Team' in project.owner
    )
  }

  // Apply search
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    projects = projects.filter(project => 
      project.name.toLowerCase().includes(query) ||
      project.description.toLowerCase().includes(query)
    )
  }

  return projects
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

const canManageProject = (project) => {
  if (!authStore.currentUser) return false
  const member = project.members.find(m => m.user_id === authStore.currentUser.id)
  return member && (member.role === 'Owner' || member.role === 'Manager')
}

const formatDate = (timestamp) => {
  return new Date(timestamp / 1000000).toLocaleDateString()
}

const createProject = async () => {
  let owner
  if (newProject.ownerType === 'user') {
    owner = { User: authStore.currentUser.id }
  } else {
    owner = { Team: newProject.teamId }
  }

  const result = await projectsStore.createProject(
    newProject.name,
    newProject.description,
    owner
  )
  
  if (result.success) {
    showCreateModal.value = false
    // Reset form
    newProject.name = ''
    newProject.description = ''
    newProject.ownerType = 'user'
    newProject.teamId = ''
  }
}

const editProject = (project) => {
  // This would open an edit modal
  console.log('Edit project:', project)
}

onMounted(async () => {
  if (authStore.currentUser) {
    await Promise.all([
      projectsStore.fetchUserProjects(authStore.currentUser.id),
      teamsStore.fetchUserTeams(authStore.currentUser.id)
    ])
  }
})
</script> 