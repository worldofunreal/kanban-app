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

// State management
thread_local! {
    static USERS: RefCell<HashMap<String, User>> = RefCell::new(HashMap::new());
    static TEAMS: RefCell<HashMap<String, Team>> = RefCell::new(HashMap::new());
    static PROJECTS: RefCell<HashMap<String, Project>> = RefCell::new(HashMap::new());
    static INVITES: RefCell<HashMap<String, Invite>> = RefCell::new(HashMap::new());
    static USER_PRINCIPALS: RefCell<HashMap<Principal, String>> = RefCell::new(HashMap::new());
}

// User management
#[ic_cdk::update]
async fn create_user(profile: UserProfile) -> Result<String, Error> {
    let caller_principal = msg_caller();
    
    // Check if user already exists with better error handling
    if USER_PRINCIPALS.with(|principals| principals.borrow().contains_key(&caller_principal)) {
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
    
    let user_id = utils::generate_id().await;
    let now = time();
    
    let user = User {
        id: user_id.clone(),
        profile,
        created_at: now,
        updated_at: now,
    };
    
    USERS.with(|users| {
        users.borrow_mut().insert(user_id.clone(), user);
    });
    
    USER_PRINCIPALS.with(|principals| {
        principals.borrow_mut().insert(caller_principal, user_id.clone());
    });
    
    Ok(user_id)
}

#[ic_cdk::query]
fn get_user(user_id: String) -> Option<User> {
    USERS.with(|users| users.borrow().get(&user_id).cloned())
}

#[ic_cdk::query]
fn is_user_registered() -> bool {
    let caller_principal = msg_caller();
    USER_PRINCIPALS.with(|principals| principals.borrow().contains_key(&caller_principal))
}

#[ic_cdk::query]
fn get_user_by_principal(principal: Principal) -> Option<User> {
    let user_id = USER_PRINCIPALS.with(|principals| principals.borrow().get(&principal).cloned())?;
    get_user(user_id)
}

#[ic_cdk::query]
fn is_username_available(username: String) -> bool {
    USERS.with(|users| {
        !users.borrow().values().any(|user| user.profile.username == username)
    })
}

#[ic_cdk::update]
async fn update_profile(user_id: String, profile_update: UserProfileUpdate) -> Result<(), Error> {
    let caller_principal = msg_caller();
    let current_user_id = get_current_user_id(&caller_principal)?;
    
    if current_user_id != user_id {
        return Err(Error::Unauthorized);
    }
    
    USERS.with(|users| {
        if let Some(user) = users.borrow_mut().get_mut(&user_id) {
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
    let user_id = get_current_user_id(&caller_principal)?;
    
    USERS.with(|users| {
        if let Some(user) = users.borrow_mut().get_mut(&user_id) {
            user.profile.theme_preferences = Some(theme_preferences);
            user.updated_at = time();
            Ok(())
        } else {
            Err(Error::UserNotFound)
        }
    })
}

#[ic_cdk::update]
async fn update_username(user_id: String, username: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    let current_user_id = get_current_user_id(&caller_principal)?;
    
    if current_user_id != user_id {
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
        if let Some(user) = users.borrow_mut().get_mut(&user_id) {
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
    USERS.with(|users| users.borrow().values().cloned().collect())
}

// Team management
#[ic_cdk::update]
async fn create_team(name: String, description: String, is_public: bool) -> Result<String, Error> {
    let caller_principal = msg_caller();
    let user_id = get_current_user_id(&caller_principal)?;
    
    let team_id = utils::generate_id().await;
    let now = time();
    
    let team = Team {
        id: team_id.clone(),
        name,
        description,
        is_public,
        owner_id: user_id.clone(),
        members: vec![TeamMember {
            user_id,
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
    TEAMS.with(|teams| teams.borrow().get(&team_id).cloned())
}

#[ic_cdk::update]
async fn update_team(team_id: String, updates: TeamUpdate) -> Result<(), Error> {
    let caller_principal = msg_caller();
    let user_id = get_current_user_id(&caller_principal)?;
    
    TEAMS.with(|teams| {
        if let Some(team) = teams.borrow_mut().get_mut(&team_id) {
            // Check if user is owner or has management role
            let user_role = get_user_role_in_team(team, &user_id)?;
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
    let user_id = get_current_user_id(&caller_principal)?;
    
    TEAMS.with(|teams| {
        if let Some(team) = teams.borrow().get(&team_id) {
            if team.owner_id != user_id {
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
fn get_user_teams(user_id: String) -> Vec<Team> {
    TEAMS.with(|teams| {
        teams
            .borrow()
            .values()
            .filter(|team| {
                team.members.iter().any(|member| member.user_id == user_id)
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
    let user_id = get_current_user_id(&caller_principal)?;
    
    // Validate owner
    match &owner {
        Owner::User(owner_user_id) => {
            if owner_user_id != &user_id {
                return Err(Error::InsufficientPermissions);
            }
        }
        Owner::Team(team_id) => {
            let team = get_team(team_id.clone()).ok_or(Error::TeamNotFound)?;
            let user_role = get_user_role_in_team(&team, &user_id)?;
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
    PROJECTS.with(|projects| projects.borrow().get(&project_id).cloned())
}

#[ic_cdk::update]
async fn update_project(project_id: String, updates: ProjectUpdate) -> Result<(), Error> {
    let caller_principal = msg_caller();
    let user_id = get_current_user_id(&caller_principal)?;
    
    PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow_mut().get_mut(&project_id) {
            // Check if user has permission to update
            let user_role = get_user_role_in_project(project, &user_id)?;
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
    let user_id = get_current_user_id(&caller_principal)?;
    
    PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow().get(&project_id) {
            let user_role = get_user_role_in_project(project, &user_id)?;
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
    let user_id = get_current_user_id(&caller_principal)?;
    
    PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow_mut().get_mut(&project_id) {
            let user_role = get_user_role_in_project(project, &user_id)?;
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
fn get_user_projects(user_id: String) -> Vec<Project> {
    PROJECTS.with(|projects| {
        projects
            .borrow()
            .values()
            .filter(|project| {
                project.members.iter().any(|member| member.user_id == user_id)
            })
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_team_projects(team_id: String) -> Vec<Project> {
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
async fn invite_user(target: InviteTarget, role: Role, expires_at: Option<u64>) -> Result<String, Error> {
    let caller_principal = msg_caller();
    let user_id = get_current_user_id(&caller_principal)?;
    
    let invite_id = utils::generate_id().await;
    let now = time();
    
    let invite = Invite {
        id: invite_id.clone(),
        target,
        role,
        invited_by: user_id,
        expires_at,
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
    let user_id = get_current_user_id(&caller_principal)?;
    
    INVITES.with(|invites| {
        if let Some(invite) = invites.borrow().get(&invite_id) {
            // Check if invite is expired
            if let Some(expires_at) = invite.expires_at {
                if time() > expires_at {
                    return Err(Error::InviteExpired);
                }
            }
            
            match &invite.target {
                InviteTarget::Team(team_id) => {
                    add_user_to_team(team_id, &user_id, &invite.role)?;
                }
                InviteTarget::Project(project_id) => {
                    add_user_to_project(project_id, &user_id, &invite.role)?;
                }
            }
            
            invites.borrow_mut().remove(&invite_id);
            Ok(())
        } else {
            Err(Error::InviteNotFound)
        }
    })
}

#[ic_cdk::update]
async fn decline_invite(invite_id: String) -> Result<(), Error> {
    INVITES.with(|invites| {
        if invites.borrow().contains_key(&invite_id) {
            invites.borrow_mut().remove(&invite_id);
            Ok(())
        } else {
            Err(Error::InviteNotFound)
        }
    })
}

#[ic_cdk::update]
async fn remove_member(target: InviteTarget, user_id: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    let current_user_id = get_current_user_id(&caller_principal)?;
    
    match target {
        InviteTarget::Team(team_id) => {
            let team = get_team(team_id.clone()).ok_or(Error::TeamNotFound)?;
            let user_role = get_user_role_in_team(&team, &current_user_id)?;
            if user_role != Role::Owner && user_role != Role::Manager {
                return Err(Error::InsufficientPermissions);
            }
            remove_user_from_team(&team_id, &user_id)
        }
        InviteTarget::Project(project_id) => {
            let project = get_project(project_id.clone()).ok_or(Error::ProjectNotFound)?;
            let user_role = get_user_role_in_project(&project, &current_user_id)?;
            if user_role != Role::Owner && user_role != Role::Manager {
                return Err(Error::InsufficientPermissions);
            }
            remove_user_from_project(&project_id, &user_id)
        }
    }
}

#[ic_cdk::query]
fn get_invites(_user_id: String) -> Vec<Invite> {
    INVITES.with(|invites| {
        invites
            .borrow()
            .values()
            .filter(|_invite| {
                // This is a simplified version - in a real app you'd track who was invited
                true
            })
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_pending_invites(_user_id: String) -> Vec<Invite> {
    INVITES.with(|invites| {
        invites
            .borrow()
            .values()
            .filter(|_invite| {
                // This is a simplified version - in a real app you'd track pending invites
                true
            })
            .cloned()
            .collect()
    })
}

// Utility functions
#[ic_cdk::query]
fn get_current_user() -> Option<User> {
    let caller_principal = msg_caller();
    let user_id = get_current_user_id(&caller_principal).ok()?;
    get_user(user_id)
}

#[ic_cdk::query]
fn health_check() -> String {
    "Kanban App Backend is running!".to_string()
}

// Helper functions
fn get_current_user_id(caller_principal: &Principal) -> Result<String, Error> {
    USER_PRINCIPALS
        .with(|principals| principals.borrow().get(caller_principal).cloned())
        .ok_or(Error::Unauthorized)
}

fn get_user_role_in_team(team: &Team, user_id: &str) -> Result<Role, Error> {
    team.members
        .iter()
        .find(|member| member.user_id == user_id)
        .map(|member| member.role.clone())
        .ok_or(Error::InsufficientPermissions)
}

fn get_user_role_in_project(project: &Project, user_id: &str) -> Result<Role, Error> {
    project.members
        .iter()
        .find(|member| member.user_id == user_id)
        .map(|member| member.role.clone())
        .ok_or(Error::InsufficientPermissions)
}

fn add_user_to_team(team_id: &str, user_id: &str, role: &Role) -> Result<(), Error> {
    TEAMS.with(|teams| {
        if let Some(team) = teams.borrow_mut().get_mut(team_id) {
            // Check if user is already a member
            if team.members.iter().any(|member| member.user_id == user_id) {
                return Err(Error::AlreadyExists);
            }
            
            team.members.push(TeamMember {
                user_id: user_id.to_string(),
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

fn add_user_to_project(project_id: &str, user_id: &str, role: &Role) -> Result<(), Error> {
    PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow_mut().get_mut(project_id) {
            // Check if user is already a member
            if project.members.iter().any(|member| member.user_id == user_id) {
                return Err(Error::AlreadyExists);
            }
            
            project.members.push(ProjectMember {
                user_id: user_id.to_string(),
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

fn remove_user_from_team(team_id: &str, user_id: &str) -> Result<(), Error> {
    TEAMS.with(|teams| {
        if let Some(team) = teams.borrow_mut().get_mut(team_id) {
            team.members.retain(|member| member.user_id != user_id);
            team.updated_at = time();
            Ok(())
        } else {
            Err(Error::TeamNotFound)
        }
    })
}

fn remove_user_from_project(project_id: &str, user_id: &str) -> Result<(), Error> {
    PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow_mut().get_mut(project_id) {
            project.members.retain(|member| member.user_id != user_id);
            project.updated_at = time();
            Ok(())
        } else {
            Err(Error::ProjectNotFound)
        }
    })
}

ic_cdk::export_candid!();
