use crate::errors::{CurseForgeError, CurseForgeResult};
use reqwest::{Client, ClientBuilder, header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT}};
use std::time::Duration;
use url::Url;

/// Configuration for the CurseForge API client
#[derive(Debug, Clone)]
pub struct CurseForgeConfig {
    /// CurseForge API key
    pub api_key: String,
    /// Base URL for the CurseForge API
    pub base_url: String,
    /// Request timeout
    pub timeout: Duration,
    /// User agent string
    pub user_agent: String,
    /// Maximum retries for failed requests
    pub max_retries: u32,
    /// Retry delay between attempts
    pub retry_delay: Duration,
}

impl Default for CurseForgeConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.curseforge.com/v1".to_string(),
            timeout: Duration::from_secs(30),
            user_agent: "curseforge-api-wrapper/0.1.0".to_string(),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
        }
    }
}

/// CurseForge API client
#[derive(Debug, Clone)]
pub struct CurseForgeClient {
    client: Client,
    config: CurseForgeConfig,
}

impl CurseForgeClient {
    /// Create a new CurseForge client with the given API key
    pub fn new(api_key: String) -> CurseForgeResult<Self> {
        Self::with_config(CurseForgeConfig {
            api_key,
            ..Default::default()
        })
    }

    /// Create a new CurseForge client with custom configuration
    pub fn with_config(config: CurseForgeConfig) -> CurseForgeResult<Self> {
        if config.api_key.is_empty() {
            return Err(CurseForgeError::InvalidApiKey);
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.api_key))
                .map_err(|_| CurseForgeError::InvalidApiKey)?,
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&config.user_agent)
                .map_err(|_| CurseForgeError::InvalidParameters("Invalid user agent".to_string()))?,
        );

        let client = ClientBuilder::new()
            .timeout(config.timeout)
            .default_headers(headers)
            .build()
            .map_err(CurseForgeError::Request)?;

        Ok(Self { client, config })
    }

    /// Get a reference to the underlying HTTP client
    pub fn http_client(&self) -> &Client {
        &self.client
    }

    /// Get a mutable reference to the underlying HTTP client
    pub fn http_client_mut(&mut self) -> &mut Client {
        &mut self.client
    }

    /// Get the configuration
    pub fn config(&self) -> &CurseForgeConfig {
        &self.config
    }

    /// Get a mutable reference to the configuration
    pub fn config_mut(&mut self) -> &mut CurseForgeConfig {
        &mut self.config
    }

    /// Build a URL for the given endpoint
    pub fn build_url(&self, endpoint: &str) -> CurseForgeResult<Url> {
        let url = format!("{}{}", self.config.base_url, endpoint);
        Url::parse(&url).map_err(CurseForgeError::Url)
    }

    /// Make a GET request with retry logic
    pub async fn get<T>(&self, endpoint: &str) -> CurseForgeResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.request_with_retry(|client| {
            let url = self.build_url(endpoint)?;
            Ok(client.get(url))
        })
        .await
    }

    /// Make a POST request with retry logic
    pub async fn post<T, B>(&self, endpoint: &str, body: &B) -> CurseForgeResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        self.request_with_retry(|client| {
            let url = self.build_url(endpoint)?;
            Ok(client.post(url).json(body))
        })
        .await
    }

    /// Make a request with retry logic
    async fn request_with_retry<T, F>(&self, request_builder: F) -> CurseForgeResult<T>
    where
        T: serde::de::DeserializeOwned,
        F: Fn(&Client) -> CurseForgeResult<reqwest::RequestBuilder>,
    {
        let mut last_error = None;
        let mut attempt = 0;

        while attempt <= self.config.max_retries {
            match self.make_request(&request_builder).await {
                Ok(response) => return Ok(response),
                Err(error) => {
                    last_error = Some(error.clone());
                    
                    if !error.is_retryable() || attempt == self.config.max_retries {
                        break;
                    }

                    attempt += 1;
                    tokio::time::sleep(self.config.retry_delay).await;
                }
            }
        }

        Err(last_error.unwrap_or_else(|| CurseForgeError::Unknown("Request failed".to_string())))
    }

    /// Make a single request
    async fn make_request<T, F>(&self, request_builder: &F) -> CurseForgeResult<T>
    where
        T: serde::de::DeserializeOwned,
        F: Fn(&Client) -> CurseForgeResult<reqwest::RequestBuilder>,
    {
        let request = request_builder(&self.client)?;
        let response = request.send().await.map_err(CurseForgeError::Request)?;

        let status = response.status();
        
        if status.is_success() {
            let data = response.json::<T>().await.map_err(CurseForgeError::Request)?;
            Ok(data)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            match status.as_u16() {
                401 => Err(CurseForgeError::AuthenticationRequired),
                403 => Err(CurseForgeError::PermissionDenied(error_text)),
                404 => Err(CurseForgeError::NotFound(error_text)),
                429 => Err(CurseForgeError::RateLimitExceeded),
                400..=499 => Err(CurseForgeError::InvalidParameters(error_text)),
                500..=599 => Err(CurseForgeError::Api {
                    status: status.as_u16(),
                    message: error_text,
                }),
                _ => Err(CurseForgeError::Api {
                    status: status.as_u16(),
                    message: error_text,
                }),
            }
        }
    }

    /// Download a file to the given path
    pub async fn download_file(&self, url: &str, path: &std::path::Path) -> CurseForgeResult<()> {
        let response = self.client.get(url).send().await.map_err(CurseForgeError::Request)?;
        
        if !response.status().is_success() {
            return Err(CurseForgeError::DownloadFailed(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().await.unwrap_or_else(|_| "Unknown error".to_string())
            )));
        }

        let mut file = std::fs::File::create(path).map_err(CurseForgeError::Io)?;
        let mut stream = response.bytes_stream();
        
        use futures_util::StreamExt;
        use std::io::Write;
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(CurseForgeError::Request)?;
            file.write_all(&chunk).map_err(CurseForgeError::Io)?;
        }

        Ok(())
    }

    /// Upload a file
    pub async fn upload_file<T>(&self, endpoint: &str, file_path: &std::path::Path) -> CurseForgeResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let file = tokio::fs::File::open(file_path).await.map_err(CurseForgeError::Io)?;
        let file_size = file.metadata().await.map_err(CurseForgeError::Io)?.len();
        
        // Check file size limit (assuming 100MB limit)
        const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;
        if file_size > MAX_FILE_SIZE {
            return Err(CurseForgeError::FileTooLarge {
                size: file_size,
                max: MAX_FILE_SIZE,
            });
        }

        let url = self.build_url(endpoint)?;
        let response = self.client
            .post(url)
            .body(file)
            .send()
            .await
            .map_err(CurseForgeError::Request)?;

        if response.status().is_success() {
            let data = response.json::<T>().await.map_err(CurseForgeError::Request)?;
            Ok(data)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(CurseForgeError::UploadFailed(error_text))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = CurseForgeConfig::default();
        assert_eq!(config.base_url, "https://api.curseforge.com/v1");
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_client_new_with_empty_api_key() {
        let result = CurseForgeClient::new(String::new());
        assert!(matches!(result, Err(CurseForgeError::InvalidApiKey)));
    }

    #[test]
    fn test_client_new_with_valid_api_key() {
        let result = CurseForgeClient::new("test-api-key".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_url() {
        let client = CurseForgeClient::new("test-api-key".to_string()).unwrap();
        let url = client.build_url("/test").unwrap();
        assert_eq!(url.as_str(), "https://api.curseforge.com/v1/test");
    }
} 