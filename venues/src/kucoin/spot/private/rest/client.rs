//! KuCoin private REST API client
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use serde::de::DeserializeOwned;
use sha2::Sha256;

use crate::kucoin::spot::{ApiError, RateLimiter, ResponseHeaders, RestResponse, Result};

/// Private REST client for KuCoin spot market
pub struct RestClient {
    pub base_url: String,
    pub client: Client,
    pub rate_limiter: RateLimiter,
    api_key: Box<dyn ExposableSecret>,
    api_secret: Box<dyn ExposableSecret>,
    api_passphrase: Box<dyn ExposableSecret>,
    #[allow(dead_code)]
    is_sandbox: bool,
}

impl RestClient {
    /// Create a new private REST client
    pub fn new(
        base_url: impl Into<String>,
        rate_limiter: RateLimiter,
        client: Client,
        api_key: impl Into<Box<dyn ExposableSecret>>,
        api_secret: impl Into<Box<dyn ExposableSecret>>,
        api_passphrase: impl Into<Box<dyn ExposableSecret>>,
        is_sandbox: bool,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
            api_key: api_key.into(),
            api_secret: api_secret.into(),
            api_passphrase: api_passphrase.into(),
            is_sandbox,
        }
    }

    /// Create a new private REST client with default production settings
    pub fn new_with_credentials(
        api_key: impl Into<Box<dyn ExposableSecret>>,
        api_secret: impl Into<Box<dyn ExposableSecret>>,
        api_passphrase: impl Into<Box<dyn ExposableSecret>>,
    ) -> Self {
        Self::new(
            "https://api.kucoin.com",
            RateLimiter::new(),
            Client::new(),
            api_key,
            api_secret,
            api_passphrase,
            false,
        )
    }

    /// Create a new private REST client for sandbox environment
    pub fn new_sandbox(
        api_key: impl Into<Box<dyn ExposableSecret>>,
        api_secret: impl Into<Box<dyn ExposableSecret>>,
        api_passphrase: impl Into<Box<dyn ExposableSecret>>,
    ) -> Self {
        Self::new(
            "https://openapi-sandbox.kucoin.com",
            RateLimiter::new(),
            Client::new(),
            api_key,
            api_secret,
            api_passphrase,
            true,
        )
    }

    /// Generate authentication headers for KuCoin API
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
        let signature = BASE64.encode(&mac.finalize().into_bytes());

        // Create passphrase signature for KC-API-PASSPHRASE header
        let mut passphrase_mac =
            Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).map_err(|e| ApiError::Other {
                code: "AUTH_ERROR".to_string(),
                message: format!("Failed to create passphrase HMAC: {}", e),
            })?;

        passphrase_mac.update(api_passphrase.as_bytes());
        let passphrase_signature = BASE64.encode(&passphrase_mac.finalize().into_bytes());

        let mut headers = HashMap::new();
        headers.insert("KC-API-KEY".to_string(), api_key);
        headers.insert("KC-API-SIGN".to_string(), signature);
        headers.insert("KC-API-TIMESTAMP".to_string(), timestamp.to_string());
        headers.insert("KC-API-PASSPHRASE".to_string(), passphrase_signature);
        headers.insert("KC-API-KEY-VERSION".to_string(), "2".to_string());

        Ok(headers)
    }

    /// Make a GET request to the private API
    pub async fn get<T>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
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

        let mut request = self.client.get(&url);

        if let Some(params) = params {
            request = request.query(&params);
        }

        // Create auth headers
        let auth_headers = self.create_auth_headers("GET", endpoint, "", timestamp)?;

        for (key, value) in auth_headers {
            request = request.header(&key, &value);
        }

        let response = request.send().await?;

        let status = response.status();
        let headers = response.headers().clone();

        let text = response.text().await?;

        if !status.is_success() {
            // Try to parse as error response
            if let Ok(error_response) =
                serde_json::from_str::<super::super::super::ErrorResponse>(&text)
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

    /// Make a POST request to the private API
    pub async fn post<T>(
        &self,
        endpoint: &str,
        body: &str,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
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

        let mut request = self.client.post(&url);

        // Create auth headers
        let auth_headers = self.create_auth_headers("POST", endpoint, body, timestamp)?;

        for (key, value) in auth_headers {
            request = request.header(&key, &value);
        }

        request = request
            .header("Content-Type", "application/json")
            .body(body.to_string());

        let response = request.send().await?;

        let status = response.status();
        let headers = response.headers().clone();

        let text = response.text().await?;

        if !status.is_success() {
            // Try to parse as error response
            if let Ok(error_response) =
                serde_json::from_str::<super::super::super::ErrorResponse>(&text)
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

    /// Make a DELETE request to the private API
    pub async fn delete<T>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
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

        let mut request = self.client.delete(&url);

        if let Some(params) = params {
            request = request.query(&params);
        }

        // Create auth headers
        let auth_headers = self.create_auth_headers("DELETE", endpoint, "", timestamp)?;

        for (key, value) in auth_headers {
            request = request.header(&key, &value);
        }

        let response = request.send().await?;

        let status = response.status();
        let headers = response.headers().clone();

        let text = response.text().await?;

        if !status.is_success() {
            // Try to parse as error response
            if let Ok(error_response) =
                serde_json::from_str::<super::super::super::ErrorResponse>(&text)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSecret(String);

    impl ExposableSecret for MockSecret {
        fn expose_secret(&self) -> String {
            self.0.clone()
        }
    }

    #[test]
    fn test_rest_client_creation() {
        let client = RestClient::new(
            "https://api.kucoin.com",
            RateLimiter::new(),
            Client::new(),
            Box::new(MockSecret("test_key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_passphrase".to_string())) as Box<dyn ExposableSecret>,
            false,
        );

        assert_eq!(client.base_url, "https://api.kucoin.com");
        assert_eq!(client.is_sandbox, false);
    }

    #[test]
    fn test_rest_client_new_with_credentials() {
        let client = RestClient::new_with_credentials(
            Box::new(MockSecret("test_key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_passphrase".to_string())) as Box<dyn ExposableSecret>,
        );

        assert_eq!(client.base_url, "https://api.kucoin.com");
        assert_eq!(client.is_sandbox, false);
    }

    #[test]
    fn test_rest_client_new_sandbox() {
        let client = RestClient::new_sandbox(
            Box::new(MockSecret("test_key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("test_passphrase".to_string())) as Box<dyn ExposableSecret>,
        );

        assert_eq!(client.base_url, "https://openapi-sandbox.kucoin.com");
        assert_eq!(client.is_sandbox, true);
    }

    #[test]
    fn test_create_auth_headers() {
        let client = RestClient::new(
            "https://api.kucoin.com",
            RateLimiter::new(),
            Client::new(),
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
            "https://api.kucoin.com",
            RateLimiter::new(),
            Client::new(),
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
                "{\"symbol\":\"BTC-USDT\"}",
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
                "{\"symbol\":\"BTC-USDT\"}",
                timestamp,
            )
            .unwrap();
        assert_eq!(headers.get("KC-API-SIGN"), headers2.get("KC-API-SIGN"));
    }

    #[test]
    fn test_auth_headers_different_methods() {
        let client = RestClient::new(
            "https://api.kucoin.com",
            RateLimiter::new(),
            Client::new(),
            Box::new(MockSecret("key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("pass".to_string())) as Box<dyn ExposableSecret>,
            false,
        );

        let timestamp = 1234567890123i64;

        let get_headers = client
            .create_auth_headers("GET", "/api/v1/accounts", "", timestamp)
            .unwrap();
        let post_headers = client
            .create_auth_headers("POST", "/api/v1/accounts", "", timestamp)
            .unwrap();
        let delete_headers = client
            .create_auth_headers("DELETE", "/api/v1/accounts", "", timestamp)
            .unwrap();

        // Different methods should produce different signatures
        assert_ne!(
            get_headers.get("KC-API-SIGN"),
            post_headers.get("KC-API-SIGN")
        );
        assert_ne!(
            get_headers.get("KC-API-SIGN"),
            delete_headers.get("KC-API-SIGN")
        );
        assert_ne!(
            post_headers.get("KC-API-SIGN"),
            delete_headers.get("KC-API-SIGN")
        );
    }

    #[test]
    fn test_auth_headers_different_endpoints() {
        let client = RestClient::new(
            "https://api.kucoin.com",
            RateLimiter::new(),
            Client::new(),
            Box::new(MockSecret("key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("pass".to_string())) as Box<dyn ExposableSecret>,
            false,
        );

        let timestamp = 1234567890123i64;

        let headers1 = client
            .create_auth_headers("GET", "/api/v1/accounts", "", timestamp)
            .unwrap();
        let headers2 = client
            .create_auth_headers("GET", "/api/v1/orders", "", timestamp)
            .unwrap();

        // Different endpoints should produce different signatures
        assert_ne!(headers1.get("KC-API-SIGN"), headers2.get("KC-API-SIGN"));
    }

    #[test]
    fn test_auth_headers_with_body() {
        let client = RestClient::new(
            "https://api.kucoin.com",
            RateLimiter::new(),
            Client::new(),
            Box::new(MockSecret("key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("pass".to_string())) as Box<dyn ExposableSecret>,
            false,
        );

        let timestamp = 1234567890123i64;

        let headers_no_body = client
            .create_auth_headers("POST", "/api/v1/orders", "", timestamp)
            .unwrap();
        let headers_with_body = client
            .create_auth_headers(
                "POST",
                "/api/v1/orders",
                "{\"symbol\":\"BTC-USDT\"}",
                timestamp,
            )
            .unwrap();

        // Different body content should produce different signatures
        assert_ne!(
            headers_no_body.get("KC-API-SIGN"),
            headers_with_body.get("KC-API-SIGN")
        );
    }

    #[test]
    fn test_passphrase_signature() {
        let client = RestClient::new(
            "https://api.kucoin.com",
            RateLimiter::new(),
            Client::new(),
            Box::new(MockSecret("key".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("secret".to_string())) as Box<dyn ExposableSecret>,
            Box::new(MockSecret("passphrase123".to_string())) as Box<dyn ExposableSecret>,
            false,
        );

        let timestamp = 1234567890123i64;
        let headers = client
            .create_auth_headers("GET", "/api/v1/test", "", timestamp)
            .unwrap();

        // Passphrase should be signed and not be the plain passphrase
        assert!(headers.contains_key("KC-API-PASSPHRASE"));
        assert_ne!(headers.get("KC-API-PASSPHRASE").unwrap(), "passphrase123");
    }
}
