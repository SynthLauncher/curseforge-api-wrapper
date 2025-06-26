use crate::api::client::CurseForgeClient;
use crate::errors::CurseForgeResult;
use crate::models::{
    project::{Project, ProjectDescription, ProjectDependency, ProjectDependencyType},
    ApiResponse, PaginatedResponse,
};
use std::path::Path;

/// Get a project by its ID
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `project_id` - The project ID
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the project data on success
///
/// # Example
/// ```no_run
/// use curseforge_api_wrapper::{CurseForgeClient, api::project::get_project};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = CurseForgeClient::new("your-api-key".to_string())?;
///     let project = get_project(&client, 12345).await?;
///     println!("Project: {:?}", project);
///     Ok(())
/// }
/// ```
pub async fn get_project(client: &CurseForgeClient, project_id: u32) -> CurseForgeResult<Project> {
    let endpoint = format!("/mods/{}", project_id);
    let response: ApiResponse<Project> = client.get(&endpoint).await?;
    Ok(response.data)
}

/// Get a project by its slug
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `slug` - The project slug
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the project data on success
pub async fn get_project_by_slug(client: &CurseForgeClient, slug: &str) -> CurseForgeResult<Project> {
    let endpoint = format!("/mods/search?gameId=432&slug={}", slug);
    let response: PaginatedResponse<Project> = client.get(&endpoint).await?;
    
    if response.data.is_empty() {
        return Err(crate::errors::CurseForgeError::NotFound(format!("Project with slug '{}' not found", slug)));
    }
    
    Ok(response.data[0].clone())
}

/// Get project description
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `project_id` - The project ID
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the project description on success
pub async fn get_project_description(
    client: &CurseForgeClient,
    project_id: u32,
) -> CurseForgeResult<ProjectDescription> {
    let endpoint = format!("/mods/{}/description", project_id);
    let response: ApiResponse<ProjectDescription> = client.get(&endpoint).await?;
    Ok(response.data)
}

/// Get project dependencies
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `project_id` - The project ID
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the project dependencies on success
pub async fn get_project_dependencies(
    client: &CurseForgeClient,
    project_id: u32,
) -> CurseForgeResult<Vec<ProjectDependency>> {
    let endpoint = format!("/mods/{}/dependencies", project_id);
    let response: ApiResponse<Vec<ProjectDependency>> = client.get(&endpoint).await?;
    Ok(response.data)
}

/// Get project files
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `project_id` - The project ID
/// * `game_version` - Optional game version filter
/// * `mod_loader_type` - Optional mod loader type filter
/// * `game_version_type_id` - Optional game version type ID filter
/// * `index` - Optional pagination index
/// * `page_size` - Optional page size (max 50)
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the project files on success
pub async fn get_project_files(
    client: &CurseForgeClient,
    project_id: u32,
    game_version: Option<&str>,
    mod_loader_type: Option<&str>,
    game_version_type_id: Option<u32>,
    index: Option<u32>,
    page_size: Option<u32>,
) -> CurseForgeResult<PaginatedResponse<crate::models::file::File>> {
    let mut endpoint = format!("/mods/{}/files", project_id);
    let mut params = Vec::new();

    if let Some(version) = game_version {
        params.push(format!("gameVersion={}", version));
    }
    if let Some(loader) = mod_loader_type {
        params.push(format!("modLoaderType={}", loader));
    }
    if let Some(type_id) = game_version_type_id {
        params.push(format!("gameVersionTypeId={}", type_id));
    }
    if let Some(idx) = index {
        params.push(format!("index={}", idx));
    }
    if let Some(size) = page_size {
        params.push(format!("pageSize={}", size.min(50)));
    }

    if !params.is_empty() {
        endpoint.push('?');
        endpoint.push_str(&params.join("&"));
    }

    client.get(&endpoint).await
}

/// Get a specific project file
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `project_id` - The project ID
/// * `file_id` - The file ID
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the file data on success
pub async fn get_project_file(
    client: &CurseForgeClient,
    project_id: u32,
    file_id: u32,
) -> CurseForgeResult<crate::models::file::File> {
    let endpoint = format!("/mods/{}/files/{}", project_id, file_id);
    let response: ApiResponse<crate::models::file::File> = client.get(&endpoint).await?;
    Ok(response.data)
}

/// Get project file changelog
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `project_id` - The project ID
/// * `file_id` - The file ID
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the file changelog on success
pub async fn get_project_file_changelog(
    client: &CurseForgeClient,
    project_id: u32,
    file_id: u32,
) -> CurseForgeResult<crate::models::file::FileChangelog> {
    let endpoint = format!("/mods/{}/files/{}/changelog", project_id, file_id);
    let response: ApiResponse<crate::models::file::FileChangelog> = client.get(&endpoint).await?;
    Ok(response.data)
}

/// Download a project file
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `file` - The file to download
/// * `destination` - The destination path
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the download path on success
pub async fn download_project_file(
    client: &CurseForgeClient,
    file: &crate::models::file::File,
    destination: &Path,
) -> CurseForgeResult<std::path::PathBuf> {
    let download_url = file.download_url.as_ref()
        .ok_or_else(|| crate::errors::CurseForgeError::DownloadFailed("No download URL available".to_string()))?;

    let file_path = destination.join(&file.file_name);
    client.download_file(download_url, &file_path).await?;
    Ok(file_path)
}

/// Get project dependencies
///
/// # Arguments
///
/// * `client` - The CurseForge client
/// * `project_id` - The project ID
/// * `file_id` - Optional file ID filter
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the project dependencies on success
pub async fn get_project_dependencies_with_file(
    client: &CurseForgeClient,
    project_id: u32,
    file_id: Option<u32>,
) -> CurseForgeResult<Vec<ProjectDependency>> {
    let mut endpoint = format!("/mods/{}/dependencies", project_id);
    
    if let Some(fid) = file_id {
        endpoint.push_str(&format!("?fileId={}", fid));
    }
    
    let response: ApiResponse<Vec<ProjectDependency>> = client.get(&endpoint).await?;
    Ok(response.data)
}

/// Get project dependency types
///
/// # Arguments
///
/// * `client` - The CurseForge client
///
/// # Returns
///
/// Returns a `CurseForgeResult` with the dependency types on success
pub async fn get_dependency_types(
    client: &CurseForgeClient,
) -> CurseForgeResult<Vec<ProjectDependencyType>> {
    let response: ApiResponse<Vec<ProjectDependencyType>> = client.get("/mods/dependency-types").await?;
    Ok(response.data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::client::CurseForgeClient;

    #[tokio::test]
    async fn test_get_project() {
        // This test requires a valid API key and would make actual API calls
        // For now, we'll just test the client creation
        let client = CurseForgeClient::new("test-api-key".to_string()).unwrap();
        assert_eq!(client.config().api_key, "test-api-key");
    }

    #[test]
    fn test_endpoint_formatting() {
        let endpoint = format!("/mods/{}", 12345);
        assert_eq!(endpoint, "/mods/12345");
    }
} 