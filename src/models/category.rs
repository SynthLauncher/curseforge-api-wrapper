use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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

/// Represents a category section
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

/// Represents a category class
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryClass {
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