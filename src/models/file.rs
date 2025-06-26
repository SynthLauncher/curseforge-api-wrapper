use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::{ReleaseType, FileStatus, FileHash, FileDependency, FileModule};

/// Represents a CurseForge file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
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

/// Represents a file changelog
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileChangelog {
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

/// Represents a file description
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileDescription {
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