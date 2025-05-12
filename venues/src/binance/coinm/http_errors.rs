use thiserror::Error;
use reqwest::StatusCode;
use reqwest::Response;

/// HTTP-related errors for Binance Coin-M Futures API
#[derive(Error, Debug)]
pub enum BinanceCoinMHttpError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl From<StatusCode> for BinanceCoinMHttpError {
    fn from(status: StatusCode) -> Self {
        match status {
            StatusCode::UNAUTHORIZED => BinanceCoinMHttpError::AuthenticationError("Invalid API key or signature".to_string()),
            StatusCode::TOO_MANY_REQUESTS => BinanceCoinMHttpError::RateLimitExceeded,
            StatusCode::BAD_REQUEST => BinanceCoinMHttpError::InvalidResponse("Bad request".to_string()),
            StatusCode::FORBIDDEN => BinanceCoinMHttpError::AuthenticationError("Access forbidden".to_string()),
            StatusCode::NOT_FOUND => BinanceCoinMHttpError::InvalidResponse("Resource not found".to_string()),
            StatusCode::INTERNAL_SERVER_ERROR => BinanceCoinMHttpError::InvalidResponse("Internal server error".to_string()),
            StatusCode::SERVICE_UNAVAILABLE => BinanceCoinMHttpError::InvalidResponse("Service unavailable".to_string()),
            _ => BinanceCoinMHttpError::InvalidResponse(format!("Unexpected status code: {}", status)),
        }
    }
}

impl BinanceCoinMHttpError {
    pub async fn from_response(response: Response) -> Self {
        match response.status().as_u16() {
            429 => BinanceCoinMHttpError::RateLimitExceeded,
            401 | 403 => BinanceCoinMHttpError::AuthenticationError("Invalid API key or signature".to_string()),
            400 => BinanceCoinMHttpError::InvalidResponse("Invalid request parameters".to_string()),
            _ => BinanceCoinMHttpError::InvalidResponse(format!("Unexpected status code: {}", response.status())),
        }
    }
}

pub type BinanceCoinMHttpResult<T> = Result<T, BinanceCoinMHttpError>; 