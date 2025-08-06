import { createActor } from '../../../declarations/backend';
import { HttpAgent } from '@dfinity/agent';

// Global variable to store the current identity
let currentIdentity = null;

// Set the current identity (called from auth store)
export function setCurrentIdentity(identity) {
  currentIdentity = identity;
}

// Clear the current identity (called from auth store on logout)
export function clearCurrentIdentity() {
  currentIdentity = null;
}

// Create an authenticated backend actor
async function createAuthenticatedBackendActor() {
  if (!currentIdentity) {
    throw new Error('No authenticated identity available');
  }

  // Create agent with authenticated identity
  const agent = new HttpAgent({
    identity: currentIdentity,
    host: 'http://127.0.0.1:4943'
  });

  // Fetch root key for local development
  await agent.fetchRootKey();

  // Create actor with authenticated identity
  return createActor(process.env.CANISTER_ID_BACKEND, {
    agent
  });
}

// Unified canister service with all methods
class CanisterService {
  // User management
  async createUser(profile) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.create_user(profile);
  }

  async getCurrentUser() {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.get_current_user();
  }

  async isUserRegistered() {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.is_user_registered();
  }

  async updateProfile(principal, updates) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.update_profile(principal, updates);
  }

  async updateUsername(principal, username) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.update_username(principal, username);
  }

  async updateThemePreferences(themePreferences) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.update_theme_preferences(themePreferences);
  }

  async getUser(userId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.get_user(userId);
  }

  // Team management
  async createTeam(name, description, isPublic) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.create_team(name, description, isPublic);
  }

  async getTeam(teamId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.get_team(teamId);
  }

  async updateTeam(teamId, updates) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.update_team(teamId, updates);
  }

  async deleteTeam(teamId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.delete_team(teamId);
  }

  async getUserTeams(userId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.get_user_teams(userId);
  }

  async getPublicTeams() {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.get_public_teams();
  }

  // Project management
  async createProject(name, description, owner) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.create_project(name, description, owner);
  }

  async getProject(projectId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.get_project(projectId);
  }

  async updateProject(projectId, updates) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.update_project(projectId, updates);
  }

  async deleteProject(projectId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.delete_project(projectId);
  }

  async transferOwnership(projectId, newOwner) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.transfer_ownership(projectId, newOwner);
  }

  async getUserProjects(userId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.get_user_projects(userId);
  }

  async getTeamProjects(teamId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.get_team_projects(teamId);
  }

  // Invitation management
  async inviteUser(target, role, expiresAt) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.invite_user(target, role, expiresAt);
  }

  async acceptInvite(inviteId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.accept_invite(inviteId);
  }

  async declineInvite(inviteId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.decline_invite(inviteId);
  }

  async removeMember(target, userId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.remove_member(target, userId);
  }

  async getInvites(userId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.get_invites(userId);
  }

  async getPendingInvites(userId) {
    const backendActor = await createAuthenticatedBackendActor();
    return await backendActor.get_pending_invites(userId);
  }
}

// Export singleton instance
export default new CanisterService(); 