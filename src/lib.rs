use std::env;

/// Loads the CurseForge API key from the environment (using dotenv if available)
pub fn load_api_key() -> String {
    dotenv::dotenv().ok();
    env::var("CURSEFORGE_API_KEY").expect("CURSEFORGE_API_KEY must be set in .env or environment")
}

pub mod api;
pub mod models;
pub mod errors;
