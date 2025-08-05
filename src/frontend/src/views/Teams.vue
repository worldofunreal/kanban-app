<template>
  <div class="space-y-6">
    <!-- Header -->
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold">Teams</h1>
            <p class="text-muted-foreground mt-1">
              Manage your teams and collaborate with others.
            </p>
          </div>
          <Button @click="showCreateModal = true">
            Create Team
          </Button>
        </div>
      </CardContent>
    </Card>

    <!-- Filters -->
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center space-x-4">
          <div class="flex-1">
            <Input
              v-model="searchQuery"
              type="text"
              placeholder="Search teams..."
            />
          </div>
          <select
            v-model="filterType"
            class="flex h-10 w-48 rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
          >
            <option value="all">All Teams</option>
            <option value="my">My Teams</option>
            <option value="public">Public Teams</option>
          </select>
        </div>
      </CardContent>
    </Card>

    <!-- Teams Grid -->
    <Card>
      <CardContent class="p-6">
        <div v-if="teamsStore.loading" class="space-y-4">
          <div v-for="i in 6" :key="i" class="h-32 rounded-lg bg-muted animate-pulse"></div>
        </div>
        <div v-else-if="filteredTeams.length === 0" class="text-center py-12">
          <svg class="mx-auto h-12 w-12 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
          </svg>
          <h3 class="mt-2 text-sm font-medium">No teams found</h3>
          <p class="mt-1 text-sm text-muted-foreground">
            {{ filterType === 'all' ? 'Get started by creating your first team.' : 'No teams match your current filters.' }}
          </p>
          <div v-if="filterType === 'all'" class="mt-6">
            <Button @click="showCreateModal = true">
              Create Team
            </Button>
          </div>
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <Card
            v-for="team in filteredTeams"
            :key="team.id"
            class="hover:shadow-lg transition-shadow"
          >
            <CardContent class="p-6">
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <h3 class="text-lg font-medium">{{ team.name }}</h3>
                  <p class="text-sm text-muted-foreground mt-1">{{ team.description }}</p>
                </div>
                <div class="ml-4">
                  <Badge :variant="team.is_public ? 'default' : 'secondary'">
                    {{ team.is_public ? 'Public' : 'Private' }}
                  </Badge>
                </div>
              </div>
              
              <div class="mt-4 flex items-center justify-between">
                <div class="flex items-center space-x-2">
                  <span class="text-sm text-muted-foreground">
                    {{ team.members.length }} member{{ team.members.length !== 1 ? 's' : '' }}
                  </span>
                  <span class="text-sm text-muted-foreground">•</span>
                  <span class="text-sm text-muted-foreground">
                    {{ formatDate(team.created_at) }}
                  </span>
                </div>
                <div class="flex items-center space-x-2">
                  <Button variant="ghost" size="sm" as-child>
                    <router-link :to="`/teams/${team.id}`">
                      View
                    </router-link>
                  </Button>
                  <Button
                    v-if="canManageTeam(team)"
                    variant="ghost"
                    size="sm"
                    @click="editTeam(team)"
                  >
                    Edit
                  </Button>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </CardContent>
    </Card>

    <!-- Create Team Modal -->
    <Dialog v-model:open="showCreateModal">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create New Team</DialogTitle>
        </DialogHeader>
        <form class="space-y-4" @submit.prevent="createTeam">
          <div class="space-y-2">
            <Label for="team-name">Team Name</Label>
            <Input
              id="team-name"
              v-model="newTeam.name"
              type="text"
              required
              placeholder="Enter team name"
            />
          </div>
          
          <div class="space-y-2">
            <Label for="team-description">Description</Label>
            <Textarea
              id="team-description"
              v-model="newTeam.description"
              rows="3"
              placeholder="Describe your team"
            />
          </div>
          
          <div class="flex items-center space-x-2">
            <Checkbox
              id="team-public"
              v-model="newTeam.isPublic"
            />
            <Label for="team-public">Make this team public</Label>
          </div>
          
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              @click="showCreateModal = false"
            >
              Cancel
            </Button>
            <Button
              type="submit"
              :disabled="teamsStore.loading"
            >
              <span v-if="teamsStore.loading">Creating...</span>
              <span v-else>Create Team</span>
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, reactive } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useTeamsStore } from '../stores/teams'
import Button from '../components/ui/Button.vue'
import Card from '../components/ui/Card.vue'

import CardContent from '../components/ui/CardContent.vue'
import Badge from '../components/ui/Badge.vue'
import Input from '../components/ui/Input.vue'
import Label from '../components/ui/Label.vue'
import Textarea from '../components/ui/Textarea.vue'
import Checkbox from '../components/ui/Checkbox.vue'
import Dialog from '../components/ui/Dialog.vue'
import DialogContent from '../components/ui/DialogContent.vue'
import DialogHeader from '../components/ui/DialogHeader.vue'
import DialogTitle from '../components/ui/DialogTitle.vue'
import DialogFooter from '../components/ui/DialogFooter.vue'

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