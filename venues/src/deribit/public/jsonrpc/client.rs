//! JSON-RPC client for Deribit public endpoints

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Cow;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::deribit::{
    DeribitError, DeribitResult, EndpointType, JsonRpcRequest, JsonRpcResponse, RateLimiter,
};

/// Public JSON-RPC client for Deribit exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling for Deribit's JSON-RPC API.
#[derive(Debug)]
pub struct JsonRpcClient {
    /// The base URL for the Deribit JSON-RPC API (e.g., "https://www.deribit.com/api/v2")
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits
    pub rate_limiter: RateLimiter,

    /// Request ID counter for JSON-RPC requests
    request_id_counter: AtomicU64,
}

impl JsonRpcClient {
    /// Creates a new Deribit public JSON-RPC client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Deribit JSON-RPC API
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
            request_id_counter: AtomicU64::new(1),
        }
    }

    /// Generate the next request ID
    fn next_request_id(&self) -> u64 {
        self.request_id_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Send a JSON-RPC request to a public endpoint
    ///
    /// # Arguments
    /// * `method` - The JSON-RPC method name (e.g., "public/status")
    /// * `params` - Optional request parameters (must implement Serialize)
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the response data or an error
    pub async fn send_request<T, P>(
        &self,
        method: &str,
        params: Option<P>,
        endpoint_type: EndpointType,
    ) -> DeribitResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        // Check rate limits before making the request
        self.rate_limiter.check_limits(endpoint_type.clone()).await?;

        // Generate request ID and create JSON-RPC request
        let request_id = self.next_request_id();
        let request = JsonRpcRequest::new(request_id, method.to_string(), params);

        // Build the HTTP request
        let response = self
            .client
            .post(self.base_url.as_ref())
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        // Record request for rate limiting after sending
        self.rate_limiter.record_request(endpoint_type).await;

        // Check HTTP status
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(DeribitError::Error(format!("HTTP {}: {}", status, error_text)));
        }

        // Parse the JSON-RPC response
        let response_text = response.text().await?;
        let json_response: JsonRpcResponse<T> = serde_json::from_str(&response_text)?;

        // Convert to result
        json_response.into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::AccountTier;

    #[test]
    fn test_client_creation() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let jsonrpc_client = JsonRpcClient::new("https://www.deribit.com/api/v2", client, rate_limiter);

        assert_eq!(jsonrpc_client.base_url, "https://www.deribit.com/api/v2");
    }

    #[test]
    fn test_request_id_generation() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let jsonrpc_client = JsonRpcClient::new("https://www.deribit.com/api/v2", client, rate_limiter);

        let id1 = jsonrpc_client.next_request_id();
        let id2 = jsonrpc_client.next_request_id();
        let id3 = jsonrpc_client.next_request_id();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let jsonrpc_client = JsonRpcClient::new("https://www.deribit.com/api/v2", client, rate_limiter);

        // Test that rate limiting works (this shouldn't fail since we're not actually hitting limits)
        let result = jsonrpc_client
            .rate_limiter
            .check_limits(EndpointType::PublicStatus)
            .await;
        assert!(result.is_ok());
    }
}