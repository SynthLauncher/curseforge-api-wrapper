use crate::api::client::CurseForgeClient;
use crate::errors::CurseForgeResult;
use crate::models::{category::Category, ApiResponse};

/// Get all categories
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `game_id` - The game ID (default: 432 for Minecraft)
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the categories on success
///
/// # Example
/// ```no_run
/// use curseforge_api_wrapper::{CurseForgeClient, api::category::get_categories};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = CurseForgeClient::new("your-api-key".to_string())?;
///     let categories = get_categories(&client, Some(432)).await?;
///     println!("Found {} categories", categories.len());
///     Ok(())
/// }
/// ```
pub async fn get_categories(
    client: &CurseForgeClient,
    game_id: Option<u32>,
) -> CurseForgeResult<Vec<Category>> {
    let mut endpoint = "/categories".to_string();
    
    if let Some(id) = game_id {
        endpoint.push_str(&format!("?gameId={}", id));
    }
    
    let response: ApiResponse<Vec<Category>> = client.get(&endpoint).await?;
    Ok(response.data)
}

/// Get categories for Minecraft (game ID 432)
///
/// # Arguments
///
/// * `client` - The CurseForge client
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the Minecraft categories on success
pub async fn get_minecraft_categories(client: &CurseForgeClient) -> CurseForgeResult<Vec<Category>> {
    get_categories(client, Some(432)).await
}

/// Get a category by ID
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `category_id` - The category ID
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the category on success
pub async fn get_category(
    client: &CurseForgeClient,
    category_id: u32,
) -> CurseForgeResult<Category> {
    let endpoint = format!("/categories/{}", category_id);
    let response: ApiResponse<Category> = client.get(&endpoint).await?;
    Ok(response.data)
}

/// Get categories by class ID
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `class_id` - The class ID
/// * `game_id` - Optional game ID
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the categories on success
pub async fn get_categories_by_class(
    client: &CurseForgeClient,
    class_id: u32,
    game_id: Option<u32>,
) -> CurseForgeResult<Vec<Category>> {
    let mut endpoint = format!("/categories?classId={}", class_id);
    
    if let Some(id) = game_id {
        endpoint.push_str(&format!("&gameId={}", id));
    }
    
    let response: ApiResponse<Vec<Category>> = client.get(&endpoint).await?;
    Ok(response.data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::client::CurseForgeClient;

    #[test]
    fn test_endpoint_formatting() {
        let endpoint = format!("/categories/{}", 123);
        assert_eq!(endpoint, "/categories/123");
    }

    #[test]
    fn test_client_creation() {
        let client = CurseForgeClient::new("test-api-key".to_string()).unwrap();
        assert_eq!(client.config().api_key, "test-api-key");
    }
} 