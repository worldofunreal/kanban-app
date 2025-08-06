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
  'status' : InviteStatus,
  'role' : Role,
  'created_at' : bigint,
  'target' : InviteTarget,
  'invited_by' : Principal,
  'invited_user' : Principal,
}
export type InviteStatus = { 'Accepted' : null } |
  { 'Declined' : null } |
  { 'Cancelled' : null } |
  { 'Pending' : null };
export type InviteTarget = { 'Team' : string } |
  { 'Project' : string };
export type Owner = { 'Team' : string } |
  { 'User' : Principal };
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
  'principal' : Principal,
  'role' : Role,
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
export type Result_2 = { 'Ok' : Principal } |
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
  'owner_principal' : Principal,
}
export interface TeamMember {
  'principal' : Principal,
  'role' : Role,
  'joined_at' : bigint,
}
export interface TeamUpdate {
  'is_public' : [] | [boolean],
  'name' : [] | [string],
  'description' : [] | [string],
}
export interface ThemePreferences { 'color' : string, 'dark_mode' : boolean }
export interface User {
  'updated_at' : bigint,
  'principal' : Principal,
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
  'cancel_invite' : ActorMethod<[string], Result>,
  'create_project' : ActorMethod<[string, string, Owner], Result_1>,
  'create_team' : ActorMethod<[string, string, boolean], Result_1>,
  'create_user' : ActorMethod<[UserProfile], Result_2>,
  'decline_invite' : ActorMethod<[string], Result>,
  'delete_project' : ActorMethod<[string], Result>,
  'delete_team' : ActorMethod<[string], Result>,
  'get_invites' : ActorMethod<[Principal], Array<Invite>>,
  'get_pending_invites' : ActorMethod<[Principal], Array<Invite>>,
  'get_project' : ActorMethod<[string], [] | [Project]>,
  'get_public_teams' : ActorMethod<[], Array<Team>>,
  'get_team' : ActorMethod<[string], [] | [Team]>,
  'get_team_projects' : ActorMethod<[string], Array<Project>>,
  'get_user' : ActorMethod<[Principal], [] | [User]>,
  'get_user_projects' : ActorMethod<[Principal], Array<Project>>,
  'get_user_teams' : ActorMethod<[Principal], Array<Team>>,
  'get_users' : ActorMethod<[], Array<User>>,
  'health_check' : ActorMethod<[], string>,
  'invite_user' : ActorMethod<[InviteTarget, Role, Principal], Result_1>,
  'remove_member' : ActorMethod<[InviteTarget, Principal], Result>,
  'transfer_ownership' : ActorMethod<[string, Owner], Result>,
  'update_profile' : ActorMethod<[Principal, UserProfileUpdate], Result>,
  'update_project' : ActorMethod<[string, ProjectUpdate], Result>,
  'update_team' : ActorMethod<[string, TeamUpdate], Result>,
  'update_theme_preferences' : ActorMethod<[ThemePreferences], Result>,
  'update_username' : ActorMethod<[Principal, string], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
