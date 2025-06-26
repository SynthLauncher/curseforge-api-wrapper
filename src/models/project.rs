use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::{ProjectStatus, Category, FileDependency, FileModule};

/// Represents a CurseForge project
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub id: u32,
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub links: ProjectLinks,
    pub summary: String,
    pub status: ProjectStatus,
    pub download_count: u64,
    pub is_featured: bool,
    pub primary_category_id: u32,
    pub categories: Vec<Category>,
    pub class_id: Option<u32>,
    pub authors: Vec<ProjectAuthor>,
    pub logo: Option<ProjectLogo>,
    pub screenshots: Vec<ProjectScreenshot>,
    pub main_file_id: u32,
    pub latest_files: Vec<ProjectFile>,
    pub latest_file_indexes: Vec<ProjectFileIndex>,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub date_released: DateTime<Utc>,
    pub allow_mod_distribution: Option<bool>,
    pub game_popularity_rank: u32,
    pub is_available: bool,
    pub thumbs_up: u32,
}

/// Represents project links
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectLinks {
    pub website_url: Option<String>,
    pub wiki_url: Option<String>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
}

/// Represents a project author
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectAuthor {
    pub id: u32,
    pub name: String,
    pub url: String,
}

/// Represents a project logo
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectLogo {
    pub id: u32,
    pub mod_id: u32,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail_url: String,
    pub url: String,
}

/// Represents a project screenshot
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectScreenshot {
    pub id: u32,
    pub mod_id: u32,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail_url: String,
    pub url: String,
}

/// Represents a project file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectFile {
    pub id: u32,
    pub display_name: String,
    pub file_name: String,
    pub file_date: DateTime<Utc>,
    pub file_length: u64,
    pub download_count: u64,
    pub download_url: Option<String>,
    pub game_versions: Vec<String>,
    pub sortable_game_versions: Vec<SortableGameVersion>,
    pub dependencies: Vec<FileDependency>,
    pub expose_as_alternative: Option<bool>,
    pub parent_project_file_id: Option<u32>,
    pub alternate_file_id: Option<u32>,
    pub is_available: bool,
    pub modules: Vec<FileModule>,
    pub package_fingerprint: u64,
    pub game_version_date_released: DateTime<Utc>,
    pub game_version_map: Vec<GameVersionMap>,
    pub install_metadata: Option<InstallMetadata>,
    pub changelog: Option<String>,
    pub has_install_script: bool,
    pub is_compatible_with_client: bool,
    pub category_section_package_type: u32,
    pub restrict_project_file_access: u32,
    pub project_status: u32,
    pub render_cache_id: Option<u32>,
    pub file_legacy_mapping_id: Option<u32>,
    pub project_id: u32,
    pub parent_project_id: Option<u32>,
    pub parent_file_legacy_mapping_id: Option<u32>,
    pub file_type_id: Option<u32>,
    pub package_fingerprint_id: u64,
    pub game_version_mapping_file_type: u32,
    pub game_version_mapping_type: u32,
    pub game_id: u32,
    pub is_server_pack: bool,
    pub server_pack_file_id: Option<u32>,
    pub game_display_name: String,
    pub sync: bool,
}

/// Represents a sortable game version
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SortableGameVersion {
    pub game_version_padded: String,
    pub game_version: String,
    pub game_version_release_date: DateTime<Utc>,
    pub game_version_name: String,
}

/// Represents a game version map
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameVersionMap {
    pub game_version_id: u32,
    pub game_version_name: String,
    pub game_version_type_id: Option<u32>,
    pub game_version_status: u32,
    pub game_version_type_status: u32,
}

/// Represents install metadata
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstallMetadata {
    pub folder_name: Option<String>,
    pub rename: Option<String>,
    pub file_id: u32,
}

/// Represents a project file index
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectFileIndex {
    pub game_version: String,
    pub file_id: u32,
    pub filename: String,
    pub release_type: u32,
    pub game_version_type_id: Option<u32>,
    pub mod_loader: Option<u32>,
}

/// Represents a project search result
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectSearchResult {
    pub id: u32,
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub links: ProjectLinks,
    pub summary: String,
    pub status: ProjectStatus,
    pub download_count: u64,
    pub is_featured: bool,
    pub primary_category_id: u32,
    pub categories: Vec<Category>,
    pub class_id: Option<u32>,
    pub authors: Vec<ProjectAuthor>,
    pub logo: Option<ProjectLogo>,
    pub screenshots: Vec<ProjectScreenshot>,
    pub main_file_id: u32,
    pub latest_files: Vec<ProjectFile>,
    pub latest_file_indexes: Vec<ProjectFileIndex>,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub date_released: DateTime<Utc>,
    pub allow_mod_distribution: Option<bool>,
    pub game_popularity_rank: u32,
    pub is_available: bool,
    pub thumbs_up: u32,
    pub root_id: Option<u32>,
    pub mod_id: Option<u32>,
    pub game_name: String,
    pub logo_url: Option<String>,
    pub author: String,
    pub display_categories: Vec<Category>,
    pub attachments: Vec<ProjectAttachment>,
    pub latest_early_access_files_indexes: Vec<ProjectFileIndex>,
    pub date_updated: DateTime<Utc>,
    pub latest_early_access_files: Vec<ProjectFile>,
    pub is_experimental: bool,
}

/// Represents a project attachment
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectAttachment {
    pub id: u32,
    pub project_id: u32,
    pub description: Option<String>,
    pub is_default: bool,
    pub thumbnail_url: String,
    pub title: String,
    pub url: String,
    pub status: u32,
}

/// Represents a project description
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectDescription {
    pub id: u32,
    pub description: String,
    pub links: ProjectLinks,
}

/// Represents a project dependency
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectDependency {
    pub id: u32,
    pub addon_id: u32,
    pub type_id: u32,
    pub file_id: Option<u32>,
}

/// Represents a project dependency type
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectDependencyType {
    pub id: u32,
    pub name: String,
} 