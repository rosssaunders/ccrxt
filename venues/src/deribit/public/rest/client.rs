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

    /// Send a request to a public endpoint
    ///
    /// # Arguments  
    /// * `endpoint` - The API endpoint path (e.g., "public/auth")
    /// * `params` - The request parameters
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the response data or an error
    pub async fn send_request<T, P>(
        &self,
        endpoint: &str,
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

        // Build the URL
        let url = format!("{}/{}", self.base_url, endpoint);

        // Convert params to form data
        let form_data = serde_urlencoded::to_string(&params)
            .map_err(|e| Errors::Error(format!("Failed to serialize params: {}", e)))?;

        // Send the request
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_data)
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
        self.send_request(
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

    #[test]
    fn test_auth_request_serialization() {
        let request = AuthRequest::client_credentials(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
        ).with_scope("session:test".to_string());

        // Test that it can be serialized to form data
        let form_data = serde_urlencoded::to_string(&request).unwrap();
        assert!(form_data.contains("grant_type=client_credentials"));
        assert!(form_data.contains("client_id=test_client_id"));
        assert!(form_data.contains("client_secret=test_client_secret"));
        assert!(form_data.contains("scope=session%3Atest"));
    }

    #[test]
    fn test_auth_response_deserialization() {
        let json_response = r#"{
            "id": 1,
            "jsonrpc": "2.0",
            "result": {
                "access_token": "test_token_123",
                "enabled_features": ["restricted_block_trades"],
                "expires_in": 3600,
                "google_login": false,
                "mandatory_tfa_status": "none",
                "refresh_token": "test_refresh_123",
                "scope": "session:test",
                "state": "test_state",
                "token_type": "bearer"
            }
        }"#;

        let response: super::AuthJsonRpcResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result.access_token, "test_token_123");
        assert_eq!(response.result.expires_in, 3600);
        assert_eq!(response.result.token_type, "bearer");
        assert_eq!(response.result.scope, "session:test");
        assert_eq!(response.result.state, Some("test_state".to_string()));
        assert!(!response.result.google_login);
    }
}