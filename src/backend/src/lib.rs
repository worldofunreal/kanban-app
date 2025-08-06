use candid::Principal;
use std::collections::HashMap;
use std::cell::RefCell;
use std::result::Result;

// Re-export types for candid
pub use crate::types::*;

mod types;
mod utils;
mod user;
mod team;
mod project;
mod invite;

// State management - using Principal as primary key
thread_local! {
    static USERS: RefCell<HashMap<Principal, User>> = RefCell::new(HashMap::new());
    static TEAMS: RefCell<HashMap<String, Team>> = RefCell::new(HashMap::new());
    static PROJECTS: RefCell<HashMap<String, Project>> = RefCell::new(HashMap::new());
    static INVITES: RefCell<HashMap<String, Invite>> = RefCell::new(HashMap::new());
}

// Public API endpoints - User management
#[ic_cdk::update]
async fn create_user(profile: UserProfile) -> Result<Principal, Error> {
    user::create_user(profile).await
}

#[ic_cdk::query]
fn get_user(principal: Principal) -> Option<User> {
    user::get_user(principal)
}

#[ic_cdk::update]
async fn update_profile(principal: Principal, profile_update: UserProfileUpdate) -> Result<(), Error> {
    user::update_profile(principal, profile_update).await
}

#[ic_cdk::update]
async fn update_theme_preferences(theme_preferences: ThemePreferences) -> Result<(), Error> {
    user::update_theme_preferences(theme_preferences).await
}

#[ic_cdk::update]
async fn update_username(principal: Principal, username: String) -> Result<(), Error> {
    user::update_username(principal, username).await
}

#[ic_cdk::query]
fn get_users() -> Vec<User> {
    user::get_users()
}

// Public API endpoints - Team management
#[ic_cdk::update]
async fn create_team(name: String, description: String, is_public: bool) -> Result<String, Error> {
    team::create_team(name, description, is_public).await
}

#[ic_cdk::query]
fn get_team(team_id: String) -> Option<Team> {
    team::get_team(team_id)
}

#[ic_cdk::update]
async fn update_team(team_id: String, updates: TeamUpdate) -> Result<(), Error> {
    team::update_team(team_id, updates).await
}

#[ic_cdk::update]
async fn delete_team(team_id: String) -> Result<(), Error> {
    team::delete_team(team_id).await
}

#[ic_cdk::query]
fn get_user_teams(principal: Principal) -> Vec<Team> {
    team::get_user_teams(principal)
}

#[ic_cdk::query]
fn get_public_teams() -> Vec<Team> {
    team::get_public_teams()
}

// Public API endpoints - Project management
#[ic_cdk::update]
async fn create_project(name: String, description: String, owner: Owner) -> Result<String, Error> {
    project::create_project(name, description, owner).await
}

#[ic_cdk::query]
fn get_project(project_id: String) -> Option<Project> {
    project::get_project(project_id)
}

#[ic_cdk::update]
async fn update_project(project_id: String, updates: ProjectUpdate) -> Result<(), Error> {
    project::update_project(project_id, updates).await
}

#[ic_cdk::update]
async fn delete_project(project_id: String) -> Result<(), Error> {
    project::delete_project(project_id).await
}

#[ic_cdk::update]
async fn transfer_ownership(project_id: String, new_owner: Owner) -> Result<(), Error> {
    project::transfer_ownership(project_id, new_owner).await
}

#[ic_cdk::query]
fn get_user_projects(principal: Principal) -> Vec<Project> {
    project::get_user_projects(principal)
}

#[ic_cdk::query]
fn get_team_projects(team_id: String) -> Vec<Project> {
    project::get_team_projects(team_id)
}

// Public API endpoints - Invitation management
#[ic_cdk::update]
async fn invite_user(target: InviteTarget, role: Role, invited_user: Principal) -> Result<String, Error> {
    invite::invite_user(target, role, invited_user).await
}

#[ic_cdk::update]
async fn accept_invite(invite_id: String) -> Result<(), Error> {
    invite::accept_invite(invite_id).await
}

#[ic_cdk::update]
async fn decline_invite(invite_id: String) -> Result<(), Error> {
    invite::decline_invite(invite_id).await
}

#[ic_cdk::update]
async fn cancel_invite(invite_id: String) -> Result<(), Error> {
    invite::cancel_invite(invite_id).await
}

#[ic_cdk::update]
async fn remove_member(target: InviteTarget, principal: Principal) -> Result<(), Error> {
    invite::remove_member(target, principal).await
}

#[ic_cdk::query]
fn get_invites(principal: Principal) -> Vec<Invite> {
    invite::get_invites(principal)
}

#[ic_cdk::query]
fn get_pending_invites(principal: Principal) -> Vec<Invite> {
    invite::get_pending_invites(principal)
}

// Utility functions
#[ic_cdk::query]
fn health_check() -> String {
    "Kanban App Backend is running!".to_string()
}

ic_cdk::export_candid!();
