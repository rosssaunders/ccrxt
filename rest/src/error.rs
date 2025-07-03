use thiserror::Error;

/// Common error type for REST clients
#[derive(Error, Debug)]
pub enum RestError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
