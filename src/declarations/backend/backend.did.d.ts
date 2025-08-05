import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Error = { 'InvalidInput' : string } |
  { 'NotFound' : null } |
  { 'InviteNotFound' : null } |
  { 'InviteExpired' : null } |
  { 'TeamNotFound' : null } |
  { 'Unauthorized' : null } |
  { 'AlreadyExists' : null } |
  { 'InsufficientPermissions' : null } |
  { 'InternalError' : string } |
  { 'ProjectNotFound' : null } |
  { 'UserNotFound' : null };
export interface Invite {
  'id' : string,
  'role' : Role,
  'created_at' : bigint,
  'target' : InviteTarget,
  'invited_by' : string,
  'expires_at' : [] | [bigint],
}
export type InviteTarget = { 'Team' : string } |
  { 'Project' : string };
export type Owner = { 'Team' : string } |
  { 'User' : string };
export interface Project {
  'id' : string,
  'updated_at' : bigint,
  'members' : Array<TeamMember>,
  'owner' : Owner,
  'name' : string,
  'description' : string,
  'created_at' : bigint,
}
export interface ProjectMember {
  'role' : Role,
  'user_id' : string,
  'joined_at' : bigint,
}
export interface ProjectUpdate {
  'name' : [] | [string],
  'description' : [] | [string],
}
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : string } |
  { 'Err' : Error };
export type Role = { 'Collaborator' : null } |
  { 'Owner' : null } |
  { 'Manager' : null };
export interface Team {
  'id' : string,
  'is_public' : boolean,
  'updated_at' : bigint,
  'members' : Array<TeamMember>,
  'name' : string,
  'description' : string,
  'created_at' : bigint,
  'owner_id' : string,
}
export interface TeamMember {
  'role' : Role,
  'user_id' : string,
  'joined_at' : bigint,
}
export interface TeamUpdate {
  'is_public' : [] | [boolean],
  'name' : [] | [string],
  'description' : [] | [string],
}
export interface ThemePreferences { 'color' : string, 'dark_mode' : boolean }
export interface User {
  'id' : string,
  'updated_at' : bigint,
  'created_at' : bigint,
  'profile' : UserProfile,
}
export interface UserProfile {
  'bio' : [] | [string],
  'username' : string,
  'avatar_url' : [] | [string],
  'name' : string,
  'theme_preferences' : [] | [ThemePreferences],
  'email' : [] | [string],
}
export interface UserProfileUpdate {
  'bio' : [] | [[] | [string]],
  'username' : [] | [string],
  'avatar_url' : [] | [[] | [string]],
  'name' : [] | [string],
  'theme_preferences' : [] | [[] | [ThemePreferences]],
  'email' : [] | [[] | [string]],
}
export interface _SERVICE {
  'accept_invite' : ActorMethod<[string], Result>,
  'create_project' : ActorMethod<[string, string, Owner], Result_1>,
  'create_team' : ActorMethod<[string, string, boolean], Result_1>,
  'create_user' : ActorMethod<[UserProfile], Result_1>,
  'decline_invite' : ActorMethod<[string], Result>,
  'delete_project' : ActorMethod<[string], Result>,
  'delete_team' : ActorMethod<[string], Result>,
  'get_current_user' : ActorMethod<[], [] | [User]>,
  'get_invites' : ActorMethod<[string], Array<Invite>>,
  'get_pending_invites' : ActorMethod<[string], Array<Invite>>,
  'get_project' : ActorMethod<[string], [] | [Project]>,
  'get_public_teams' : ActorMethod<[], Array<Team>>,
  'get_team' : ActorMethod<[string], [] | [Team]>,
  'get_team_projects' : ActorMethod<[string], Array<Project>>,
  'get_user' : ActorMethod<[string], [] | [User]>,
  'get_user_by_principal' : ActorMethod<[Principal], [] | [User]>,
  'get_user_projects' : ActorMethod<[string], Array<Project>>,
  'get_user_teams' : ActorMethod<[string], Array<Team>>,
  'get_users' : ActorMethod<[], Array<User>>,
  'health_check' : ActorMethod<[], string>,
  'invite_user' : ActorMethod<[InviteTarget, Role, [] | [bigint]], Result_1>,
  'is_user_registered' : ActorMethod<[], boolean>,
  'is_username_available' : ActorMethod<[string], boolean>,
  'remove_member' : ActorMethod<[InviteTarget, string], Result>,
  'transfer_ownership' : ActorMethod<[string, Owner], Result>,
  'update_profile' : ActorMethod<[string, UserProfileUpdate], Result>,
  'update_project' : ActorMethod<[string, ProjectUpdate], Result>,
  'update_team' : ActorMethod<[string, TeamUpdate], Result>,
  'update_theme_preferences' : ActorMethod<[ThemePreferences], Result>,
  'update_username' : ActorMethod<[string, string], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
