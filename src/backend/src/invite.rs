use candid::Principal;
use ic_cdk::api::time;
use ic_cdk::api::msg_caller;
use std::result::Result;

use crate::types::*;
use crate::utils;
use crate::team;
use crate::project;

// Invitation management
pub async fn invite_user(target: InviteTarget, role: Role, invited_user: Principal) -> Result<String, Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return Err(Error::Unauthorized);
    }
    
    // Ensure invited user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&invited_user)) {
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
    
    crate::INVITES.with(|invites| {
        invites.borrow_mut().insert(invite_id.clone(), invite);
    });
    
    Ok(invite_id)
}

pub async fn accept_invite(invite_id: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return Err(Error::Unauthorized);
    }
    
    crate::INVITES.with(|invites| {
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
                    team::add_user_to_team(team_id, &caller_principal, &invite.role)?;
                }
                InviteTarget::Project(project_id) => {
                    project::add_user_to_project(project_id, &caller_principal, &invite.role)?;
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

pub async fn decline_invite(invite_id: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return Err(Error::Unauthorized);
    }
    
    crate::INVITES.with(|invites| {
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

pub async fn cancel_invite(invite_id: String) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return Err(Error::Unauthorized);
    }
    
    crate::INVITES.with(|invites| {
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

pub async fn remove_member(target: InviteTarget, principal: Principal) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    match target {
        InviteTarget::Team(team_id) => {
            let team = team::get_team(team_id.clone()).ok_or(Error::TeamNotFound)?;
            let user_role = team::get_user_role_in_team(&team, &caller_principal)?;
            if user_role != Role::Owner && user_role != Role::Manager {
                return Err(Error::InsufficientPermissions);
            }
            team::remove_user_from_team(&team_id, &principal)
        }
        InviteTarget::Project(project_id) => {
            let project = project::get_project(project_id.clone()).ok_or(Error::ProjectNotFound)?;
            let user_role = project::get_user_role_in_project(&project, &caller_principal)?;
            if user_role != Role::Owner && user_role != Role::Manager {
                return Err(Error::InsufficientPermissions);
            }
            project::remove_user_from_project(&project_id, &principal)
        }
    }
}

pub fn get_invites(principal: Principal) -> Vec<Invite> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // Users can only query their own invites
    if caller_principal != principal {
        return vec![]; // Return empty for security
    }
    
    crate::INVITES.with(|invites| {
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

pub fn get_pending_invites(principal: Principal) -> Vec<Invite> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // Users can only query their own invites
    if caller_principal != principal {
        return vec![]; // Return empty for security
    }
    
    crate::INVITES.with(|invites| {
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