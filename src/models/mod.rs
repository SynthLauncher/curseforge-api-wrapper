use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub mod project;
pub mod file;
pub mod category;
pub mod game;
pub mod search;
pub mod fingerprint;

/// Represents the status of a CurseForge project
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Approved,
    Rejected,
    Draft,
    Unknown,
}

/// Represents the category section of a CurseForge project
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CategorySection {
    Mods,
    Modpacks,
    ResourcePacks,
    Worlds,
    Customization,
    Tools,
    BukkitPlugins,
    Unknown,
}

/// Represents the release type of a file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseType {
    Release,
    Beta,
    Alpha,
}

/// Represents the file status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileStatus {
    Processing,
    ChangesRequired,
    UnderReview,
    Approved,
    Rejected,
    MalwareDetected,
    Deleted,
    Archived,
    Testing,
    ApprovedByThirdParty,
    ReadyForReview,
    Deprecated,
    Baking,
    PendingProject,
    Unknown,
}

/// Represents the relationship type between projects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationType {
    EmbeddedLibrary,
    OptionalDependency,
    RequiredDependency,
    Tool,
    Incompatible,
    Include,
}

/// Represents a CurseForge user
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub twitch_id: Option<u32>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

/// Represents a CurseForge category
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Category {
    pub id: u32,
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub url: String,
    pub icon_url: Option<String>,
    pub date_modified: DateTime<Utc>,
    pub is_class: bool,
    pub class_id: Option<u32>,
    pub parent_category_id: Option<u32>,
    pub display_index: u32,
}

/// Represents a CurseForge game
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Game {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub date_modified: DateTime<Utc>,
    pub assets: Option<GameAssets>,
    pub status: u32,
    pub api_status: u32,
}

/// Represents game assets
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameAssets {
    pub icon_url: Option<String>,
    pub tile_url: Option<String>,
    pub cover_image: Option<String>,
}

/// Represents a CurseForge API response wrapper
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

/// Represents a paginated API response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

/// Represents pagination information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pagination {
    pub index: u32,
    pub page_size: u32,
    pub result_count: u32,
    pub total_count: u32,
}

/// Represents a file hash for fingerprinting
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileHash {
    pub value: String,
    pub algo: u32,
}

/// Represents a file dependency
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileDependency {
    pub mod_id: u32,
    pub relation_type: RelationType,
}

/// Represents a file module
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileModule {
    pub name: String,
    pub fingerprint: u64,
} 