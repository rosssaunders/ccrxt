// REST client for ByBit V5 private endpoints.
//
// Provides access to all private REST API endpoints for ByBit Exchange.
// All requests are authenticated and require API credentials.

use std::{borrow::Cow, sync::Arc};

use hmac::{Hmac, Mac};
use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
    secrets::ExposableSecret,
};
use serde::Serialize;
use sha2::Sha256;

use crate::bybit::{EndpointType, Errors, RateLimiter, RestResult};

/// Private REST client for ByBit V5 exchange
///
/// This client handles all private API endpoints that require authentication.
/// It provides automatic rate limiting, error handling, and request signing.
pub struct RestClient {
    /// The base URL for the ByBit V5 private REST API (e.g., "https://api.bybit.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub http_client: Arc<dyn HttpClient>,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with ByBit's rate limits for private endpoints.
    pub rate_limiter: RateLimiter,

    /// The encrypted API key.
    pub(crate) api_key: Box<dyn ExposableSecret>,

    /// The encrypted API secret.
    pub(crate) api_secret: Box<dyn ExposableSecret>,
}

impl RestClient {
    /// Creates a new RestClient with encrypted API credentials
    ///
    /// # Arguments
    /// * `api_key` - The encrypted API key
    /// * `api_secret` - The encrypted API secret
    /// * `base_url` - The base URL for the API
    /// * `rate_limiter` - The rate limiter instance
    /// * `http_client` - The HTTP client instance
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        rate_limiter: RateLimiter,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        Self {
            api_key,
            api_secret,
            base_url: base_url.into(),
            rate_limiter,
            http_client,
        }
    }

    /// Generate HMAC-SHA256 signature for ByBit V5 API
    fn sign_payload(&self, payload: &str) -> Result<String, Errors> {
        let secret_key = self.api_secret.expose_secret();
        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
            .map_err(|e| Errors::AuthError(format!("Invalid secret key: {e}")))?;

        mac.update(payload.as_bytes());
        let result = mac.finalize();
        Ok(hex::encode(result.into_bytes()))
    }

    /// High-performance GET request method (optimized for HFT)
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path
    /// * `request` - The request parameters
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// The deserialized response or an error
    pub async fn send_get_signed_request<T, R>(
        &self,
        endpoint: &str,
        request: R,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
        R: Serialize,
    {
        // Check rate limits before making request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let recv_window = "5000"; // 5 seconds receive window

        // Serialize query parameters for GET
        let query_string = serde_urlencoded::to_string(&request)?;

        // Create signature payload: timestamp + api_key + recv_window + query_string
        let mut payload = format!(
            "{}{}{}",
            timestamp,
            self.api_key.expose_secret(),
            recv_window
        );

        if !query_string.is_empty() {
            payload.push_str(&query_string);
        }

        // Generate HMAC-SHA256 signature
        let signature = self.sign_payload(&payload)?;

        // Build the URL with query parameters
        let url = if query_string.is_empty() {
            format!("{}{}", self.base_url, endpoint)
        } else {
            format!("{}{}?{}", self.base_url, endpoint, query_string)
        };

        // Build request
        let request = RequestBuilder::new(HttpMethod::Get, url)
            .header("X-BAPI-API-KEY", &self.api_key.expose_secret())
            .header("X-BAPI-TIMESTAMP", &timestamp)
            .header("X-BAPI-SIGN", &signature)
            .header("X-BAPI-RECV-WINDOW", recv_window)
            .header("Content-Type", "application/json")
            .build();

        // Send the request
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| Errors::NetworkError(format!("HTTP request failed: {e}")))?;

        // Record the request for rate limiting
        self.rate_limiter.increment_request(endpoint_type).await;

        // Check for HTTP errors
        let status = response.status;
        if status != 200 && status != 201 {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Errors::ApiError(format!("HTTP {status}: {error_text}")));
        }

        // Parse the response
        let response_text = response
            .text()
            .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;
        let parsed_response: T = serde_json::from_str(&response_text)?;

        Ok(parsed_response)
    }

    /// High-performance POST request method (optimized for HFT)
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path
    /// * `request` - The request parameters
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// The deserialized response or an error
    pub async fn send_post_signed_request<T, R>(
        &self,
        endpoint: &str,
        request: R,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
        R: Serialize,
    {
        // Check rate limits before making request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let recv_window = "5000"; // 5 seconds receive window

        // Serialize body for POST
        let body = serde_json::to_string(&request)?;

        // Create signature payload: timestamp + api_key + recv_window + body
        let payload = format!(
            "{}{}{}{}",
            timestamp,
            self.api_key.expose_secret(),
            recv_window,
            body
        );

        // Generate HMAC-SHA256 signature
        let signature = self.sign_payload(&payload)?;

        // Build the URL
        let url = format!("{}{}", self.base_url, endpoint);

        // Build request
        let request_obj = RequestBuilder::new(HttpMethod::Post, url)
            .header("X-BAPI-API-KEY", &self.api_key.expose_secret())
            .header("X-BAPI-TIMESTAMP", &timestamp)
            .header("X-BAPI-SIGN", &signature)
            .header("X-BAPI-RECV-WINDOW", recv_window)
            .header("Content-Type", "application/json")
            .body(body.into_bytes())
            .build();

        // Send the request
        let response = self
            .http_client
            .execute(request_obj)
            .await
            .map_err(|e| Errors::NetworkError(format!("HTTP request failed: {e}")))?;

        // Record the request for rate limiting
        self.rate_limiter.increment_request(endpoint_type).await;

        // Check for HTTP errors
        let status = response.status;
        if status != 200 && status != 201 {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Errors::ApiError(format!("HTTP {status}: {error_text}")));
        }

        // Parse the response
        let response_text = response
            .text()
            .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;
        let parsed_response: T = serde_json::from_str(&response_text)?;

        Ok(parsed_response)
    }

    /// High-performance PUT request method (optimized for HFT)
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path
    /// * `request` - The request parameters
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// The deserialized response or an error
    pub async fn send_put_signed_request<T, R>(
        &self,
        endpoint: &str,
        request: R,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
        R: Serialize,
    {
        // Check rate limits before making request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let recv_window = "5000"; // 5 seconds receive window

        // Serialize body for PUT
        let body = serde_json::to_string(&request)?;

        // Create signature payload: timestamp + api_key + recv_window + body
        let payload = format!(
            "{}{}{}{}",
            timestamp,
            self.api_key.expose_secret(),
            recv_window,
            body
        );

        // Generate HMAC-SHA256 signature
        let signature = self.sign_payload(&payload)?;

        // Build the URL
        let url = format!("{}{}", self.base_url, endpoint);

        // Build request
        let request_obj = RequestBuilder::new(HttpMethod::Put, url)
            .header("X-BAPI-API-KEY", &self.api_key.expose_secret())
            .header("X-BAPI-TIMESTAMP", &timestamp)
            .header("X-BAPI-SIGN", &signature)
            .header("X-BAPI-RECV-WINDOW", recv_window)
            .header("Content-Type", "application/json")
            .body(body.into_bytes())
            .build();

        // Send the request
        let response = self
            .http_client
            .execute(request_obj)
            .await
            .map_err(|e| Errors::NetworkError(format!("HTTP request failed: {e}")))?;

        // Record the request for rate limiting
        self.rate_limiter.increment_request(endpoint_type).await;

        // Check for HTTP errors
        let status = response.status;
        if status != 200 && status != 201 {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Errors::ApiError(format!("HTTP {status}: {error_text}")));
        }

        // Parse the response
        let response_text = response
            .text()
            .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;
        let parsed_response: T = serde_json::from_str(&response_text)?;

        Ok(parsed_response)
    }

    /// High-performance DELETE request method (optimized for HFT)
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path
    /// * `request` - The request parameters
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// The deserialized response or an error
    pub async fn send_delete_signed_request<T, R>(
        &self,
        endpoint: &str,
        request: R,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
        R: Serialize,
    {
        // Check rate limits before making request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let recv_window = "5000"; // 5 seconds receive window

        // Serialize query parameters for DELETE
        let query_string = serde_urlencoded::to_string(&request)?;

        // Create signature payload: timestamp + api_key + recv_window + query_string
        let mut payload = format!(
            "{}{}{}",
            timestamp,
            self.api_key.expose_secret(),
            recv_window
        );

        if !query_string.is_empty() {
            payload.push_str(&query_string);
        }

        // Generate HMAC-SHA256 signature
        let signature = self.sign_payload(&payload)?;

        // Build the URL with query parameters
        let url = if query_string.is_empty() {
            format!("{}{}", self.base_url, endpoint)
        } else {
            format!("{}{}?{}", self.base_url, endpoint, query_string)
        };

        // Build request
        let request = RequestBuilder::new(HttpMethod::Delete, url)
            .header("X-BAPI-API-KEY", &self.api_key.expose_secret())
            .header("X-BAPI-TIMESTAMP", &timestamp)
            .header("X-BAPI-SIGN", &signature)
            .header("X-BAPI-RECV-WINDOW", recv_window)
            .header("Content-Type", "application/json")
            .build();

        // Send the request
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| Errors::NetworkError(format!("HTTP request failed: {e}")))?;

        // Record the request for rate limiting
        self.rate_limiter.increment_request(endpoint_type).await;

        // Check for HTTP errors
        let status = response.status;
        if status != 200 && status != 201 {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Errors::ApiError(format!("HTTP {status}: {error_text}")));
        }

        // Parse the response
        let response_text = response
            .text()
            .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;
        let parsed_response: T = serde_json::from_str(&response_text)?;

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;

    use super::*;

    struct TestSecret {
        secret: String,
    }

    impl TestSecret {
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    impl ExposableSecret for TestSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    #[test]
    fn test_private_client_creation() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://api.bybit.com",
            rate_limiter,
            http_client,
        );

        assert_eq!(rest_client.base_url, "https://api.bybit.com");
    }

    #[test]
    fn test_sign_payload() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://api.bybit.com",
            rate_limiter,
            http_client,
        );

        let payload = "test_payload";
        let signature = rest_client.sign_payload(payload);
        assert!(signature.is_ok());
        if let Ok(sig) = signature {
            assert!(!sig.is_empty());
        } else {
            assert_eq!(true, false, "Signature should be Ok");
        }
    }
}
