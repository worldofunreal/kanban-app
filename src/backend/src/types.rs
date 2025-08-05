use candid::{CandidType, Deserialize};
use serde::Serialize;

// Type aliases
pub type UserId = String;
pub type TeamId = String;
pub type ProjectId = String;
pub type InviteId = String;
pub type Timestamp = u64;

// Theme preferences
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct ThemePreferences {
    pub color: String,      // color scheme name
    pub dark_mode: bool,    // true for dark, false for light
}

// User profile information
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct UserProfile {
    pub name: String,
    pub username: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub theme_preferences: Option<ThemePreferences>,
}

// Complete user data
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct User {
    pub id: UserId,
    pub profile: UserProfile,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

// User roles in teams and projects
#[derive(CandidType, Deserialize, Clone, Serialize, PartialEq, Eq)]
pub enum Role {
    Owner,
    Manager,
    Collaborator,
}

// Team member information
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct TeamMember {
    pub user_id: UserId,
    pub role: Role,
    pub joined_at: Timestamp,
}

// Team information
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct Team {
    pub id: TeamId,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub owner_id: UserId,
    pub members: Vec<TeamMember>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

// Project owner can be a user or team
#[derive(CandidType, Deserialize, Clone, Serialize, PartialEq, Eq)]
pub enum Owner {
    User(UserId),
    Team(TeamId),
}

// Project member information
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct ProjectMember {
    pub user_id: UserId,
    pub role: Role,
    pub joined_at: Timestamp,
}

// Project information
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub description: String,
    pub owner: Owner,
    pub members: Vec<ProjectMember>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

// Invitation target can be a team or project
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub enum InviteTarget {
    Team(TeamId),
    Project(ProjectId),
}

// Invitation information
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct Invite {
    pub id: InviteId,
    pub target: InviteTarget,
    pub role: Role,
    pub invited_by: UserId,
    pub expires_at: Option<Timestamp>,
    pub created_at: Timestamp,
}

// Update types for partial updates
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct UserProfileUpdate {
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<Option<String>>,
    pub avatar_url: Option<Option<String>>,
    pub bio: Option<Option<String>>,
    pub theme_preferences: Option<Option<ThemePreferences>>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct TeamUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(CandidType, Deserialize, Clone, Serialize)]
pub struct ProjectUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

// Error types
#[derive(CandidType, Deserialize, Clone, Serialize)]
pub enum Error {
    Unauthorized,
    NotFound,
    AlreadyExists,
    InvalidInput(String),
    InsufficientPermissions,
    InviteExpired,
    InviteNotFound,
    UserNotFound,
    TeamNotFound,
    ProjectNotFound,
    InternalError(String),
}

// Response types for queries
pub type UsersResponse = Vec<User>;
pub type TeamsResponse = Vec<Team>;
pub type ProjectsResponse = Vec<Project>;
pub type InvitesResponse = Vec<Invite>; 