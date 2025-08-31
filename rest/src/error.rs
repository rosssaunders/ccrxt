use thiserror::Error;

/// Common error type for REST clients
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RestError {
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
