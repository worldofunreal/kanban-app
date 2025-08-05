export const idlFactory = ({ IDL }) => {
  const Error = IDL.Variant({
    'InvalidInput' : IDL.Text,
    'NotFound' : IDL.Null,
    'InviteNotFound' : IDL.Null,
    'InviteExpired' : IDL.Null,
    'TeamNotFound' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'AlreadyExists' : IDL.Null,
    'InsufficientPermissions' : IDL.Null,
    'InternalError' : IDL.Text,
    'ProjectNotFound' : IDL.Null,
    'UserNotFound' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const Owner = IDL.Variant({ 'Team' : IDL.Text, 'User' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : Error });
  const ThemePreferences = IDL.Record({
    'color' : IDL.Text,
    'dark_mode' : IDL.Bool,
  });
  const UserProfile = IDL.Record({
    'bio' : IDL.Opt(IDL.Text),
    'username' : IDL.Text,
    'avatar_url' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
    'theme_preferences' : IDL.Opt(ThemePreferences),
    'email' : IDL.Opt(IDL.Text),
  });
  const User = IDL.Record({
    'id' : IDL.Text,
    'updated_at' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'profile' : UserProfile,
  });
  const Role = IDL.Variant({
    'Collaborator' : IDL.Null,
    'Owner' : IDL.Null,
    'Manager' : IDL.Null,
  });
  const InviteTarget = IDL.Variant({ 'Team' : IDL.Text, 'Project' : IDL.Text });
  const Invite = IDL.Record({
    'id' : IDL.Text,
    'role' : Role,
    'created_at' : IDL.Nat64,
    'target' : InviteTarget,
    'invited_by' : IDL.Text,
    'expires_at' : IDL.Opt(IDL.Nat64),
  });
  const TeamMember = IDL.Record({
    'role' : Role,
    'user_id' : IDL.Text,
    'joined_at' : IDL.Nat64,
  });
  const Project = IDL.Record({
    'id' : IDL.Text,
    'updated_at' : IDL.Nat64,
    'members' : IDL.Vec(TeamMember),
    'owner' : Owner,
    'name' : IDL.Text,
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
  });
  const Team = IDL.Record({
    'id' : IDL.Text,
    'is_public' : IDL.Bool,
    'updated_at' : IDL.Nat64,
    'members' : IDL.Vec(TeamMember),
    'name' : IDL.Text,
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
    'owner_id' : IDL.Text,
  });
  const UserProfileUpdate = IDL.Record({
    'bio' : IDL.Opt(IDL.Opt(IDL.Text)),
    'username' : IDL.Opt(IDL.Text),
    'avatar_url' : IDL.Opt(IDL.Opt(IDL.Text)),
    'name' : IDL.Opt(IDL.Text),
    'theme_preferences' : IDL.Opt(IDL.Opt(ThemePreferences)),
    'email' : IDL.Opt(IDL.Opt(IDL.Text)),
  });
  const ProjectUpdate = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'description' : IDL.Opt(IDL.Text),
  });
  const TeamUpdate = IDL.Record({
    'is_public' : IDL.Opt(IDL.Bool),
    'name' : IDL.Opt(IDL.Text),
    'description' : IDL.Opt(IDL.Text),
  });
  return IDL.Service({
    'accept_invite' : IDL.Func([IDL.Text], [Result], []),
    'create_project' : IDL.Func([IDL.Text, IDL.Text, Owner], [Result_1], []),
    'create_team' : IDL.Func([IDL.Text, IDL.Text, IDL.Bool], [Result_1], []),
    'create_user' : IDL.Func([UserProfile], [Result_1], []),
    'decline_invite' : IDL.Func([IDL.Text], [Result], []),
    'delete_project' : IDL.Func([IDL.Text], [Result], []),
    'delete_team' : IDL.Func([IDL.Text], [Result], []),
    'get_current_user' : IDL.Func([], [IDL.Opt(User)], ['query']),
    'get_invites' : IDL.Func([IDL.Text], [IDL.Vec(Invite)], ['query']),
    'get_pending_invites' : IDL.Func([IDL.Text], [IDL.Vec(Invite)], ['query']),
    'get_project' : IDL.Func([IDL.Text], [IDL.Opt(Project)], ['query']),
    'get_public_teams' : IDL.Func([], [IDL.Vec(Team)], ['query']),
    'get_team' : IDL.Func([IDL.Text], [IDL.Opt(Team)], ['query']),
    'get_team_projects' : IDL.Func([IDL.Text], [IDL.Vec(Project)], ['query']),
    'get_user' : IDL.Func([IDL.Text], [IDL.Opt(User)], ['query']),
    'get_user_by_principal' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(User)],
        ['query'],
      ),
    'get_user_projects' : IDL.Func([IDL.Text], [IDL.Vec(Project)], ['query']),
    'get_user_teams' : IDL.Func([IDL.Text], [IDL.Vec(Team)], ['query']),
    'get_users' : IDL.Func([], [IDL.Vec(User)], ['query']),
    'health_check' : IDL.Func([], [IDL.Text], ['query']),
    'invite_user' : IDL.Func(
        [InviteTarget, Role, IDL.Opt(IDL.Nat64)],
        [Result_1],
        [],
      ),
    'is_user_registered' : IDL.Func([], [IDL.Bool], ['query']),
    'is_username_available' : IDL.Func([IDL.Text], [IDL.Bool], ['query']),
    'remove_member' : IDL.Func([InviteTarget, IDL.Text], [Result], []),
    'transfer_ownership' : IDL.Func([IDL.Text, Owner], [Result], []),
    'update_profile' : IDL.Func([IDL.Text, UserProfileUpdate], [Result], []),
    'update_project' : IDL.Func([IDL.Text, ProjectUpdate], [Result], []),
    'update_team' : IDL.Func([IDL.Text, TeamUpdate], [Result], []),
    'update_theme_preferences' : IDL.Func([ThemePreferences], [Result], []),
    'update_username' : IDL.Func([IDL.Text, IDL.Text], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
