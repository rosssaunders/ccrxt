use std::{collections::HashMap, sync::Arc};

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;
use hmac::{Hmac, Mac};
use rest::{HttpClient, HttpError, Method as HttpMethod, RequestBuilder, secrets::ExposableSecret};
use serde::de::DeserializeOwned;
use sha2::Sha256;

use crate::kucoin::spot::{ApiError, RateLimiter, ResponseHeaders, RestResponse, Result};

/// Private REST client for KuCoin futures market
pub struct RestClient {
    pub base_url: String,
    pub http_client: Arc<dyn HttpClient>,
    pub rate_limiter: RateLimiter,
    api_key: Box<dyn ExposableSecret>,
    api_secret: Box<dyn ExposableSecret>,
    api_passphrase: Box<dyn ExposableSecret>,
    #[allow(dead_code)]
    is_sandbox: bool,
}

impl RestClient {
    /// Create a new private futures REST client
    pub fn new(
        base_url: impl Into<String>,
        rate_limiter: RateLimiter,
        http_client: Arc<dyn HttpClient>,
        api_key: impl Into<Box<dyn ExposableSecret>>,
        api_secret: impl Into<Box<dyn ExposableSecret>>,
        api_passphrase: impl Into<Box<dyn ExposableSecret>>,
        is_sandbox: bool,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            http_client,
            rate_limiter,
            api_key: api_key.into(),
            api_secret: api_secret.into(),
            api_passphrase: api_passphrase.into(),
            is_sandbox,
        }
    }

    /// Create a new private futures REST client with default production settings
    pub fn new_with_credentials(
        api_key: impl Into<Box<dyn ExposableSecret>>,
        api_secret: impl Into<Box<dyn ExposableSecret>>,
        api_passphrase: impl Into<Box<dyn ExposableSecret>>,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        Self::new(
            "https://api-futures.kucoin.com",
            RateLimiter::new(),
            http_client,
            api_key,
            api_secret,
            api_passphrase,
            false,
        )
    }

    /// Create a new private futures REST client for sandbox environment
    pub fn new_sandbox(
        api_key: impl Into<Box<dyn ExposableSecret>>,
        api_secret: impl Into<Box<dyn ExposableSecret>>,
        api_passphrase: impl Into<Box<dyn ExposableSecret>>,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        Self::new(
            "https://api-sandbox-futures.kucoin.com",
            RateLimiter::new(),
            http_client,
            api_key,
            api_secret,
            api_passphrase,
            true,
        )
    }

    /// Generate authentication headers for KuCoin futures API
    fn create_auth_headers(
        &self,
        method: &str,
        endpoint: &str,
        body: &str,
        timestamp: i64,
    ) -> Result<HashMap<String, String>> {
        let api_key = self.api_key.expose_secret();
        let api_secret = self.api_secret.expose_secret();
        let api_passphrase = self.api_passphrase.expose_secret();

        // Create the string to sign: timestamp + method + endpoint + body
        let str_to_sign = format!("{}{}{}{}", timestamp, method, endpoint, body);

        // Create HMAC-SHA256 signature
        let mut mac =
            Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).map_err(|e| ApiError::Other {
                code: "AUTH_ERROR".to_string(),
                message: format!("Failed to create HMAC: {}", e),
            })?;

        mac.update(str_to_sign.as_bytes());
        let signature = BASE64.encode(mac.finalize().into_bytes());

        // Create passphrase signature for KC-API-PASSPHRASE header
        let mut passphrase_mac =
            Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).map_err(|e| ApiError::Other {
                code: "AUTH_ERROR".to_string(),
                message: format!("Failed to create passphrase HMAC: {}", e),
            })?;

        passphrase_mac.update(api_passphrase.as_bytes());
        let passphrase_signature = BASE64.encode(passphrase_mac.finalize().into_bytes());

        let mut headers = HashMap::new();
        headers.insert("KC-API-KEY".to_string(), api_key);
        headers.insert("KC-API-SIGN".to_string(), signature);
        headers.insert("KC-API-TIMESTAMP".to_string(), timestamp.to_string());
        headers.insert("KC-API-PASSPHRASE".to_string(), passphrase_signature);
        headers.insert("KC-API-KEY-VERSION".to_string(), "2".to_string());

        Ok(headers)
    }

    /// Execute an HTTP request and handle the response
    async fn execute<T>(&self, request: rest::Request) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| match e {
                HttpError::Network(msg) => ApiError::Http(format!("Network error: {}", msg)),
                HttpError::Timeout => ApiError::Http("Request timeout".to_string()),
                HttpError::Http { status, body } => {
                    ApiError::Http(format!("HTTP {}: {}", status, body))
                }
                other => ApiError::Http(format!("HTTP error: {}", other)),
            })?;

        let status = response.status;
        let headers = response.headers.clone();

        let text = response
            .text()
            .map_err(|e| ApiError::Http(format!("Failed to read response: {}", e)))?;

        if !(200..300).contains(&status) {
            // Try to parse as error response
            if let Ok(error_response) =
                serde_json::from_str::<crate::kucoin::spot::ErrorResponse>(&text)
            {
                return Err(ApiError::from(error_response).into());
            } else {
                return Err(ApiError::Http(format!("HTTP {}: {}", status, text)).into());
            }
        }

        // Parse successful response
        let response: RestResponse<T> = serde_json::from_str(&text)
            .map_err(|e| ApiError::JsonParsing(format!("Failed to parse response: {}", e)))?;

        // Check if KuCoin indicates success
        if !response.is_success() {
            return Err(ApiError::Other {
                code: response.code.clone(),
                message: "KuCoin API returned non-success code".to_string(),
            }
            .into());
        }

        let rate_limit_headers = ResponseHeaders::from_headers(&headers);

        Ok((response, rate_limit_headers))
    }

    /// Send a GET request to the private futures API
    pub async fn send_request<T, R>(
        &self,
        endpoint: &str,
        request: Option<&R>,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
        R: serde::Serialize,
    {
        self.get(endpoint, request).await
    }

    /// Alias for get method to match spot client interface
    pub async fn get_with_request<T, R>(
        &self,
        endpoint: &str,
        request: &R,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
        R: serde::Serialize,
    {
        self.get(endpoint, Some(request)).await
    }

    /// Alias for post method to match spot client interface
    pub async fn post_with_request<T, R>(
        &self,
        endpoint: &str,
        request: &R,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
        R: serde::Serialize,
    {
        self.post(endpoint, request).await
    }

    /// Alias for delete method to match spot client interface
    pub async fn delete_with_request<T, R>(
        &self,
        endpoint: &str,
        request: &R,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
        R: serde::Serialize,
    {
        self.delete(endpoint, Some(request)).await
    }

    /// Make a GET request to the private futures API
    pub async fn get<T, R>(
        &self,
        endpoint: &str,
        request: Option<&R>,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
        R: serde::Serialize,
    {
        // Check rate limiter
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".to_string(),
                message: "Rate limit exceeded".to_string(),
            }
            .into());
        }

        let timestamp = Utc::now().timestamp_millis();
        let url = format!("{}{}", self.base_url, endpoint);

        // Build URL with query parameters
        let (full_url, endpoint_sign) = if let Some(req_data) = request {
            let params = serde_urlencoded::to_string(req_data).map_err(|e| {
                ApiError::JsonParsing(format!("Failed to serialize request: {}", e))
            })?;
            if !params.is_empty() {
                (
                    format!("{}?{}", url, params),
                    format!("{}?{}", endpoint, params),
                )
            } else {
                (url, endpoint.to_string())
            }
        } else {
            (url, endpoint.to_string())
        };

        // Create auth headers
        let auth_headers = self.create_auth_headers("GET", &endpoint_sign, "", timestamp)?;

        let mut headers = HashMap::new();
        for (key, value) in auth_headers {
            headers.insert(key, value);
        }

        let request = RequestBuilder::new(HttpMethod::Get, full_url)
            .headers(headers)
            .build();

        self.execute(request).await
    }

    /// Make a POST request to the private futures API
    pub async fn post<T, B>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        // Check rate limiter
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".to_string(),
                message: "Rate limit exceeded".to_string(),
            }
            .into());
        }

        let timestamp = Utc::now().timestamp_millis();
        let url = format!("{}{}", self.base_url, endpoint);

        let body_str = serde_json::to_string(body).map_err(|e| {
            ApiError::JsonParsing(format!("Failed to serialize request body: {}", e))
        })?;

        // Create auth headers
        let auth_headers = self.create_auth_headers("POST", endpoint, &body_str, timestamp)?;

        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        for (key, value) in auth_headers {
            headers.insert(key, value);
        }

        let request = RequestBuilder::new(HttpMethod::Post, url)
            .headers(headers)
            .body(body_str.as_bytes().to_vec())
            .build();

        self.execute(request).await
    }

    /// Make a DELETE request to the private futures API
    pub async fn delete<T, B>(
        &self,
        endpoint: &str,
        params: Option<B>,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
        B: serde::Serialize,
    {
        // Check rate limiter
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".to_string(),
                message: "Rate limit exceeded".to_string(),
            }
            .into());
        }

        let timestamp = Utc::now().timestamp_millis();
        let url = format!("{}{}", self.base_url, endpoint);

        // Build URL with query parameters
        let (full_url, endpoint_sign) = if let Some(params) = &params {
            let query_params = serde_urlencoded::to_string(params).map_err(|e| {
                ApiError::JsonParsing(format!("Failed to serialize query params: {}", e))
            })?;
            if !query_params.is_empty() {
                (
                    format!("{}?{}", url, query_params),
                    format!("{}?{}", endpoint, query_params),
                )
            } else {
                (url, endpoint.to_string())
            }
        } else {
            (url, endpoint.to_string())
        };

        // Create auth headers
        let auth_headers = self.create_auth_headers("DELETE", &endpoint_sign, "", timestamp)?;

        let mut headers = HashMap::new();
        for (key, value) in auth_headers {
            headers.insert(key, value);
        }

        let request = RequestBuilder::new(HttpMethod::Delete, full_url)
            .headers(headers)
            .build();

        self.execute(request).await
    }
}

#[cfg(test)]
mod tests {
    use rest::NativeHttpClient;

    use super::*;

    struct MockSecret(String);

    impl ExposableSecret for MockSecret {
        fn expose_secret(&self) -> String {
            self.0.clone()
        }
    }

    fn mock_http_client() -> Arc<dyn HttpClient> {
        Arc::new(NativeHttpClient::new().unwrap())
    }

    #[test]
    fn test_rest_client_creation() {
        let client = RestClient::new(
            "https://api-futures.kucoin.com",
            RateLimiter::new(),
            mock_http_client(),
            Box::new(MockSecret("test_key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_passphrase".to_string())) as Box<dyn ExposableSecret>,
            false,
        );

        assert_eq!(client.base_url, "https://api-futures.kucoin.com");
        assert!(!client.is_sandbox);
    }

    #[test]
    fn test_rest_client_new_with_credentials() {
        let client = RestClient::new_with_credentials(
            Box::new(MockSecret("test_key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_passphrase".to_string())) as Box<dyn ExposableSecret>,
            mock_http_client(),
        );

        assert_eq!(client.base_url, "https://api-futures.kucoin.com");
        assert!(!client.is_sandbox);
    }

    #[test]
    fn test_rest_client_new_sandbox() {
        let client = RestClient::new_sandbox(
            Box::new(MockSecret("test_key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_passphrase".to_string())) as Box<dyn ExposableSecret>,
            mock_http_client(),
        );

        assert_eq!(client.base_url, "https://api-sandbox-futures.kucoin.com");
        assert!(client.is_sandbox);
    }

    #[test]
    fn test_create_auth_headers() {
        let client = RestClient::new(
            "https://api-futures.kucoin.com",
            RateLimiter::new(),
            mock_http_client(),
            Box::new(MockSecret("test_key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_passphrase".to_string())) as Box<dyn ExposableSecret>,
            false,
        );

        let timestamp = 1234567890123i64;
        let headers = client
            .create_auth_headers("GET", "/api/v1/test", "", timestamp)
            .unwrap();

        assert_eq!(headers.get("KC-API-KEY").unwrap(), "test_key");
        assert_eq!(headers.get("KC-API-TIMESTAMP").unwrap(), "1234567890123");
        assert_eq!(headers.get("KC-API-KEY-VERSION").unwrap(), "2");
        assert!(headers.contains_key("KC-API-SIGN"));
        assert!(headers.contains_key("KC-API-PASSPHRASE"));
    }

    #[test]
    fn test_auth_signature_generation() {
        let client = RestClient::new(
            "https://api-futures.kucoin.com",
            RateLimiter::new(),
            mock_http_client(),
            Box::new(MockSecret("api_key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("api_secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("api_passphrase".to_string())) as Box<dyn ExposableSecret>,
            false,
        );

        let timestamp = 1234567890123i64;
        let headers = client
            .create_auth_headers(
                "POST",
                "/api/v1/orders",
                "{\"symbol\":\"XBTUSDTM\"}",
                timestamp,
            )
            .unwrap();

        // Verify all required headers are present
        assert!(headers.contains_key("KC-API-KEY"));
        assert!(headers.contains_key("KC-API-SIGN"));
        assert!(headers.contains_key("KC-API-TIMESTAMP"));
        assert!(headers.contains_key("KC-API-PASSPHRASE"));
        assert!(headers.contains_key("KC-API-KEY-VERSION"));

        // The signature should be deterministic for the same inputs
        let headers2 = client
            .create_auth_headers(
                "POST",
                "/api/v1/orders",
                "{\"symbol\":\"XBTUSDTM\"}",
                timestamp,
            )
            .unwrap();
        assert_eq!(headers.get("KC-API-SIGN"), headers2.get("KC-API-SIGN"));
    }
}
