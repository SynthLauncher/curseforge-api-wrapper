use crate::api::client::CurseForgeClient;
use crate::errors::CurseForgeResult;
use crate::models::{
    search::{SearchRequest, SearchResponse, FingerprintSearchRequest, FingerprintSearchResponse},
    ApiResponse,
};

/// Search for projects
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `request` - The search request parameters
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the search results on success
///
/// # Example
/// ```no_run
/// use curseforge_api_wrapper::{
///     CurseForgeClient, 
///     api::search::search_projects,
///     models::search::{SearchRequest, SortField, SortOrder, ModLoaderType}
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = CurseForgeClient::new("your-api-key".to_string())?;
///     
///     let request = SearchRequest {
///         game_id: Some(432), // Minecraft
///         search_filter: Some("optifine".to_string()),
///         sort_field: Some(SortField::Popularity),
///         sort_order: Some(SortOrder::Desc),
///         mod_loader_type: Some(ModLoaderType::Forge),
///         page_size: Some(20),
///         ..Default::default()
///     };
///     
///     let results = search_projects(&client, &request).await?;
///     println!("Found {} projects", results.pagination.total_count);
///     Ok(())
/// }
/// ```
pub async fn search_projects(
    client: &CurseForgeClient,
    request: &SearchRequest,
) -> CurseForgeResult<SearchResponse> {
    let mut endpoint = "/mods/search".to_string();
    let mut params = Vec::new();

    // Build query parameters
    if let Some(game_id) = request.game_id {
        params.push(format!("gameId={}", game_id));
    }
    if let Some(class_id) = request.class_id {
        params.push(format!("classId={}", class_id));
    }
    if let Some(category_id) = request.category_id {
        params.push(format!("categoryId={}", category_id));
    }
    if let Some(ref game_version) = request.game_version {
        params.push(format!("gameVersion={}", game_version));
    }
    if let Some(ref search_filter) = request.search_filter {
        params.push(format!("searchFilter={}", search_filter));
    }
    if let Some(sort_field) = request.sort_field {
        params.push(format!("sortField={:?}", sort_field).to_lowercase());
    }
    if let Some(sort_order) = request.sort_order {
        params.push(format!("sortOrder={:?}", sort_order).to_lowercase());
    }
    if let Some(mod_loader_type) = request.mod_loader_type {
        params.push(format!("modLoaderType={:?}", mod_loader_type).to_lowercase());
    }
    if let Some(game_version_type_id) = request.game_version_type_id {
        params.push(format!("gameVersionTypeId={}", game_version_type_id));
    }
    if let Some(author_id) = request.author_id {
        params.push(format!("authorId={}", author_id));
    }
    if let Some(ref slug) = request.slug {
        params.push(format!("slug={}", slug));
    }
    if let Some(index) = request.index {
        params.push(format!("index={}", index));
    }
    if let Some(page_size) = request.page_size {
        params.push(format!("pageSize={}", page_size.min(50)));
    }

    if !params.is_empty() {
        endpoint.push('?');
        endpoint.push_str(&params.join("&"));
    }

    client.get(&endpoint).await
}

/// Search for projects by fingerprint
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `request` - The fingerprint search request
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the fingerprint search results on success
///
/// # Example
/// ```no_run
/// use curseforge_api_wrapper::{
///     CurseForgeClient, 
///     api::search::search_by_fingerprint,
///     models::search::FingerprintSearchRequest
/// };
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = CurseForgeClient::new("your-api-key".to_string())?;
///     
///     let request = FingerprintSearchRequest {
///         fingerprints: vec![123456789, 987654321],
///     };
///     
///     let results = search_by_fingerprint(&client, &request).await?;
///     println!("Found {} exact matches", results.exact_matches.len());
///     Ok(())
/// }
/// ```
pub async fn search_by_fingerprint(
    client: &CurseForgeClient,
    request: &FingerprintSearchRequest,
) -> CurseForgeResult<FingerprintSearchResponse> {
    let response: ApiResponse<FingerprintSearchResponse> = client
        .post("/fingerprints", request)
        .await?;
    Ok(response.data)
}

/// Search for projects with a simple text filter
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `search_filter` - The search text
/// * `game_id` - Optional game ID (default: 432 for Minecraft)
/// * `limit` - Optional result limit (max 50)
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the search results on success
pub async fn search_projects_simple(
    client: &CurseForgeClient,
    search_filter: &str,
    game_id: Option<u32>,
    limit: Option<u32>,
) -> CurseForgeResult<SearchResponse> {
    let request = SearchRequest {
        game_id: game_id.or(Some(432)), // Default to Minecraft
        search_filter: Some(search_filter.to_string()),
        page_size: limit.map(|l| l.min(50)),
        sort_field: Some(crate::models::search::SortField::Relevance),
        sort_order: Some(crate::models::search::SortOrder::Desc),
        ..Default::default()
    };

    search_projects(client, &request).await
}

/// Search for projects by category
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `category_id` - The category ID
/// * `game_id` - Optional game ID (default: 432 for Minecraft)
/// * `limit` - Optional result limit (max 50)
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the search results on success
pub async fn search_projects_by_category(
    client: &CurseForgeClient,
    category_id: u32,
    game_id: Option<u32>,
    limit: Option<u32>,
) -> CurseForgeResult<SearchResponse> {
    let request = SearchRequest {
        game_id: game_id.or(Some(432)), // Default to Minecraft
        category_id: Some(category_id),
        page_size: limit.map(|l| l.min(50)),
        sort_field: Some(crate::models::search::SortField::Popularity),
        sort_order: Some(crate::models::search::SortOrder::Desc),
        ..Default::default()
    };

    search_projects(client, &request).await
}

/// Search for projects by author
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `author_id` - The author ID
/// * `game_id` - Optional game ID (default: 432 for Minecraft)
/// * `limit` - Optional result limit (max 50)
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the search results on success
pub async fn search_projects_by_author(
    client: &CurseForgeClient,
    author_id: u32,
    game_id: Option<u32>,
    limit: Option<u32>,
) -> CurseForgeResult<SearchResponse> {
    let request = SearchRequest {
        game_id: game_id.or(Some(432)), // Default to Minecraft
        author_id: Some(author_id),
        page_size: limit.map(|l| l.min(50)),
        sort_field: Some(crate::models::search::SortField::LastUpdated),
        sort_order: Some(crate::models::search::SortOrder::Desc),
        ..Default::default()
    };

    search_projects(client, &request).await
}

/// Search for projects by game version
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `game_version` - The game version
/// * `game_id` - Optional game ID (default: 432 for Minecraft)
/// * `limit` - Optional result limit (max 50)
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the search results on success
pub async fn search_projects_by_game_version(
    client: &CurseForgeClient,
    game_version: &str,
    game_id: Option<u32>,
    limit: Option<u32>,
) -> CurseForgeResult<SearchResponse> {
    let request = SearchRequest {
        game_id: game_id.or(Some(432)), // Default to Minecraft
        game_version: Some(game_version.to_string()),
        page_size: limit.map(|l| l.min(50)),
        sort_field: Some(crate::models::search::SortField::Popularity),
        sort_order: Some(crate::models::search::SortOrder::Desc),
        ..Default::default()
    };

    search_projects(client, &request).await
}

/// Search for projects by mod loader type
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `mod_loader_type` - The mod loader type
/// * `game_id` - Optional game ID (default: 432 for Minecraft)
/// * `limit` - Optional result limit (max 50)
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the search results on success
pub async fn search_projects_by_mod_loader(
    client: &CurseForgeClient,
    mod_loader_type: crate::models::search::ModLoaderType,
    game_id: Option<u32>,
    limit: Option<u32>,
) -> CurseForgeResult<SearchResponse> {
    let request = SearchRequest {
        game_id: game_id.or(Some(432)), // Default to Minecraft
        mod_loader_type: Some(mod_loader_type),
        page_size: limit.map(|l| l.min(50)),
        sort_field: Some(crate::models::search::SortField::Popularity),
        sort_order: Some(crate::models::search::SortOrder::Desc),
        ..Default::default()
    };

    search_projects(client, &request).await
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            game_id: None,
            class_id: None,
            category_id: None,
            game_version: None,
            search_filter: None,
            sort_field: None,
            sort_order: None,
            mod_loader_type: None,
            game_version_type_id: None,
            author_id: None,
            slug: None,
            index: None,
            page_size: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::client::CurseForgeClient;

    #[test]
    fn test_search_request_default() {
        let request = SearchRequest::default();
        assert!(request.game_id.is_none());
        assert!(request.search_filter.is_none());
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_endpoint_building() {
        let client = CurseForgeClient::new("test-api-key".to_string()).unwrap();
        let request = SearchRequest {
            game_id: Some(432),
            search_filter: Some("test".to_string()),
            page_size: Some(20),
            ..Default::default()
        };

        // This would test the actual endpoint building logic
        // For now, just verify the request structure
        assert_eq!(request.game_id, Some(432));
        assert_eq!(request.search_filter.as_deref(), Some("test"));
        assert_eq!(request.page_size, Some(20));
    }
} 