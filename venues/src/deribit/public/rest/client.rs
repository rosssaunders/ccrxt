use crate::deribit::{
    enums::{JsonRpcRequest, JsonRpcResponse},
    errors::{ApiError, Errors},
    rate_limit::{EndpointType, RateLimiter},
};
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Type alias for results returned by Deribit REST API operations
pub type RestResult<T> = Result<T, Errors>;

/// REST client for Deribit public API
#[derive(Debug, Clone)]
pub struct RestClient {
    base_url: String,
    client: Client,
    rate_limiter: Arc<RateLimiter>,
    request_id_counter: Arc<AtomicU64>,
}

impl RestClient {
    /// Create a new REST client for Deribit public API
    pub fn new(base_url: impl Into<String>, client: Client, rate_limiter: RateLimiter) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter: Arc::new(rate_limiter),
            request_id_counter: Arc::new(AtomicU64::new(1)),
        }
    }

    /// Generate a unique request ID
    fn next_request_id(&self) -> u64 {
        self.request_id_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Send a JSON-RPC request to the Deribit API
    pub async fn send_request<T, P>(
        &self,
        method: &str,
        params: Option<P>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        // Check rate limits
        self.rate_limiter.check_limits(endpoint_type.clone()).await?;

        // Create JSON-RPC request
        let request_id = self.next_request_id();
        let json_rpc_request = JsonRpcRequest::new(method.to_string(), params, request_id);

        // Build URL - Deribit uses /api/v2 for JSON-RPC
        let url = format!("{}/api/v2/public/{}", self.base_url, method.replace("public/", ""));

        // Build HTTP request
        let mut request_builder = self.client.request(Method::POST, &url);
        request_builder = request_builder.header("Content-Type", "application/json");

        // Serialize the JSON-RPC request
        let request_body = serde_json::to_string(&json_rpc_request)?;
        request_builder = request_builder.body(request_body);

        // Execute the request
        let response = request_builder.send().await?;

        // Record the request for rate limiting
        self.rate_limiter.record_request(endpoint_type).await;

        // Handle response
        let status = response.status();
        let response_text = response.text().await?;

        if status.is_success() {
            // Parse JSON-RPC response
            let json_rpc_response: JsonRpcResponse<T> = serde_json::from_str(&response_text)
                .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;

            // Check for JSON-RPC error
            if let Some(error) = json_rpc_response.error {
                return Err(Errors::ApiError(ApiError::from(error)));
            }

            // Extract result
            json_rpc_response.result.ok_or_else(|| {
                Errors::Error("Response contained neither result nor error".to_string())
            })
        } else {
            // Try to parse as JSON-RPC error
            if let Ok(json_rpc_response) = serde_json::from_str::<JsonRpcResponse<serde_json::Value>>(&response_text) {
                if let Some(error) = json_rpc_response.error {
                    return Err(Errors::ApiError(ApiError::from(error)));
                }
            }

            Err(Errors::Error(format!("HTTP {}: {}", status, response_text)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::rate_limit::AccountTier;

    #[test]
    fn test_rest_client_creation() {
        let client = Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

        assert_eq!(rest_client.base_url, "https://test.deribit.com");
    }

    #[test]
    fn test_request_id_generation() {
        let client = Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);
        let rest_client = RestClient::new("https://test.deribit.com", client, rate_limiter);

        let id1 = rest_client.next_request_id();
        let id2 = rest_client.next_request_id();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }
}