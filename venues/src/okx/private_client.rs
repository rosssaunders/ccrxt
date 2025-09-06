// REST client for OKX private endpoints.
//
// Provides access to all private REST API endpoints for OKX Exchange.
// All requests are authenticated and require API credentials.
use std::{borrow::Cow, sync::Arc};

use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use hmac::{Hmac, Mac};
use rest::{HttpClient, Method, Request};
use secrets::ExposableSecret;
use serde::{Serialize, de::DeserializeOwned};
use sha2::Sha256;

use crate::okx::{EndpointType, Errors, RateLimiter, RestResult, credentials::Credentials};

/// Private REST client for OKX exchange
///
/// This client handles all private API endpoints that require authentication.
/// It provides automatic rate limiting, error handling, and request signing.
pub struct RestClient {
    /// The base URL for the OKX private REST API (e.g., "https://www.okx.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    base_url: Cow<'static, str>,

    /// Pre-formatted base URL with trailing slash for fast concatenation
    ///
    /// This avoids runtime string formatting in the hot path.
    formatted_base: String,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub http_client: Arc<dyn HttpClient>,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with OKX's rate limits for private endpoints.
    pub rate_limiter: RateLimiter,

    /// The API credentials for authentication.
    pub(crate) credentials: Credentials,
}

impl RestClient {
    /// Creates a new OKX private REST client.
    ///
    /// # Arguments
    /// * `credentials` - The API credentials for authentication
    /// * `base_url` - The base URL for the OKX private REST API (e.g., "https://www.okx.com")
    /// * `http_client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter for managing API limits
    pub fn new(
        credentials: Credentials,
        base_url: impl Into<Cow<'static, str>>,
        http_client: Arc<dyn HttpClient>,
        rate_limiter: RateLimiter,
    ) -> Self {
        let base_url = base_url.into();
        // Pre-format the base URL with trailing slash for fast concatenation
        let formatted_base = format!("{}/", base_url.trim_end_matches('/'));

        Self {
            base_url,
            formatted_base,
            http_client,
            rate_limiter,
            credentials,
        }
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Sign a request for OKX private endpoints
    ///
    /// Creates the signature according to OKX's signing algorithm:
    /// 1. Create pre-hash string: timestamp + method + requestPath + body
    /// 2. Sign with HMAC SHA256 using the API secret
    /// 3. Encode as Base64
    ///
    /// # Arguments
    /// * `timestamp` - The timestamp string (ISO format)
    /// * `method` - The HTTP method (uppercase)
    /// * `request_path` - The request path including query parameters
    /// * `body` - The request body (empty string for GET requests)
    ///
    /// # Returns
    /// A result containing the signature as a base64 string or an error
    pub fn sign_request(
        &self,
        timestamp: &str,
        method: &str,
        request_path: &str,
        body: &str,
    ) -> Result<String, Errors> {
        // Create the pre-hash string: timestamp + method + requestPath + body
        let pre_hash = format!("{timestamp}{method}{request_path}{body}");

        // Sign with HMAC SHA256
        let api_secret = self.credentials.api_secret.expose_secret();
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
            .map_err(|_| Errors::InvalidApiKey())?;
        mac.update(pre_hash.as_bytes());

        // Encode as Base64
        Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
    }

    /// Send a GET request to a private endpoint (optimized for HFT)
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "api/v5/account/balance")
    /// * `params` - Optional query parameters
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_get_request<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        // Check rate limits
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        // Build URL - branch-free
        let url = format!("{}{}", self.formatted_base, endpoint);

        // Create timestamp
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        // Handle query parameters for GET requests
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| Errors::Error(format!("Failed to serialize query parameters: {e}")))?;
        let (final_url, request_path) = if !query_string.is_empty() {
            (
                format!("{url}?{query_string}"),
                format!("/{endpoint}?{query_string}"),
            )
        } else {
            (url, format!("/{endpoint}"))
        };

        // Create signature for GET (no body)
        let signature = self.sign_request(&timestamp, "GET", &request_path, "")?;

        // Add required headers
        let api_key = self.credentials.api_key.expose_secret();
        let api_passphrase = self.credentials.api_passphrase.expose_secret();

        let mut headers = std::collections::HashMap::new();
        headers.insert("OK-ACCESS-KEY".to_string(), api_key.as_str().to_string());
        headers.insert("OK-ACCESS-SIGN".to_string(), signature);
        headers.insert("OK-ACCESS-TIMESTAMP".to_string(), timestamp);
        headers.insert(
            "OK-ACCESS-PASSPHRASE".to_string(),
            api_passphrase.as_str().to_string(),
        );

        // Create request
        let request = Request {
            method: Method::Get,
            url: final_url,
            headers,
            body: None,
            timeout: None,
        };

        // Send request
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| Errors::Error(format!("HTTP request failed: {e}")))?;

        // Record request for rate limiting
        self.rate_limiter.increment_request(endpoint_type).await;

        // Get response status and body in one go
        let status = response.status;
        let response_text = response
            .text()
            .map_err(|e| Errors::Error(format!("Failed to decode response body: {e}")))?;

        // Check status after getting text
        if !(200..300).contains(&status) {
            return Err(Errors::Error(format!("HTTP {status}: {response_text}")));
        }

        // Parse the response
        let parsed: crate::okx::response::ApiResponse<T> = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;

        Ok(parsed)
    }

    /// Send a POST request to a private endpoint (optimized for HFT)
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "api/v5/trade/order")
    /// * `params` - Request body parameters
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_post_request<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        // Check rate limits
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        // Build URL - branch-free
        let url = format!("{}{}", self.formatted_base, endpoint);

        // Create timestamp
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        // Handle body for POST requests - always serialize for POST
        let body = serde_json::to_string(&params)
            .map_err(|e| Errors::Error(format!("Failed to serialize request body: {e}")))?;

        let request_path = format!("/{endpoint}");

        // Create signature for POST
        let signature = self.sign_request(&timestamp, "POST", &request_path, &body)?;

        // Add required headers
        let api_key = self.credentials.api_key.expose_secret();
        let api_passphrase = self.credentials.api_passphrase.expose_secret();

        let mut headers = std::collections::HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("OK-ACCESS-KEY".to_string(), api_key.as_str().to_string());
        headers.insert("OK-ACCESS-SIGN".to_string(), signature);
        headers.insert("OK-ACCESS-TIMESTAMP".to_string(), timestamp);
        headers.insert(
            "OK-ACCESS-PASSPHRASE".to_string(),
            api_passphrase.as_str().to_string(),
        );

        // Create request
        let request = Request {
            method: Method::Post,
            url,
            headers,
            body: Some(body.into_bytes()),
            timeout: None,
        };

        // Send request
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| Errors::Error(format!("HTTP request failed: {e}")))?;

        // Record request for rate limiting
        self.rate_limiter.increment_request(endpoint_type).await;

        // Get response status and body in one go
        let status = response.status;
        let response_text = response
            .text()
            .map_err(|e| Errors::Error(format!("Failed to decode response body: {e}")))?;

        // Check status after getting text
        if !(200..300).contains(&status) {
            return Err(Errors::Error(format!("HTTP {status}: {response_text}")));
        }

        // Parse the response
        let parsed: crate::okx::response::ApiResponse<T> = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;

        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use async_trait::async_trait;
    use rest::{HttpClient, HttpError, Response};
    use secrets::SecretString;

    use super::*;

    #[derive(Debug)]
    struct MockHttpClient;

    #[async_trait]
    impl HttpClient for MockHttpClient {
        async fn execute(&self, _request: Request) -> Result<Response, HttpError> {
            Ok(Response {
                status: 200,
                headers: HashMap::new(),
                body: br#"{"code":"0","msg":"","data":[]}"#.to_vec().into(),
            })
        }
    }

    #[test]
    fn test_private_client_creation() {
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
            api_passphrase: SecretString::from("test_passphrase".to_string()),
        };
        let http_client: Arc<dyn HttpClient> = Arc::new(MockHttpClient);
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            credentials,
            "https://www.okx.com",
            http_client,
            rate_limiter,
        );

        assert_eq!(rest_client.base_url(), "https://www.okx.com");
    }

    #[test]
    fn test_signature_generation() {
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
            api_passphrase: SecretString::from("test_passphrase".to_string()),
        };
        let http_client: Arc<dyn HttpClient> = Arc::new(MockHttpClient);
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            credentials,
            "https://www.okx.com",
            http_client,
            rate_limiter,
        );

        let timestamp = "2020-12-08T09:08:57.715Z";
        let method = "GET";
        let request_path = "/api/v5/account/balance?ccy=BTC";
        let body = "";

        let signature = rest_client
            .sign_request(timestamp, method, request_path, body)
            .unwrap();

        // Verify the signature is a valid base64 string
        assert!(general_purpose::STANDARD.decode(&signature).is_ok());
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
            api_passphrase: SecretString::from("test_passphrase".to_string()),
        };
        let http_client: Arc<dyn HttpClient> = Arc::new(MockHttpClient);
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            credentials,
            "https://www.okx.com",
            http_client,
            rate_limiter,
        );

        // Verify rate limiting works
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::PrivateTrading)
            .await;
        assert!(result.is_ok());
    }
}
