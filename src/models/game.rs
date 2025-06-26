use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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

/// Represents a game version
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameVersion {
    pub id: u32,
    pub game_version_type_id: u32,
    pub name: String,
    pub slug: String,
    pub date_modified: DateTime<Utc>,
    pub game_version_status: u32,
    pub game_version_type_status: u32,
}

/// Represents a game version type
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameVersionType {
    pub id: u32,
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub date_modified: DateTime<Utc>,
    pub game_version_status: u32,
}

/// Represents a game version status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GameVersionStatus {
    Approved,
    Rejected,
    Draft,
    Unknown,
} 