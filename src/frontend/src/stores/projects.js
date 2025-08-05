import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { backend } from 'declarations/backend';

export const useProjectsStore = defineStore('projects', () => {
  // State
  const projects = ref([]);
  const currentProject = ref(null);
  const loading = ref(false);
  const error = ref(null);

  // Getters
  const userProjects = computed(() => projects.value);
  const teamProjects = computed(() =>
    projects.value.filter(
      project =>
        project.owner &&
        typeof project.owner === 'object' &&
        'Team' in project.owner
    )
  );

  // Actions
  const fetchUserProjects = async userId => {
    loading.value = true;
    error.value = null;

    try {
      const userProjectsData = await backend.get_user_projects(userId);
      projects.value = userProjectsData;
    } catch (err) {
      error.value = err.message;
      console.error('Error fetching user projects:', err);
    } finally {
      loading.value = false;
    }
  };

  const fetchTeamProjects = async teamId => {
    loading.value = true;
    error.value = null;

    try {
      const teamProjectsData = await backend.get_team_projects(teamId);
      projects.value = teamProjectsData;
    } catch (err) {
      error.value = err.message;
      console.error('Error fetching team projects:', err);
    } finally {
      loading.value = false;
    }
  };

  const createProject = async (name, description, owner) => {
    loading.value = true;
    error.value = null;

    try {
      const result = await backend.create_project(name, description, owner);

      if ('Ok' in result) {
        // Refresh projects list
        await fetchUserProjects();
        return { success: true, projectId: result.Ok };
      } else {
        error.value = result.Err;
        return { success: false, error: result.Err };
      }
    } catch (err) {
      error.value = err.message;
      return { success: false, error: err.message };
    } finally {
      loading.value = false;
    }
  };

  const updateProject = async (projectId, updates) => {
    loading.value = true;
    error.value = null;

    try {
      const result = await backend.update_project(projectId, updates);

      if ('Ok' in result) {
        // Update local state
        const projectIndex = projects.value.findIndex(
          project => project.id === projectId
        );
        if (projectIndex !== -1) {
          projects.value[projectIndex] = {
            ...projects.value[projectIndex],
            ...updates,
          };
        }
        return { success: true };
      } else {
        error.value = result.Err;
        return { success: false, error: result.Err };
      }
    } catch (err) {
      error.value = err.message;
      return { success: false, error: err.message };
    } finally {
      loading.value = false;
    }
  };

  const deleteProject = async projectId => {
    loading.value = true;
    error.value = null;

    try {
      const result = await backend.delete_project(projectId);

      if ('Ok' in result) {
        // Remove from local state
        projects.value = projects.value.filter(
          project => project.id !== projectId
        );
        return { success: true };
      } else {
        error.value = result.Err;
        return { success: false, error: result.Err };
      }
    } catch (err) {
      error.value = err.message;
      return { success: false, error: err.message };
    } finally {
      loading.value = false;
    }
  };

  const fetchProject = async projectId => {
    loading.value = true;
    error.value = null;

    try {
      const projectData = await backend.get_project(projectId);
      if (projectData) {
        currentProject.value = projectData;
        return { success: true, project: projectData };
      } else {
        error.value = 'Project not found';
        return { success: false, error: 'Project not found' };
      }
    } catch (err) {
      error.value = err.message;
      return { success: false, error: err.message };
    } finally {
      loading.value = false;
    }
  };

  const transferOwnership = async (projectId, newOwner) => {
    loading.value = true;
    error.value = null;

    try {
      const result = await backend.transfer_ownership(projectId, newOwner);

      if ('Ok' in result) {
        // Refresh project data
        await fetchProject(projectId);
        return { success: true };
      } else {
        error.value = result.Err;
        return { success: false, error: result.Err };
      }
    } catch (err) {
      error.value = err.message;
      return { success: false, error: err.message };
    } finally {
      loading.value = false;
    }
  };

  const inviteUser = async (projectId, role, expiresAt = null) => {
    loading.value = true;
    error.value = null;

    try {
      const result = await backend.invite_user(
        { Project: projectId },
        role,
        expiresAt
      );

      if ('Ok' in result) {
        return { success: true, inviteId: result.Ok };
      } else {
        error.value = result.Err;
        return { success: false, error: result.Err };
      }
    } catch (err) {
      error.value = err.message;
      return { success: false, error: err.message };
    } finally {
      loading.value = false;
    }
  };

  const removeMember = async (projectId, userId) => {
    loading.value = true;
    error.value = null;

    try {
      const result = await backend.remove_member(
        { Project: projectId },
        userId
      );

      if ('Ok' in result) {
        // Refresh project data
        await fetchProject(projectId);
        return { success: true };
      } else {
        error.value = result.Err;
        return { success: false, error: result.Err };
      }
    } catch (err) {
      error.value = err.message;
      return { success: false, error: err.message };
    } finally {
      loading.value = false;
    }
  };

  const clearError = () => {
    error.value = null;
  };

  const clearCurrentProject = () => {
    currentProject.value = null;
  };

  return {
    // State
    projects,
    currentProject,
    loading,
    error,

    // Getters
    userProjects,
    teamProjects,

    // Actions
    fetchUserProjects,
    fetchTeamProjects,
    createProject,
    updateProject,
    deleteProject,
    fetchProject,
    transferOwnership,
    inviteUser,
    removeMember,
    clearError,
    clearCurrentProject,
  };
});
