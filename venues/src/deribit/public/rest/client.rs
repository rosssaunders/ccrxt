// REST client for Deribit public endpoints.
//
// Provides access to all public REST API endpoints for Deribit.
// All requests are unauthenticated and do not require API credentials.
use std::{borrow::Cow, sync::Arc};

use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
};
use serde::de::DeserializeOwned;

use crate::deribit::{EndpointType, Errors, RateLimiter, RestResult};

/// Public REST client for Deribit exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling.
#[non_exhaustive]
pub struct RestClient {
    /// The base URL for the Deribit public REST API (e.g., "https://www.deribit.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub http_client: Arc<dyn HttpClient>,

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
        http_client: Arc<dyn HttpClient>,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            http_client,
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

        let body = serde_json::to_string(&jsonrpc_body)
            .map_err(|e| Errors::Error(format!("Failed to serialize body: {e}")))?;

        // Build the request
        // All Deribit REST (JSON-RPC) calls are POST
        let request = RequestBuilder::new(HttpMethod::Post, url)
            .header("Content-Type", "application/json")
            .body(body.into_bytes())
            .build();

        // Send the request
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| Errors::NetworkError(format!("HTTP request failed: {e}")))?;

        // Record the request after successful send
        self.rate_limiter.record_request(endpoint_type).await;

        // Check if the response was successful
        if !(response.status >= 200 && response.status < 300) {
            let status = response.status;
            let error_text = response
                .text()
                .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;
            return Err(Errors::Error(format!("HTTP {status}: {error_text}")));
        }

        // Parse the response
        let response_text = response
            .text()
            .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;

        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::deribit::AccountTier;

    #[test]
    fn test_public_client_creation() {
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://www.deribit.com", http_client, rate_limiter);

        assert_eq!(rest_client.base_url, "https://www.deribit.com");
    }

    #[test]
    fn test_url_building() {
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://www.deribit.com", http_client, rate_limiter);

        // Test that the client is properly initialized
        assert_eq!(rest_client.base_url, "https://www.deribit.com");
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new("https://www.deribit.com", http_client, rate_limiter);

        // Test that rate limiting works (this shouldn't fail since we're not actually hitting limits)
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::NonMatchingEngine)
            .await;
        assert!(result.is_ok());
    }
}
