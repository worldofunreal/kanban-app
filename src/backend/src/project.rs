use candid::Principal;
use ic_cdk::api::time;
use ic_cdk::api::msg_caller;
use std::result::Result;

use crate::types::*;
use crate::utils;
use crate::team;

// Project management
pub async fn create_project(name: String, description: String, owner: Owner) -> Result<String, Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
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
            let team = team::get_team(team_id.clone()).ok_or(Error::TeamNotFound)?;
            let user_role = team::get_user_role_in_team(&team, &caller_principal)?;
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
    
    crate::PROJECTS.with(|projects| {
        projects.borrow_mut().insert(project_id.clone(), project);
    });
    
    Ok(project_id)
}

pub fn get_project(project_id: String) -> Option<Project> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return None; // Return None if not authenticated
    }
    
    crate::PROJECTS.with(|projects| {
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

pub async fn update_project(project_id: String, updates: ProjectUpdate) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    crate::PROJECTS.with(|projects| {
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

pub async fn delete_project(project_id: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    crate::PROJECTS.with(|projects| {
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

pub async fn transfer_ownership(project_id: String, new_owner: Owner) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    crate::PROJECTS.with(|projects| {
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

pub fn get_user_projects(principal: Principal) -> Vec<Project> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // Users can only query their own projects
    if caller_principal != principal {
        return vec![]; // Return empty for security
    }
    
    crate::PROJECTS.with(|projects| {
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

pub fn get_team_projects(team_id: String) -> Vec<Project> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // Check if user is a member of the team
    let is_team_member = crate::TEAMS.with(|teams| {
        if let Some(team) = teams.borrow().get(&team_id) {
            team.members.iter().any(|member| member.principal == caller_principal)
        } else {
            false
        }
    });
    
    if !is_team_member {
        return vec![]; // Return empty for security
    }
    
    crate::PROJECTS.with(|projects| {
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

// Helper functions
pub fn get_user_role_in_project(project: &Project, principal: &Principal) -> Result<Role, Error> {
    project.members
        .iter()
        .find(|member| member.principal == *principal)
        .map(|member| member.role.clone())
        .ok_or(Error::InsufficientPermissions)
}

pub fn add_user_to_project(project_id: &str, principal: &Principal, role: &Role) -> Result<(), Error> {
    crate::PROJECTS.with(|projects| {
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

pub fn remove_user_from_project(project_id: &str, principal: &Principal) -> Result<(), Error> {
    crate::PROJECTS.with(|projects| {
        if let Some(project) = projects.borrow_mut().get_mut(project_id) {
            project.members.retain(|member| member.principal != *principal);
            project.updated_at = time();
            Ok(())
        } else {
            Err(Error::ProjectNotFound)
        }
    })
} 