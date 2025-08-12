// REST client for OKX public endpoints.
//
// Provides access to all public REST API endpoints for OKX Exchange.
// All requests are unauthenticated and do not require API credentials.
use std::borrow::Cow;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::okx::{EndpointType, Errors, RateLimiter, RestResult};

/// Public REST client for OKX exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RestClient {
    /// The base URL for the OKX public REST API (e.g., "https://www.okx.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    /// Guaranteed to end with no trailing slash for consistent URL building.
    base_url: Cow<'static, str>,

    /// Pre-formatted base URL with trailing slash for fast concatenation
    ///
    /// This avoids runtime string formatting in the hot path.
    formatted_base: String,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with OKX's rate limits for public endpoints.
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new OKX public REST client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the OKX public REST API (e.g., "https://www.okx.com")
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter for managing API limits
    pub fn new(
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: RateLimiter,
    ) -> Self {
        let base_url = base_url.into();
        // Pre-format the base URL with trailing slash for fast concatenation
        let formatted_base = format!("{}/", base_url.trim_end_matches('/'));

        Self {
            base_url,
            formatted_base,
            client,
            rate_limiter,
        }
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Send a GET request to a public endpoint (optimized for HFT)
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "api/v5/public/instruments")
    /// * `params` - Optional struct of query parameters (must implement Serialize)
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the response data or an error
    ///
    /// # Note
    /// All OKX public endpoints use GET, so this method is optimized specifically
    /// for GET requests with minimal branching for HFT performance.
    pub async fn send_get_request<T, P>(
        &self,
        endpoint: &str,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: serde::Serialize + ?Sized,
    {
        // Check rate limits before making the request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await
            .map_err(|e| Errors::Error(e.to_string()))?;

        // Build URL - branch-free for HFT optimization
        // Since public endpoints never include full URLs, we can always concatenate
        let url = format!("{}{}", self.formatted_base, endpoint);

        // Build the request - always GET for public endpoints
        let mut request_builder = self.client.get(&url);

        // Optimized parameter handling - all public endpoints use GET with query params
        // Single branch for params, no method checking needed
        if let Some(params) = params {
            // Pass params directly; the HTTP layer encodes serializable maps/structs into the query string
            request_builder = request_builder.query(params);
        }

        // Add required headers
        request_builder = request_builder.header("Content-Type", "application/json");

        // Send the request
        let response = request_builder.send().await.map_err(Errors::HttpError)?;

        // Increment rate limiter counter after successful request
        self.rate_limiter.increment_request(endpoint_type).await;

        // Get response status and body in one go to avoid multiple awaits
        let status = response.status();
        let response_text = response.text().await.map_err(Errors::HttpError)?;

        // Check status after getting text to avoid branching on success path
        if !status.is_success() {
            return Err(Errors::Error(format!("HTTP {status}: {response_text}")));
        }

        // Parse the response
        serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_client_creation() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://www.okx.com", client, rate_limiter);

        assert_eq!(rest_client.base_url(), "https://www.okx.com");
        assert_eq!(rest_client.formatted_base, "https://www.okx.com/");
    }

    #[test]
    fn test_url_building() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://www.okx.com/", client, rate_limiter);

        // Test that the client properly handles trailing slashes
        assert_eq!(rest_client.base_url(), "https://www.okx.com/");
        assert_eq!(rest_client.formatted_base, "https://www.okx.com/");
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new("https://www.okx.com", client, rate_limiter);

        // Test that rate limiting works (this shouldn't fail since we're not actually hitting limits)
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::PublicMarketData)
            .await;
        assert!(result.is_ok());
    }
}
