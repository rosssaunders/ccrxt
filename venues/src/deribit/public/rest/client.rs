use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Cow;

use crate::deribit::{EndpointType, Errors, RateLimiter, RestResult};
use super::auth::{AuthRequest, AuthJsonRpcResponse};

/// Public REST client for Deribit exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling for JSON-RPC based API.
#[derive(Debug)]
pub struct RestClient {
    /// The base URL for the Deribit public REST API (e.g., "https://www.deribit.com/api/v2")
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new Deribit public REST client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Deribit API (e.g., "https://www.deribit.com/api/v2")
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

    /// Send a JSON-RPC request to a public endpoint
    ///
    /// # Arguments
    /// * `method` - The JSON-RPC method name (e.g., "public/auth")
    /// * `params` - The request parameters
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the response data or an error
    pub async fn send_jsonrpc_request<T, P>(
        &self,
        method: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        // Check rate limits before making the request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await
            .map_err(|e| Errors::Error(e.to_string()))?;

        // Build the JSON-RPC request
        let jsonrpc_request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        // Build the URL
        let url = format!("{}/{}", self.base_url, method);

        // Send the request
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&jsonrpc_request)
            .send()
            .await?;

        // Record the request in rate limiter
        self.rate_limiter.record_request(endpoint_type).await;

        // Check if the response is successful
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Errors::Error(format!(
                "HTTP error {}: {}",
                status, text
            )));
        }

        // Parse the response
        let response_text = response.text().await?;
        
        // Try to parse as JSON-RPC response first
        if let Ok(jsonrpc_response) = serde_json::from_str::<serde_json::Value>(&response_text) {
            // Check for JSON-RPC error
            if let Some(error) = jsonrpc_response.get("error") {
                let error_code = error.get("code").and_then(|v| v.as_i64()).unwrap_or(-1);
                let error_message = error.get("message").and_then(|v| v.as_str()).unwrap_or("Unknown error");
                return Err(Errors::Error(format!("JSON-RPC error {}: {}", error_code, error_message)));
            }
            
            // Extract the result field
            if let Some(result) = jsonrpc_response.get("result") {
                return serde_json::from_value(result.clone())
                    .map_err(|e| Errors::Error(format!("Failed to parse result: {}", e)));
            }
        }
        
        // Fall back to parsing the entire response
        serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))
    }

    /// Authenticate using OAuth and retrieve an access token
    ///
    /// # Arguments
    /// * `request` - The authentication request parameters
    ///
    /// # Returns
    /// A result containing the authentication response with access token
    pub async fn auth(&self, request: &AuthRequest) -> RestResult<AuthJsonRpcResponse> {
        self.send_jsonrpc_request(
            "public/auth",
            request,
            EndpointType::NonMatchingEngine,
        )
        .await
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

        let rest_client = RestClient::new(
            "https://test.deribit.com/api/v2",
            client,
            rate_limiter,
        );

        assert_eq!(rest_client.base_url, "https://test.deribit.com/api/v2");
    }

    #[test]
    fn test_auth_request_client_credentials() {
        let request = AuthRequest::client_credentials(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
        );

        assert!(matches!(request.grant_type, crate::deribit::public::rest::auth::GrantType::ClientCredentials));
        assert_eq!(request.client_id, Some("test_client_id".to_string()));
        assert_eq!(request.client_secret, Some("test_client_secret".to_string()));
        assert!(request.refresh_token.is_none());
    }

    #[test]
    fn test_auth_request_refresh_token() {
        let request = AuthRequest::refresh_token("test_refresh_token".to_string());

        assert!(matches!(request.grant_type, crate::deribit::public::rest::auth::GrantType::RefreshToken));
        assert_eq!(request.refresh_token, Some("test_refresh_token".to_string()));
        assert!(request.client_id.is_none());
        assert!(request.client_secret.is_none());
    }

    #[test]
    fn test_auth_request_client_signature() {
        let request = AuthRequest::client_signature(
            "test_client_id".to_string(),
            1234567890,
            "test_signature".to_string(),
        )
        .with_nonce("test_nonce".to_string())
        .with_state("test_state".to_string());

        assert!(matches!(request.grant_type, crate::deribit::public::rest::auth::GrantType::ClientSignature));
        assert_eq!(request.client_id, Some("test_client_id".to_string()));
        assert_eq!(request.timestamp, Some(1234567890));
        assert_eq!(request.signature, Some("test_signature".to_string()));
        assert_eq!(request.nonce, Some("test_nonce".to_string()));
        assert_eq!(request.state, Some("test_state".to_string()));
    }
}