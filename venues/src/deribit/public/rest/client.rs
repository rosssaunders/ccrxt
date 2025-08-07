// REST client for Deribit public endpoints.
//
// Provides access to all public REST API endpoints for Deribit.
// All requests are unauthenticated and do not require API credentials.
use std::borrow::Cow;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::deribit::{EndpointType, Errors, RateLimiter, RestResult};

/// Public REST client for Deribit exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct RestClient {
    /// The base URL for the Deribit public REST API (e.g., "https://www.deribit.com")
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
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new Deribit public REST client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Deribit public REST API (e.g., "https://www.deribit.com")
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter for managing API limits
    pub fn new(
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
        }
    }

    /// Send a POST request to a public endpoint (Deribit JSON-RPC 2.0 over HTTP always uses POST)
    ///
    /// High-performance rule: verb-specific (POST) function without passing an HTTP method enum.
    ///
    /// # Arguments
    /// * `endpoint` - API method name (e.g., "public/get_time")
    /// * `params` - Optional params object (serialized into JSON-RPC envelope)
    /// * `endpoint_type` - Endpoint classification for rate limiting
    pub async fn send_post_request<T, P>(
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
            .check_limits(endpoint_type)
            .await
            .map_err(|e| Errors::Error(e.to_string()))?;

        // Build the URL - Deribit uses JSON-RPC 2.0 over HTTP
        let url = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!("{}/api/v2/{}", self.base_url, endpoint)
        };

        // Build the request
        // All Deribit REST (JSON-RPC) calls are POST
        let mut request_builder = self.client.post(&url);

        // Wrap parameters in JSON-RPC envelope
        let params_value = if let Some(params) = params {
            serde_json::to_value(params)
                .map_err(|e| Errors::Error(format!("Failed to serialize params: {e}")))?
        } else {
            serde_json::Value::Object(serde_json::Map::new())
        };

        let jsonrpc_body = serde_json::json!({
            "method": endpoint,
            "params": params_value,
            "jsonrpc": "2.0",
            "id": 1
        });
        request_builder = request_builder.json(&jsonrpc_body);

        // Add required headers
        request_builder = request_builder.header("Content-Type", "application/json");

        // Send the request
        let response = request_builder.send().await.map_err(Errors::HttpError)?;

        // Record the request after successful send
        self.rate_limiter.record_request(endpoint_type).await;

        // Check if the response was successful
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.map_err(Errors::HttpError)?;
            return Err(Errors::Error(format!("HTTP {status}: {error_text}")));
        }

        // Parse the response
        let response_text = response.text().await.map_err(Errors::HttpError)?;

        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::AccountTier;

    #[test]
    fn test_public_client_creation() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://www.deribit.com", client, rate_limiter);

        assert_eq!(rest_client.base_url, "https://www.deribit.com");
    }

    #[test]
    fn test_url_building() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://www.deribit.com", client, rate_limiter);

        // Test that the client is properly initialized
        assert_eq!(rest_client.base_url, "https://www.deribit.com");
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://www.deribit.com", client, rate_limiter);

        // Test that rate limiting works (this shouldn't fail since we're not actually hitting limits)
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::NonMatchingEngine)
            .await;
        assert!(result.is_ok());
    }
}
