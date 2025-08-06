use candid::Principal;
use ic_cdk::api::time;
use ic_cdk::api::msg_caller;
use std::result::Result;

use crate::types::*;
use crate::utils;

// Team management
pub async fn create_team(name: String, description: String, is_public: bool) -> Result<String, Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
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
    
    crate::TEAMS.with(|teams| {
        teams.borrow_mut().insert(team_id.clone(), team);
    });
    
    Ok(team_id)
}

pub fn get_team(team_id: String) -> Option<Team> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return None; // Return None if not authenticated
    }
    
    crate::TEAMS.with(|teams| {
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

pub async fn update_team(team_id: String, updates: TeamUpdate) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    crate::TEAMS.with(|teams| {
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

pub async fn delete_team(team_id: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    crate::TEAMS.with(|teams| {
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

pub fn get_user_teams(principal: Principal) -> Vec<Team> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // Users can only query their own teams
    if caller_principal != principal {
        return vec![]; // Return empty for security
    }
    
    crate::TEAMS.with(|teams| {
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

pub fn get_public_teams() -> Vec<Team> {
    crate::TEAMS.with(|teams| {
        teams
            .borrow()
            .values()
            .filter(|team| team.is_public)
            .cloned()
            .collect()
    })
}

// Helper functions
pub fn get_user_role_in_team(team: &Team, principal: &Principal) -> Result<Role, Error> {
    team.members
        .iter()
        .find(|member| member.principal == *principal)
        .map(|member| member.role.clone())
        .ok_or(Error::InsufficientPermissions)
}

pub fn add_user_to_team(team_id: &str, principal: &Principal, role: &Role) -> Result<(), Error> {
    crate::TEAMS.with(|teams| {
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

pub fn remove_user_from_team(team_id: &str, principal: &Principal) -> Result<(), Error> {
    crate::TEAMS.with(|teams| {
        if let Some(team) = teams.borrow_mut().get_mut(team_id) {
            team.members.retain(|member| member.principal != *principal);
            team.updated_at = time();
            Ok(())
        } else {
            Err(Error::TeamNotFound)
        }
    })
} 