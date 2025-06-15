//! REST client for Deribit private endpoints.
//!
//! Provides access to all private REST API endpoints for Deribit.
//! All requests are authenticated using OAuth and follow JSON-RPC 2.0 format.

use reqwest::Client;
use rest::secrets::ExposableSecret;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::borrow::Cow;

use crate::deribit::{Errors, RestResult};

/// JSON-RPC 2.0 request structure for Deribit API
#[derive(Debug, Clone, Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Value,
}

/// JSON-RPC 2.0 response structure for Deribit API
#[derive(Debug, Clone, Deserialize)]
struct JsonRpcResponse<T> {
    #[allow(dead_code)]
    jsonrpc: String,
    #[allow(dead_code)]
    id: u64,
    #[serde(flatten)]
    result_or_error: JsonRpcResultOrError<T>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum JsonRpcResultOrError<T> {
    Result { result: T },
    Error { error: crate::deribit::ApiError },
}

/// Authentication information for Deribit API
#[derive(Debug, Clone, Deserialize)]
struct AuthResponse {
    access_token: String,
    #[allow(dead_code)]
    expires_in: u64,
    #[allow(dead_code)]
    refresh_token: String,
    #[allow(dead_code)]
    scope: String,
    #[allow(dead_code)]
    token_type: String,
}

/// Private REST client for Deribit
///
/// This client handles all private API endpoints that require authentication.
/// It provides automatic rate limiting, error handling, and OAuth authentication.
pub struct RestClient {
    /// The base URL for the Deribit REST API
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests
    pub client: Client,

    /// The encrypted API client ID
    pub(crate) client_id: Box<dyn ExposableSecret>,

    /// The encrypted API client secret
    pub(crate) client_secret: Box<dyn ExposableSecret>,

    /// Current access token (if authenticated)
    access_token: Option<String>,

    /// Request ID counter for JSON-RPC
    request_id: std::sync::atomic::AtomicU64,
}

impl RestClient {
    /// Create a new REST client for Deribit private endpoints
    ///
    /// # Arguments
    /// * `client_id` - Your Deribit API client ID
    /// * `client_secret` - Your Deribit API client secret
    /// * `base_url` - The base URL for the Deribit API
    /// * `client` - HTTP client for making requests
    pub fn new(
        client_id: Box<dyn ExposableSecret>,
        client_secret: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            client_id,
            client_secret,
            access_token: None,
            request_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    /// Get the next request ID for JSON-RPC
    fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    /// Authenticate with Deribit API using OAuth
    pub async fn authenticate(&mut self) -> RestResult<()> {
        let client_id = self.client_id.expose_secret();
        let client_secret = self.client_secret.expose_secret();

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: self.next_request_id(),
            method: "public/auth".to_string(),
            params: json!({
                "grant_type": "client_credentials",
                "client_id": client_id,
                "client_secret": client_secret
            }),
        };

        let response = self
            .client
            .post(format!("{}/api/v2/public/auth", self.base_url))
            .json(&request)
            .send()
            .await?;

        let json_response: JsonRpcResponse<AuthResponse> = response.json().await?;

        match json_response.result_or_error {
            JsonRpcResultOrError::Result { result } => {
                self.access_token = Some(result.access_token);
                Ok(())
            }
            JsonRpcResultOrError::Error { error } => Err(Errors::ApiError(error)),
        }
    }

    /// Send a private API request with authentication
    pub async fn send_private_request<T>(
        &self,
        method: &str,
        params: Value,
    ) -> RestResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let access_token = self
            .access_token
            .as_ref()
            .ok_or_else(|| Errors::Error("Not authenticated. Call authenticate() first.".to_string()))?;

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: self.next_request_id(),
            method: method.to_string(),
            params,
        };

        let response = self
            .client
            .post(format!("{}/api/v2/{}", self.base_url, method))
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await?;

        let json_response: JsonRpcResponse<T> = response.json().await?;

        match json_response.result_or_error {
            JsonRpcResultOrError::Result { result } => Ok(result),
            JsonRpcResultOrError::Error { error } => Err(Errors::ApiError(error)),
        }
    }
}