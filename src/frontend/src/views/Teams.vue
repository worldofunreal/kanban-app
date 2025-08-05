<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="bg-white shadow rounded-lg p-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">Teams</h1>
          <p class="text-gray-600 mt-1">
            Manage your teams and collaborate with others.
          </p>
        </div>
        <button
          class="btn btn-primary"
          @click="showCreateModal = true"
        >
          Create Team
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
            placeholder="Search teams..."
            class="input"
          />
        </div>
        <select
          v-model="filterType"
          class="input w-48"
        >
          <option value="all">All Teams</option>
          <option value="my">My Teams</option>
          <option value="public">Public Teams</option>
        </select>
      </div>
    </div>

    <!-- Teams Grid -->
    <div class="bg-white shadow rounded-lg">
      <div class="p-6">
        <div v-if="teamsStore.loading" class="space-y-4">
          <div v-for="i in 6" :key="i" class="loading h-32 rounded-lg"></div>
        </div>
        <div v-else-if="filteredTeams.length === 0" class="text-center py-12">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
          </svg>
          <h3 class="mt-2 text-sm font-medium text-gray-900">No teams found</h3>
          <p class="mt-1 text-sm text-gray-500">
            {{ filterType === 'all' ? 'Get started by creating your first team.' : 'No teams match your current filters.' }}
          </p>
          <div v-if="filterType === 'all'" class="mt-6">
            <button
              class="btn btn-primary"
              @click="showCreateModal = true"
            >
              Create Team
            </button>
          </div>
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div
            v-for="team in filteredTeams"
            :key="team.id"
            class="border border-gray-200 rounded-lg p-6 hover:shadow-lg transition-shadow"
          >
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <h3 class="text-lg font-medium text-gray-900">{{ team.name }}</h3>
                <p class="text-sm text-gray-600 mt-1">{{ team.description }}</p>
              </div>
              <div class="ml-4">
                <span
                  :class="team.is_public ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'"
                  class="badge"
                >
                  {{ team.is_public ? 'Public' : 'Private' }}
                </span>
              </div>
            </div>
            
            <div class="mt-4 flex items-center justify-between">
              <div class="flex items-center space-x-2">
                <span class="text-sm text-gray-500">
                  {{ team.members.length }} member{{ team.members.length !== 1 ? 's' : '' }}
                </span>
                <span class="text-sm text-gray-500">•</span>
                <span class="text-sm text-gray-500">
                  {{ formatDate(team.created_at) }}
                </span>
              </div>
              <div class="flex items-center space-x-2">
                <router-link
                  :to="`/teams/${team.id}`"
                  class="text-primary-600 hover:text-primary-500 text-sm font-medium"
                >
                  View
                </router-link>
                <button
                  v-if="canManageTeam(team)"
                  class="text-gray-600 hover:text-gray-800 text-sm font-medium"
                  @click="editTeam(team)"
                >
                  Edit
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Team Modal -->
    <div
      v-if="showCreateModal"
      class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50"
      @click="showCreateModal = false"
    >
      <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white" @click.stop>
        <div class="mt-3">
          <h3 class="text-lg font-medium text-gray-900 mb-4">Create New Team</h3>
          <form @submit.prevent="createTeam">
            <div class="space-y-4">
              <div>
                <label for="team-name" class="block text-sm font-medium text-gray-700">
                  Team Name
                </label>
                <input
                  id="team-name"
                  v-model="newTeam.name"
                  type="text"
                  required
                  class="input mt-1"
                  placeholder="Enter team name"
                />
              </div>
              
              <div>
                <label for="team-description" class="block text-sm font-medium text-gray-700">
                  Description
                </label>
                <textarea
                  id="team-description"
                  v-model="newTeam.description"
                  rows="3"
                  class="input mt-1"
                  placeholder="Describe your team"
                ></textarea>
              </div>
              
              <div class="flex items-center">
                <input
                  id="team-public"
                  v-model="newTeam.isPublic"
                  type="checkbox"
                  class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                />
                <label for="team-public" class="ml-2 block text-sm text-gray-900">
                  Make this team public
                </label>
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
                :disabled="teamsStore.loading"
                class="btn btn-primary"
              >
                <span v-if="teamsStore.loading">Creating...</span>
                <span v-else>Create Team</span>
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
import { useTeamsStore } from '../stores/teams'

const authStore = useAuthStore()
const teamsStore = useTeamsStore()

const showCreateModal = ref(false)
const searchQuery = ref('')
const filterType = ref('all')

const newTeam = reactive({
  name: '',
  description: '',
  isPublic: false
})

const filteredTeams = computed(() => {
  let teams = teamsStore.userTeams

  // Apply filter
  if (filterType.value === 'my') {
    teams = teams.filter(team => team.owner_id === authStore.currentUser?.id)
  } else if (filterType.value === 'public') {
    teams = teams.filter(team => team.is_public)
  }

  // Apply search
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    teams = teams.filter(team => 
      team.name.toLowerCase().includes(query) ||
      team.description.toLowerCase().includes(query)
    )
  }

  return teams
})

const canManageTeam = (team) => {
  if (!authStore.currentUser) return false
  const member = team.members.find(m => m.user_id === authStore.currentUser.id)
  return member && (member.role === 'Owner' || member.role === 'Manager')
}

const formatDate = (timestamp) => {
  return new Date(timestamp / 1000000).toLocaleDateString()
}

const createTeam = async () => {
  const result = await teamsStore.createTeam(
    newTeam.name,
    newTeam.description,
    newTeam.isPublic
  )
  
  if (result.success) {
    showCreateModal.value = false
    // Reset form
    newTeam.name = ''
    newTeam.description = ''
    newTeam.isPublic = false
  }
}

const editTeam = (team) => {
  // This would open an edit modal
  console.log('Edit team:', team)
}

onMounted(async () => {
  if (authStore.currentUser) {
    await teamsStore.fetchUserTeams(authStore.currentUser.id)
  }
})
</script> 