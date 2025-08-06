import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import canisterService from '../services/canisterService.js';

export const useTeamsStore = defineStore('teams', () => {
  // State
  const teams = ref([]);
  const currentTeam = ref(null);
  const loading = ref(false);
  const error = ref(null);

  // Getters
  const userTeams = computed(() => teams.value);
  const publicTeams = computed(() =>
    teams.value.filter(team => team.is_public)
  );

  // Actions
  const fetchUserTeams = async userId => {
    loading.value = true;
    error.value = null;

    try {
      const userTeamsData = await canisterService.getUserTeams(userId);
      teams.value = userTeamsData;
    } catch (err) {
      error.value = err.message;
      console.error('Error fetching user teams:', err);
    } finally {
      loading.value = false;
    }
  };

  const fetchPublicTeams = async () => {
    loading.value = true;
    error.value = null;

    try {
      const publicTeamsData = await canisterService.getPublicTeams();
      teams.value = publicTeamsData;
    } catch (err) {
      error.value = err.message;
      console.error('Error fetching public teams:', err);
    } finally {
      loading.value = false;
    }
  };

  const createTeam = async (name, description, isPublic) => {
    loading.value = true;
    error.value = null;

    try {
      const result = await canisterService.createTeam(name, description, isPublic);

      if ('Ok' in result) {
        // Refresh teams list
        await fetchUserTeams();
        return { success: true, teamId: result.Ok };
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

  const updateTeam = async (teamId, updates) => {
    loading.value = true;
    error.value = null;

    try {
      const result = await canisterService.updateTeam(teamId, updates);

      if ('Ok' in result) {
        // Update local state
        const teamIndex = teams.value.findIndex(team => team.id === teamId);
        if (teamIndex !== -1) {
          teams.value[teamIndex] = { ...teams.value[teamIndex], ...updates };
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

  const deleteTeam = async teamId => {
    loading.value = true;
    error.value = null;

    try {
      const result = await canisterService.deleteTeam(teamId);

      if ('Ok' in result) {
        // Remove from local state
        teams.value = teams.value.filter(team => team.id !== teamId);
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

  const fetchTeam = async teamId => {
    loading.value = true;
    error.value = null;

    try {
      const teamData = await canisterService.getTeam(teamId);
      if (teamData) {
        currentTeam.value = teamData;
        return { success: true, team: teamData };
      } else {
        error.value = 'Team not found';
        return { success: false, error: 'Team not found' };
      }
    } catch (err) {
      error.value = err.message;
      return { success: false, error: err.message };
    } finally {
      loading.value = false;
    }
  };

  const inviteUser = async (teamId, role, expiresAt = null) => {
    loading.value = true;
    error.value = null;

    try {
      const result = await canisterService.inviteUser(
        { Team: teamId },
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

  const removeMember = async (teamId, userId) => {
    loading.value = true;
    error.value = null;

    try {
      const result = await canisterService.removeMember({ Team: teamId }, userId);

      if ('Ok' in result) {
        // Refresh team data
        await fetchTeam(teamId);
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

  const clearCurrentTeam = () => {
    currentTeam.value = null;
  };

  return {
    // State
    teams,
    currentTeam,
    loading,
    error,

    // Getters
    userTeams,
    publicTeams,

    // Actions
    fetchUserTeams,
    fetchPublicTeams,
    createTeam,
    updateTeam,
    deleteTeam,
    fetchTeam,
    inviteUser,
    removeMember,
    clearError,
    clearCurrentTeam,
  };
});
