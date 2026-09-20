use std::sync::Arc;

use bytes::Bytes;
use hyper::StatusCode;

/// Errors that can occur while making a request to the Discord API.
#[derive(Debug, thiserror::Error)]
pub enum RestError {
    #[error("failed to build the underlying HTTP request: {0}")]
    Build(#[from] hyper::http::Error),

    #[error("transport error while sending the request: {0}")]
    Hyper(#[from] hyper_util::client::legacy::Error),

    #[error("error while reading the response body: {0}")]
    Body(#[from] hyper::Error),

    #[error("request timed out")]
    Timeout,

    #[error("failed to deserialize the response body as JSON: {0}")]
    Deserialize(serde_json::Error),

    #[error("failed to serialize the request body as JSON: {0}")]
    Serialize(Arc<serde_json::Error>),

    #[error("discord returned an error response ({status}): {}", String::from_utf8_lossy(.body))]
    ApiError { status: StatusCode, body: Bytes },

    #[error("invalid header value: {0}")]
    InvalidHeader(#[from] hyper::header::InvalidHeaderValue),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error(
        "client received too many 401/403/429 responses and discord may temporarily block this IP; backing off"
    )]
    CloudflareBanRisk,
}

/// A parsed Discord API JSON error body, see <https://docs.discord.com/developers/reference#error-messages>.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DiscordApiError {
    pub code: i64,
    pub message: String,
    #[serde(default)]
    pub errors: Option<serde_json::Value>,
}

impl RestError {
    /// Try to parse the Discord error body on the [`RestError::ApiError`] variant.
    pub fn discord_error(&self) -> Option<DiscordApiError> {
        match self {
            RestError::ApiError { body, .. } => serde_json::from_slice(body).ok(),
            _ => None,
        }
    }
}
