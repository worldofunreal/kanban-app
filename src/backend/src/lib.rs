use candid::Principal;
use ic_cdk::api::time;
use ic_cdk::api::msg_caller;
use std::collections::HashMap;
use std::cell::RefCell;
use std::result::Result;

// Re-export types for candid
pub use crate::types::*;

mod types;
mod utils;

// State management - using Principal as primary key
thread_local! {
    static USERS: RefCell<HashMap<Principal, User>> = RefCell::new(HashMap::new());
    static TEAMS: RefCell<HashMap<String, Team>> = RefCell::new(HashMap::new());
    static PROJECTS: RefCell<HashMap<String, Project>> = RefCell::new(HashMap::new());
    static INVITES: RefCell<HashMap<String, Invite>> = RefCell::new(HashMap::new());
}

// User management
#[ic_cdk::update]
async fn create_user(profile: UserProfile) -> Result<Principal, Error> {
    let caller_principal = msg_caller();
    
    // Check if user already exists
    if USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return Err(Error::AlreadyExists);
    }
    
    // Validate profile data
    if profile.name.trim().is_empty() {
        return Err(Error::InvalidInput("Name cannot be empty".to_string()));
    }
    
    if profile.username.trim().is_empty() {
        return Err(Error::InvalidInput("Username cannot be empty".to_string()));
    }
    
    if profile.username.len() > 12 {
        return Err(Error::InvalidInput("Username too long (max 12 characters)".to_string()));
    }
    
    // Check if username is already taken by another user
    let username_taken = USERS.with(|users| {
        users.borrow().values().any(|user| user.profile.username == profile.username)
    });
    
    if username_taken {
        return Err(Error::InvalidInput("Username is already taken".to_string()));
    }
    
    let now = time();
    
    let user = User {
        principal: caller_principal,
        profile,
        created_at: now,
        updated_at: now,
    };
    
    USERS.with(|users| {
        users.borrow_mut().insert(caller_principal, user);
    });
    
    Ok(caller_principal)
}

#[ic_cdk::query]
fn get_user(principal: Principal) -> Option<User> {
    let caller_principal = msg_caller();
    
    // Users can only query their own data
    if caller_principal != principal {
        return None; // Return None instead of user data for security
    }
    
    USERS.with(|users| users.borrow().get(&principal).cloned())
}

#[ic_cdk::update]
async fn update_profile(principal: Principal, profile_update: UserProfileUpdate) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    // Only allow users to update their own profile
    if caller_principal != principal {
        return Err(Error::Unauthorized);
    }
    
    USERS.with(|users| {
        if let Some(user) = users.borrow_mut().get_mut(&principal) {
            if let Some(name) = profile_update.name {
                user.profile.name = name;
            }
            if let Some(username) = profile_update.username {
                user.profile.username = username;
            }
            if let Some(email) = profile_update.email {
                user.profile.email = email;
            }
            if let Some(avatar_url) = profile_update.avatar_url {
                user.profile.avatar_url = avatar_url;
            }
            if let Some(bio) = profile_update.bio {
                user.profile.bio = bio;
            }
            if let Some(theme_preferences) = profile_update.theme_preferences {
                user.profile.theme_preferences = theme_preferences;
            }
            user.updated_at = time();
            Ok(())
        } else {
            Err(Error::UserNotFound)
        }
    })
}

#[ic_cdk::update]
async fn update_theme_preferences(theme_preferences: ThemePreferences) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    USERS.with(|users| {
        if let Some(user) = users.borrow_mut().get_mut(&caller_principal) {
            user.profile.theme_preferences = Some(theme_preferences);
            user.updated_at = time();
            Ok(())
        } else {
            Err(Error::UserNotFound)
        }
    })
}

#[ic_cdk::update]
async fn update_username(principal: Principal, username: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    // Only allow users to update their own username
    if caller_principal != principal {
        return Err(Error::Unauthorized);
    }
    
    // Basic username validation
    if username.trim().is_empty() {
        return Err(Error::InvalidInput("Username cannot be empty".to_string()));
    }
    
    if username.len() > 12 {
        return Err(Error::InvalidInput("Username too long (max 12 characters)".to_string()));
    }
    
    USERS.with(|users| {
        if let Some(user) = users.borrow_mut().get_mut(&principal) {
            user.profile.username = username;
            user.updated_at = time();
            Ok(())
        } else {
            Err(Error::UserNotFound)
        }
    })
}

#[ic_cdk::query]
fn get_users() -> Vec<User> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // For now, return all users (in a real app, you'd filter by team membership, etc.)
    // This is a simplified version - in production you'd implement proper access control
    USERS.with(|users| users.borrow().values().cloned().collect())
}

// Team management
#[ic_cdk::update]
async fn create_team(name: String, description: String, is_public: bool) -> Result<String, Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return Err(Error::Unauthorized);
    }
    
    let team_id = utils::generate_id().await;
    let now = time();
    
    let team = Team {
        id: team_id.clone(),
        name,
        description,
        is_public,
        owner_principal: caller_principal,
        members: vec![TeamMember {
            principal: caller_principal,
            role: Role::Owner,
            joined_at: now,
        }],
        created_at: now,
        updated_at: now,
    };
    
    TEAMS.with(|teams| {
        teams.borrow_mut().insert(team_id.clone(), team);
    });
    
    Ok(team_id)
}

#[ic_cdk::query]
fn get_team(team_id: String) -> Option<Team> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return None; // Return None if not authenticated
    }
    
    TEAMS.with(|teams| {
        if let Some(team) = teams.borrow().get(&team_id) {
            // Allow access if team is public or user is a member
            if team.is_public || team.members.iter().any(|member| member.principal == caller_principal) {
                Some(team.clone())
            } else {
                None // Return None for security
            }
        } else {
            None
        }
    })
}

#[ic_cdk::update]
async fn update_team(team_id: String, updates: TeamUpdate) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    TEAMS.with(|teams| {
        if let Some(team) = teams.borrow_mut().get_mut(&team_id) {
            // Check if user is owner or has management role
            let user_role = get_user_role_in_team(team, &caller_principal)?;
            if user_role != Role::Owner && user_role != Role::Manager {
                return Err(Error::InsufficientPermissions);
            }
            
            if let Some(name) = updates.name {
                team.name = name;
            }
            if let Some(description) = updates.description {
                team.description = description;
            }
            if let Some(is_public) = updates.is_public {
                team.is_public = is_public;
            }
            
            team.updated_at = time();
            Ok(())
        } else {
            Err(Error::TeamNotFound)
        }
    })
}

#[ic_cdk::update]
async fn delete_team(team_id: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    TEAMS.with(|teams| {
        if let Some(team) = teams.borrow().get(&team_id) {
            if team.owner_principal != caller_principal {
                return Err(Error::InsufficientPermissions);
            }
            
            teams.borrow_mut().remove(&team_id);
            Ok(())
        } else {
            Err(Error::TeamNotFound)
        }
    })
}

#[ic_cdk::query]
fn get_user_teams(principal: Principal) -> Vec<Team> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // Users can only query their own teams
    if caller_principal != principal {
        return vec![]; // Return empty for security
    }
    
    TEAMS.with(|teams| {
        teams
            .borrow()
            .values()
            .filter(|team| {
                team.members.iter().any(|member| member.principal == principal)
            })
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_public_teams() -> Vec<Team> {
    TEAMS.with(|teams| {
        teams
            .borrow()
            .values()
            .filter(|team| team.is_public)
            .cloned()
            .collect()
    })
}

// Project management
#[ic_cdk::update]
async fn create_project(name: String, description: String, owner: Owner) -> Result<String, Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return Err(Error::Unauthorized);
    }
    
    // Validate owner
    match &owner {
        Owner::User(owner_principal) => {
            if owner_principal != &caller_principal {
                return Err(Error::InsufficientPermissions);
            }
        }
        Owner::Team(team_id) => {
            let team = get_team(team_id.clone()).ok_or(Error::TeamNotFound)?;
            let user_role = get_user_role_in_team(&team, &caller_principal)?;
            if user_role != Role::Owner && user_role != Role::Manager {
                return Err(Error::InsufficientPermissions);
            }
        }
    }
    
    let project_id = utils::generate_id().await;
    let now = time();
    
    let project = Project {
        id: project_id.clone(),
        name,
        description,
        owner,
        members: vec![],
        created_at: now,
        updated_at: now,
    };
    
    PROJECTS.with(|projects| {
        projects.borrow_mut().insert(project_id.clone(), project);
    });
    
    Ok(project_id)
}

#[ic_cdk::query]
fn get_project(project_id: String) -> Option<Project> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return None; // Return None if not authenticated
    }
    
    PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow().get(&project_id) {
            // Allow access if user is a member of the project
            if project.members.iter().any(|member| member.principal == caller_principal) {
                Some(project.clone())
            } else {
                None // Return None for security
            }
        } else {
            None
        }
    })
}

#[ic_cdk::update]
async fn update_project(project_id: String, updates: ProjectUpdate) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow_mut().get_mut(&project_id) {
            // Check if user has permission to update
            let user_role = get_user_role_in_project(project, &caller_principal)?;
            if user_role != Role::Owner && user_role != Role::Manager {
                return Err(Error::InsufficientPermissions);
            }
            
            if let Some(name) = updates.name {
                project.name = name;
            }
            if let Some(description) = updates.description {
                project.description = description;
            }
            
            project.updated_at = time();
            Ok(())
        } else {
            Err(Error::ProjectNotFound)
        }
    })
}

#[ic_cdk::update]
async fn delete_project(project_id: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow().get(&project_id) {
            let user_role = get_user_role_in_project(project, &caller_principal)?;
            if user_role != Role::Owner {
                return Err(Error::InsufficientPermissions);
            }
            
            projects.borrow_mut().remove(&project_id);
            Ok(())
        } else {
            Err(Error::ProjectNotFound)
        }
    })
}

#[ic_cdk::update]
async fn transfer_ownership(project_id: String, new_owner: Owner) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow_mut().get_mut(&project_id) {
            let user_role = get_user_role_in_project(project, &caller_principal)?;
            if user_role != Role::Owner {
                return Err(Error::InsufficientPermissions);
            }
            
            project.owner = new_owner;
            project.updated_at = time();
            Ok(())
        } else {
            Err(Error::ProjectNotFound)
        }
    })
}

#[ic_cdk::query]
fn get_user_projects(principal: Principal) -> Vec<Project> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // Users can only query their own projects
    if caller_principal != principal {
        return vec![]; // Return empty for security
    }
    
    PROJECTS.with(|projects| {
        projects
            .borrow()
            .values()
            .filter(|project| {
                project.members.iter().any(|member| member.principal == principal)
            })
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_team_projects(team_id: String) -> Vec<Project> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // Check if user is a member of the team
    let is_team_member = TEAMS.with(|teams| {
        if let Some(team) = teams.borrow().get(&team_id) {
            team.members.iter().any(|member| member.principal == caller_principal)
        } else {
            false
        }
    });
    
    if !is_team_member {
        return vec![]; // Return empty for security
    }
    
    PROJECTS.with(|projects| {
        projects
            .borrow()
            .values()
            .filter(|project| {
                matches!(project.owner, Owner::Team(ref owner_team_id) if owner_team_id == &team_id)
            })
            .cloned()
            .collect()
    })
}

// Invitation management
#[ic_cdk::update]
async fn invite_user(target: InviteTarget, role: Role, invited_user: Principal) -> Result<String, Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return Err(Error::Unauthorized);
    }
    
    // Ensure invited user exists
    if !USERS.with(|users| users.borrow().contains_key(&invited_user)) {
        return Err(Error::UserNotFound);
    }
    
    let invite_id = utils::generate_id().await;
    let now = time();
    
    let invite = Invite {
        id: invite_id.clone(),
        target,
        role,
        invited_by: caller_principal,
        invited_user,
        status: InviteStatus::Pending,
        created_at: now,
    };
    
    INVITES.with(|invites| {
        invites.borrow_mut().insert(invite_id.clone(), invite);
    });
    
    Ok(invite_id)
}

#[ic_cdk::update]
async fn accept_invite(invite_id: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return Err(Error::Unauthorized);
    }
    
    INVITES.with(|invites| {
        if let Some(invite) = invites.borrow_mut().get_mut(&invite_id) {
            // Only the invited user can accept the invite
            if invite.invited_user != caller_principal {
                return Err(Error::Unauthorized);
            }
            
            // Check if invite is still pending
            if invite.status != InviteStatus::Pending {
                return Err(Error::InviteExpired);
            }
            
            // Add user to team/project
            match &invite.target {
                InviteTarget::Team(team_id) => {
                    add_user_to_team(team_id, &caller_principal, &invite.role)?;
                }
                InviteTarget::Project(project_id) => {
                    add_user_to_project(project_id, &caller_principal, &invite.role)?;
                }
            }
            
            // Update invite status to accepted
            invite.status = InviteStatus::Accepted;
            Ok(())
        } else {
            Err(Error::InviteNotFound)
        }
    })
}

#[ic_cdk::update]
async fn decline_invite(invite_id: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return Err(Error::Unauthorized);
    }
    
    INVITES.with(|invites| {
        if let Some(invite) = invites.borrow_mut().get_mut(&invite_id) {
            // Only the invited user can decline the invite
            if invite.invited_user != caller_principal {
                return Err(Error::Unauthorized);
            }
            
            // Check if invite is still pending
            if invite.status != InviteStatus::Pending {
                return Err(Error::InviteExpired);
            }
            
            // Update invite status to declined
            invite.status = InviteStatus::Declined;
            Ok(())
        } else {
            Err(Error::InviteNotFound)
        }
    })
}

#[ic_cdk::update]
async fn cancel_invite(invite_id: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return Err(Error::Unauthorized);
    }
    
    INVITES.with(|invites| {
        if let Some(invite) = invites.borrow_mut().get_mut(&invite_id) {
            // Only the person who sent the invite can cancel it
            if invite.invited_by != caller_principal {
                return Err(Error::Unauthorized);
            }
            
            // Check if invite is still pending
            if invite.status != InviteStatus::Pending {
                return Err(Error::InviteExpired);
            }
            
            // Update invite status to cancelled
            invite.status = InviteStatus::Cancelled;
            Ok(())
        } else {
            Err(Error::InviteNotFound)
        }
    })
}

#[ic_cdk::update]
async fn remove_member(target: InviteTarget, principal: Principal) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    match target {
        InviteTarget::Team(team_id) => {
            let team = get_team(team_id.clone()).ok_or(Error::TeamNotFound)?;
            let user_role = get_user_role_in_team(&team, &caller_principal)?;
            if user_role != Role::Owner && user_role != Role::Manager {
                return Err(Error::InsufficientPermissions);
            }
            remove_user_from_team(&team_id, &principal)
        }
        InviteTarget::Project(project_id) => {
            let project = get_project(project_id.clone()).ok_or(Error::ProjectNotFound)?;
            let user_role = get_user_role_in_project(&project, &caller_principal)?;
            if user_role != Role::Owner && user_role != Role::Manager {
                return Err(Error::InsufficientPermissions);
            }
            remove_user_from_project(&project_id, &principal)
        }
    }
}

#[ic_cdk::query]
fn get_invites(principal: Principal) -> Vec<Invite> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // Users can only query their own invites
    if caller_principal != principal {
        return vec![]; // Return empty for security
    }
    
    INVITES.with(|invites| {
        invites
            .borrow()
            .values()
            .filter(|invite| {
                // Return invites where the user is either the inviter or the invited user
                invite.invited_by == principal || invite.invited_user == principal
            })
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_pending_invites(principal: Principal) -> Vec<Invite> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // Users can only query their own invites
    if caller_principal != principal {
        return vec![]; // Return empty for security
    }
    
    INVITES.with(|invites| {
        invites
            .borrow()
            .values()
            .filter(|invite| {
                // Return only pending invites where the user is the invited user
                invite.status == InviteStatus::Pending && invite.invited_user == principal
            })
            .cloned()
            .collect()
    })
}

// Utility functions
#[ic_cdk::query]
fn health_check() -> String {
    "Kanban App Backend is running!".to_string()
}

// Helper functions
fn get_user_role_in_team(team: &Team, principal: &Principal) -> Result<Role, Error> {
    team.members
        .iter()
        .find(|member| member.principal == *principal)
        .map(|member| member.role.clone())
        .ok_or(Error::InsufficientPermissions)
}

fn get_user_role_in_project(project: &Project, principal: &Principal) -> Result<Role, Error> {
    project.members
        .iter()
        .find(|member| member.principal == *principal)
        .map(|member| member.role.clone())
        .ok_or(Error::InsufficientPermissions)
}

fn add_user_to_team(team_id: &str, principal: &Principal, role: &Role) -> Result<(), Error> {
    TEAMS.with(|teams| {
        if let Some(team) = teams.borrow_mut().get_mut(team_id) {
            // Check if user is already a member
            if team.members.iter().any(|member| member.principal == *principal) {
                return Err(Error::AlreadyExists);
            }
            
            team.members.push(TeamMember {
                principal: *principal,
                role: role.clone(),
                joined_at: time(),
            });
            team.updated_at = time();
            Ok(())
        } else {
            Err(Error::TeamNotFound)
        }
    })
}

fn add_user_to_project(project_id: &str, principal: &Principal, role: &Role) -> Result<(), Error> {
    PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow_mut().get_mut(project_id) {
            // Check if user is already a member
            if project.members.iter().any(|member| member.principal == *principal) {
                return Err(Error::AlreadyExists);
            }
            
            project.members.push(ProjectMember {
                principal: *principal,
                role: role.clone(),
                joined_at: time(),
            });
            project.updated_at = time();
            Ok(())
        } else {
            Err(Error::ProjectNotFound)
        }
    })
}

fn remove_user_from_team(team_id: &str, principal: &Principal) -> Result<(), Error> {
    TEAMS.with(|teams| {
        if let Some(team) = teams.borrow_mut().get_mut(team_id) {
            team.members.retain(|member| member.principal != *principal);
            team.updated_at = time();
            Ok(())
        } else {
            Err(Error::TeamNotFound)
        }
    })
}

fn remove_user_from_project(project_id: &str, principal: &Principal) -> Result<(), Error> {
    PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow_mut().get_mut(project_id) {
            project.members.retain(|member| member.principal != *principal);
            project.updated_at = time();
            Ok(())
        } else {
            Err(Error::ProjectNotFound)
        }
    })
}

ic_cdk::export_candid!();
