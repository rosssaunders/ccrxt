//! BitMart error types for futures endpoints
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::borrow::Cow;

/// Error response struct as returned by BitMart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: Cow<'static, str>,
}

/// BitMart error enum
#[derive(Debug, Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BitmartError {
    /// Not Found - The requested interface could not be found
    #[error("Not Found: {0}")]
    NotFound(Cow<'static, str>),
    /// Forbidden - No permission to access the resource
    #[error("Forbidden: {0}")]
    Forbidden(Cow<'static, str>),
    /// Unauthorized - Authentication failed
    #[error("Unauthorized: {0}")]
    Unauthorized(Cow<'static, str>),
    /// Internal Server Error - BitMart service problem
    #[error("Internal Server Error: {0}")]
    InternalServerError(Cow<'static, str>),
    /// Other error
    #[error("Other error: {0}")]
    Other(i32, Cow<'static, str>),
}

impl From<ErrorResponse> for BitmartError {
    fn from(resp: ErrorResponse) -> Self {
        match resp.code {
            404 => BitmartError::NotFound(resp.message),
            403 => BitmartError::Forbidden(resp.message),
            401 => BitmartError::Unauthorized(resp.message),
            500 => BitmartError::InternalServerError(resp.message),
            code => BitmartError::Other(code, resp.message),
        }
    }
}

/// BitMart result type alias
pub type Result<T> = std::result::Result<T, BitmartError>;
