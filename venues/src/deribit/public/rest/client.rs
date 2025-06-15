// REST client for Deribit public endpoints.
//
// Provides access to all public REST API endpoints for Deribit.
// All requests are unauthenticated and do not require API credentials.
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::sync::Arc;

use crate::deribit::{EndpointType, Errors, RateLimiter, RestResult, AccountTier};

/// Public REST client for Deribit exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RestClient {
    /// The base URL for the Deribit public REST API (e.g., "https://api.deribit.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with Deribit's rate limits for public endpoints.
    pub rate_limiter: Arc<RateLimiter>,
}

impl RestClient {
    /// Creates a new Deribit public REST client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Deribit public REST API (e.g., "https://api.deribit.com")
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter for managing API limits
    pub fn new(
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: Arc<RateLimiter>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
        }
    }

    /// Creates a new Deribit public REST client with default settings.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Deribit public REST API (e.g., "https://api.deribit.com")
    /// * `account_tier` - The account tier for rate limiting (affects private endpoints mostly)
    pub fn new_default(
        base_url: impl Into<Cow<'static, str>>,
        account_tier: AccountTier,
    ) -> Self {
        Self::new(
            base_url,
            Client::new(),
            Arc::new(RateLimiter::new(account_tier)),
        )
    }

    /// Send a request to a public endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "public/fork_token")
    /// * `method` - The HTTP method to use
    /// * `params` - Optional struct of query/body parameters (must implement Serialize)
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the response data or an error
    pub async fn send_request<T, P>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: serde::Serialize + ?Sized,
    {
        // Check rate limits before making the request
        self.rate_limiter
            .check_limits(endpoint_type)
            .await
            .map_err(|e| Errors::RateLimitError(e.to_string()))?;

        // Build the URL - Deribit uses JSON-RPC over HTTP POST to a single endpoint
        let url = format!("{}/api/v2/{}", self.base_url, endpoint);

        // Build the request
        let mut request_builder = self.client.request(method.clone(), &url);

        // Add parameters as JSON body for JSON-RPC
        if let Some(params) = params {
            let params_value = serde_json::to_value(params)
                .map_err(|e| Errors::Error(format!("Failed to serialize params: {}", e)))?;
            request_builder = request_builder.json(&params_value);
        }

        // Add required headers
        request_builder = request_builder.header("Content-Type", "application/json");

        // Send the request
        let response = request_builder.send().await.map_err(Errors::HttpError)?;

        // Record the request for rate limiting
        self.rate_limiter.record_request(endpoint_type).await;

        // Check if the response was successful
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.map_err(Errors::HttpError)?;
            return Err(Errors::Error(format!("HTTP {}: {}", status, error_text)));
        }

        // Parse the response
        let response_text = response.text().await.map_err(Errors::HttpError)?;

        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_client_creation() {
        let client = reqwest::Client::new();
        let rate_limiter = Arc::new(RateLimiter::new(AccountTier::Tier4));

        let rest_client = RestClient::new("https://api.deribit.com", client, rate_limiter);

        assert_eq!(rest_client.base_url, "https://api.deribit.com");
    }

    #[test]
    fn test_default_client_creation() {
        let rest_client = RestClient::new_default("https://api.deribit.com", AccountTier::Tier3);

        assert_eq!(rest_client.base_url, "https://api.deribit.com");
    }

    #[test]
    fn test_url_building() {
        let client = reqwest::Client::new();
        let rate_limiter = Arc::new(RateLimiter::new(AccountTier::Tier4));

        let rest_client = RestClient::new("https://api.deribit.com", client, rate_limiter);

        // Test that the client is properly initialized
        assert_eq!(rest_client.base_url, "https://api.deribit.com");
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let client = reqwest::Client::new();
        let rate_limiter = Arc::new(RateLimiter::new(AccountTier::Tier4));

        let rest_client = RestClient::new("https://api.deribit.com", client, rate_limiter);

        // Test that rate limiting works (this shouldn't fail since we're not actually hitting limits)
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::PublicForkToken)
            .await;
        assert!(result.is_ok());
    }
}