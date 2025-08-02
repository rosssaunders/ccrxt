use std::time::Duration;

use async_trait::async_trait;

use crate::{error::RestError, http_client::Method, rate_limiter::RateLimiter};

/// Common trait for venue-specific HTTP headers
pub trait VenueHeaders: Send + Sync + std::fmt::Debug {}

/// Common trait for venue-specific error responses
pub trait VenueErrorResponse: Send + Sync + std::fmt::Debug {
    /// Get the error code from the response
    fn code(&self) -> i32;

    /// Get the error message from the response
    fn message(&self) -> String;
}

/// Common trait for venue-specific REST responses
#[derive(Debug, Clone)]
pub struct RestResponse<T, H, E>
where
    T: Send + Sync + std::fmt::Debug,
    H: VenueHeaders,
    E: VenueErrorResponse,
{
    /// The actual data payload from the response
    pub data: T,

    /// Time spent waiting on rate limiting
    pub rate_limit_duration: Duration,

    /// Total time spent on the request
    pub request_duration: Duration,

    /// Venue-specific HTTP headers
    pub headers: H,

    /// Venue-specific error response if the request failed
    pub error: Option<E>,
}

/// Common trait for venue-specific REST clients
#[async_trait]
pub trait RestClient<T, H, E>
where
    T: Send + Sync + std::fmt::Debug,
    H: VenueHeaders,
    E: VenueErrorResponse,
{
    /// Make a REST request and return a RestResponse
    async fn request(
        &self,
        endpoint: &str,
        method: Method,
        query: Option<&str>,
    ) -> Result<RestResponse<T, H, E>, RestError>;

    /// Get the base URL for the REST API
    fn base_url(&self) -> &str;

    /// Get the rate limiter for the client
    fn rate_limiter(&self) -> &dyn RateLimiter;
}
