mod errors;

// Re-export errors module
pub use errors::{Errors, ApiError, ErrorResponse};

/// Type alias for results returned by Crypto.com API operations
pub type RestResult<T> = Result<T, Errors>;