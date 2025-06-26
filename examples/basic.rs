use curseforge_api_wrapper::{load_api_key, api::client::CurseForgeClient, api::project::get_project};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from .env or environment
    let api_key = load_api_key();
    let client = CurseForgeClient::new(api_key)?;

    // Example: Fetch a Minecraft mod project by ID (e.g., 238222 is JEI)
    let project_id = 388172;
    let project = get_project(&client, project_id).await?;
    println!("Project name: {}", project.name);
    println!("Project summary: {}", project.summary);
    Ok(())
} 