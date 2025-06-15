// REST client for Deribit public endpoints.
//
// Provides access to all public REST API endpoints for Deribit Exchange.
// All requests are unauthenticated and do not require API credentials.
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Cow;

use crate::deribit::{EndpointType, Errors, RateLimiter, RestResult};

/// Public REST client for Deribit exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling.
#[non_exhaustive]
#[derive(Debug)]
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

    /// Send a request to a public endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "public/exchange_token")
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
        P: Serialize + ?Sized,
    {
        // Check rate limits before making the request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await
            .map_err(|e| Errors::RateLimitError(e.to_string()))?;

        // Build the URL - Deribit uses /api/v2/ prefix
        let url = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!("{}/api/v2/{}", self.base_url, endpoint)
        };

        // Build the request
        let mut request_builder = self.client.request(method.clone(), &url);

        // Add JSON-RPC 2.0 content type
        request_builder = request_builder.header("Content-Type", "application/json");

        // Add parameters - Deribit uses JSON-RPC 2.0 format for requests
        if let Some(params) = params {
            let json_rpc_request = serde_json::json!({
                "jsonrpc": "2.0",
                "method": endpoint,
                "params": params,
                "id": 1
            });
            request_builder = request_builder.json(&json_rpc_request);
        } else {
            let json_rpc_request = serde_json::json!({
                "jsonrpc": "2.0",
                "method": endpoint,
                "id": 1
            });
            request_builder = request_builder.json(&json_rpc_request);
        }

        // Send the request
        let response = request_builder.send().await?;

        // Record the request for rate limiting
        self.rate_limiter.record_request(endpoint_type).await;

        // Handle the response
        let status = response.status();
        if status.is_success() {
            let response_text = response.text().await?;

            // Try to parse as a successful JSON-RPC response first
            if let Ok(json_rpc_response) = serde_json::from_str::<serde_json::Value>(&response_text) {
                // Check if it's an error response
                if json_rpc_response.get("error").is_some() {
                    let api_error: crate::deribit::ApiError = serde_json::from_str(&response_text)?;
                    return Err(Errors::ApiError(api_error));
                }
                
                // Extract the result field for successful responses
                if let Some(result) = json_rpc_response.get("result") {
                    let parsed_result: T = serde_json::from_value(result.clone())?;
                    return Ok(parsed_result);
                }
            }

            // If we can't parse as JSON-RPC, try to parse directly
            let parsed: T = serde_json::from_str(&response_text)
                .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;
            Ok(parsed)
        } else {
            let error_text = response.text().await?;
            Err(Errors::Error(format!("HTTP {}: {}", status, error_text)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::{AccountTier, RateLimiter};

    #[test]
    fn test_client_creation() {
        let client = Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let rest_client = RestClient::new("https://www.deribit.com", client, rate_limiter);
        
        assert_eq!(rest_client.base_url, "https://www.deribit.com");
    }
}