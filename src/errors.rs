use thiserror::Error;

/// Represents all possible errors that can occur when using the CurseForge API wrapper
#[derive(Error, Debug)]
pub enum CurseForgeError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL parsing failed
    #[error("URL parsing failed: {0}")]
    Url(#[from] url::ParseError),

    /// IO operation failed
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// API returned an error response
    #[error("API error: {status} - {message}")]
    Api {
        status: u16,
        message: String,
    },

    /// Invalid API key
    #[error("Invalid API key")]
    InvalidApiKey,

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Try again later.")]
    RateLimitExceeded,

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Invalid parameters provided
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    /// Authentication required
    #[error("Authentication required")]
    AuthenticationRequired,

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// File download failed
    #[error("File download failed: {0}")]
    DownloadFailed(String),

    /// File upload failed
    #[error("File upload failed: {0}")]
    UploadFailed(String),

    /// Invalid file format
    #[error("Invalid file format: {0}")]
    InvalidFileFormat(String),

    /// File too large
    #[error("File too large: {size} bytes (max: {max} bytes)")]
    FileTooLarge {
        size: u64,
        max: u64,
    },

    /// Network timeout
    #[error("Network timeout after {timeout:?}")]
    Timeout {
        timeout: std::time::Duration,
    },

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Clone for CurseForgeError {
    fn clone(&self) -> Self {
        match self {
            CurseForgeError::Api { status, message } =>
                CurseForgeError::Api { status: *status, message: message.clone() },
            CurseForgeError::InvalidApiKey => CurseForgeError::InvalidApiKey,
            CurseForgeError::RateLimitExceeded => CurseForgeError::RateLimitExceeded,
            CurseForgeError::NotFound(msg) => CurseForgeError::NotFound(msg.clone()),
            CurseForgeError::InvalidParameters(msg) => CurseForgeError::InvalidParameters(msg.clone()),
            CurseForgeError::AuthenticationRequired => CurseForgeError::AuthenticationRequired,
            CurseForgeError::PermissionDenied(msg) => CurseForgeError::PermissionDenied(msg.clone()),
            CurseForgeError::DownloadFailed(msg) => CurseForgeError::DownloadFailed(msg.clone()),
            CurseForgeError::UploadFailed(msg) => CurseForgeError::UploadFailed(msg.clone()),
            CurseForgeError::InvalidFileFormat(msg) => CurseForgeError::InvalidFileFormat(msg.clone()),
            CurseForgeError::FileTooLarge { size, max } => CurseForgeError::FileTooLarge { size: *size, max: *max },
            CurseForgeError::Timeout { timeout } => CurseForgeError::Timeout { timeout: *timeout },
            CurseForgeError::Unknown(msg) => CurseForgeError::Unknown(msg.clone()),
            // For non-cloneable errors, fallback to Unknown
            CurseForgeError::Request(_) => CurseForgeError::Unknown("Request error (not cloneable)".to_string()),
            CurseForgeError::Json(_) => CurseForgeError::Unknown("JSON error (not cloneable)".to_string()),
            CurseForgeError::Url(_) => CurseForgeError::Unknown("URL error (not cloneable)".to_string()),
            CurseForgeError::Io(_) => CurseForgeError::Unknown("IO error (not cloneable)".to_string()),
        }
    }
}

impl CurseForgeError {
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            CurseForgeError::Request(_) | CurseForgeError::Timeout { .. } | CurseForgeError::RateLimitExceeded
        )
    }

    /// Get the HTTP status code if available
    pub fn status_code(&self) -> Option<u16> {
        match self {
            CurseForgeError::Api { status, .. } => Some(*status),
            CurseForgeError::Request(e) => e.status().map(|s| s.as_u16()),
            _ => None,
        }
    }
}

/// Result type for CurseForge API operations
pub type CurseForgeResult<T> = Result<T, CurseForgeError>;

/// Helper trait for converting other error types to CurseForgeError
pub trait IntoCurseForgeError<T> {
    fn into_curseforge_error(self) -> CurseForgeResult<T>;
}

impl<T, E> IntoCurseForgeError<T> for Result<T, E>
where
    E: Into<CurseForgeError>,
{
    fn into_curseforge_error(self) -> CurseForgeResult<T> {
        self.map_err(Into::into)
    }
}
