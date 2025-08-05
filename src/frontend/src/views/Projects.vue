<template>
  <div class="space-y-6">
    <!-- Header -->
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold">Projects</h1>
            <p class="text-muted-foreground mt-1">
              Manage your projects and track progress.
            </p>
          </div>
          <Button @click="showCreateModal = true">
            Create Project
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
              placeholder="Search projects..."
            />
          </div>
          <select
            v-model="filterType"
            class="flex h-10 w-48 rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
          >
            <option value="all">All Projects</option>
            <option value="personal">Personal Projects</option>
            <option value="team">Team Projects</option>
          </select>
        </div>
      </CardContent>
    </Card>

    <!-- Projects Grid -->
    <Card>
      <CardContent class="p-6">
        <div v-if="projectsStore.loading" class="space-y-4">
          <div v-for="i in 6" :key="i" class="h-32 rounded-lg bg-muted animate-pulse"></div>
        </div>
        <div v-else-if="filteredProjects.length === 0" class="text-center py-12">
          <svg class="mx-auto h-12 w-12 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
          </svg>
          <h3 class="mt-2 text-sm font-medium">No projects found</h3>
          <p class="mt-1 text-sm text-muted-foreground">
            {{ filterType === 'all' ? 'Get started by creating your first project.' : 'No projects match your current filters.' }}
          </p>
          <div v-if="filterType === 'all'" class="mt-6">
            <Button @click="showCreateModal = true">
              Create Project
            </Button>
          </div>
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <Card
            v-for="project in filteredProjects"
            :key="project.id"
            class="hover:shadow-lg transition-shadow"
          >
            <CardContent class="p-6">
              <div class="flex items-start justify-between">
                <div class="flex-1">
                  <h3 class="text-lg font-medium">{{ project.name }}</h3>
                  <p class="text-sm text-muted-foreground mt-1">{{ project.description }}</p>
                </div>
                <div class="ml-4">
                  <Badge :variant="getOwnerBadgeVariant(project.owner)">
                    {{ getOwnerType(project.owner) }}
                  </Badge>
                </div>
              </div>
              
              <div class="mt-4 flex items-center justify-between">
                <div class="flex items-center space-x-2">
                  <span class="text-sm text-muted-foreground">
                    {{ project.members.length }} member{{ project.members.length !== 1 ? 's' : '' }}
                  </span>
                  <span class="text-sm text-muted-foreground">•</span>
                  <span class="text-sm text-muted-foreground">
                    {{ formatDate(project.created_at) }}
                  </span>
                </div>
                <div class="flex items-center space-x-2">
                  <Button variant="ghost" size="sm" as-child>
                    <router-link :to="`/projects/${project.id}`">
                      View
                    </router-link>
                  </Button>
                  <Button
                    v-if="canManageProject(project)"
                    variant="ghost"
                    size="sm"
                    @click="editProject(project)"
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

    <!-- Create Project Modal -->
    <Dialog v-model:open="showCreateModal">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create New Project</DialogTitle>
        </DialogHeader>
        <form class="space-y-4" @submit.prevent="createProject">
          <div class="space-y-2">
            <Label for="project-name">Project Name</Label>
            <Input
              id="project-name"
              v-model="newProject.name"
              type="text"
              required
              placeholder="Enter project name"
            />
          </div>
          
          <div class="space-y-2">
            <Label for="project-description">Description</Label>
            <Textarea
              id="project-description"
              v-model="newProject.description"
              rows="3"
              placeholder="Describe your project"
            />
          </div>
          
          <div class="space-y-2">
            <Label for="project-owner">Owner</Label>
            <select
              id="project-owner"
              v-model="newProject.ownerType"
              class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            >
              <option value="user">Personal Project</option>
              <option value="team">Team Project</option>
            </select>
          </div>
          
          <div v-if="newProject.ownerType === 'team'" class="space-y-2">
            <Label for="project-team">Select Team</Label>
            <select
              id="project-team"
              v-model="newProject.teamId"
              class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
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
              :disabled="projectsStore.loading"
            >
              <span v-if="projectsStore.loading">Creating...</span>
              <span v-else>Create Project</span>
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
import { useProjectsStore } from '../stores/projects'
import { useTeamsStore } from '../stores/teams'
import Button from '../components/ui/Button.vue'
import Card from '../components/ui/Card.vue'
import CardContent from '../components/ui/CardContent.vue'
import Badge from '../components/ui/Badge.vue'
import Input from '../components/ui/Input.vue'
import Label from '../components/ui/Label.vue'
import Textarea from '../components/ui/Textarea.vue'
import Dialog from '../components/ui/Dialog.vue'
import DialogContent from '../components/ui/DialogContent.vue'
import DialogHeader from '../components/ui/DialogHeader.vue'
import DialogTitle from '../components/ui/DialogTitle.vue'
import DialogFooter from '../components/ui/DialogFooter.vue'

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

const getOwnerBadgeVariant = (owner) => {
  if (typeof owner === 'object') {
    if ('User' in owner) return 'default'
    if ('Team' in owner) return 'secondary'
  }
  return 'outline'
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