use candid::Principal;
use ic_cdk::api::time;
use ic_cdk::api::msg_caller;
use std::result::Result;

use crate::types::*;

// User management
pub async fn create_user(profile: UserProfile) -> Result<Principal, Error> {
    let caller_principal = msg_caller();
    
    // Check if user already exists
    if crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
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
    let username_taken = crate::USERS.with(|users| {
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
    
    crate::USERS.with(|users| {
        users.borrow_mut().insert(caller_principal, user);
    });
    
    Ok(caller_principal)
}

pub fn get_user(principal: Principal) -> Option<User> {
    let caller_principal = msg_caller();
    
    // Users can only query their own data
    if caller_principal != principal {
        return None; // Return None instead of user data for security
    }
    
    crate::USERS.with(|users| users.borrow().get(&principal).cloned())
}

pub async fn update_profile(principal: Principal, profile_update: UserProfileUpdate) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    // Only allow users to update their own profile
    if caller_principal != principal {
        return Err(Error::Unauthorized);
    }
    
    crate::USERS.with(|users| {
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

pub async fn update_theme_preferences(theme_preferences: ThemePreferences) -> Result<(), Error> {
    let caller_principal = msg_caller();
    
    crate::USERS.with(|users| {
        if let Some(user) = users.borrow_mut().get_mut(&caller_principal) {
            user.profile.theme_preferences = Some(theme_preferences);
            user.updated_at = time();
            Ok(())
        } else {
            Err(Error::UserNotFound)
        }
    })
}

pub async fn update_username(principal: Principal, username: String) -> Result<(), Error> {
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
    
    crate::USERS.with(|users| {
        if let Some(user) = users.borrow_mut().get_mut(&principal) {
            user.profile.username = username;
            user.updated_at = time();
            Ok(())
        } else {
            Err(Error::UserNotFound)
        }
    })
}

pub fn get_users() -> Vec<User> {
    let caller_principal = msg_caller();
    
    // Ensure user exists
    if !crate::USERS.with(|users| users.borrow().contains_key(&caller_principal)) {
        return vec![]; // Return empty if not authenticated
    }
    
    // For now, return all users (in a real app, you'd filter by team membership, etc.)
    // This is a simplified version - in production you'd implement proper access control
    crate::USERS.with(|users| users.borrow().values().cloned().collect())
} 