// REST client for ByBit V5 private endpoints.
//
// Provides access to all private REST API endpoints for ByBit Exchange.
// All requests are authenticated and require API credentials.

use std::borrow::Cow;

use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use serde::Serialize;
use serde::de::DeserializeOwned;
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
    pub client: Client,

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
    /// * `client` - The HTTP client instance
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        rate_limiter: RateLimiter,
        client: Client,
    ) -> Self {
        Self {
            api_key,
            api_secret,
            base_url: base_url.into(),
            rate_limiter,
            client,
        }
    }

    /// Sends a signed request to the ByBit V5 API
    ///
    /// This method automatically handles timestamp generation and request signing for private endpoints.
    /// It follows ByBit V5 authentication requirements.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/v5/account/wallet-balance")
    /// * `method` - The HTTP method to use
    /// * `request` - The request parameters implementing Serialize
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the parsed response data, or an error
    pub(super) async fn send_signed_request<T, R>(&self, endpoint: &str, method: reqwest::Method, request: R, endpoint_type: EndpointType) -> RestResult<T>
    where
        T: DeserializeOwned,
        R: Serialize,
    {
        // Check rate limits before making request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let recv_window = "5000"; // 5 seconds receive window

        // Serialize query parameters
        let query_string = if method == reqwest::Method::GET {
            serde_urlencoded::to_string(&request)?
        } else {
            String::new()
        };

        // Create signature payload: timestamp + api_key + recv_window + query_string (+ body for POST)
        let mut payload = format!(
            "{}{}{}",
            timestamp,
            self.api_key.expose_secret(),
            recv_window
        );

        if method == reqwest::Method::GET && !query_string.is_empty() {
            payload.push_str(&query_string);
        } else if method == reqwest::Method::POST {
            let body = serde_json::to_string(&request)?;
            payload.push_str(&body);
        }

        // Generate HMAC-SHA256 signature
        let signature = self.sign_payload(&payload)?;

        // Build the URL
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request_builder = self.client.request(method.clone(), &url);

        // Add headers
        request_builder = request_builder
            .header("X-BAPI-API-KEY", self.api_key.expose_secret())
            .header("X-BAPI-TIMESTAMP", &timestamp)
            .header("X-BAPI-SIGN", &signature)
            .header("X-BAPI-RECV-WINDOW", recv_window)
            .header("Content-Type", "application/json");

        // Add query parameters for GET or body for POST
        if method == reqwest::Method::GET && !query_string.is_empty() {
            request_builder = request_builder.query(&[("", &query_string)]);
        } else if method == reqwest::Method::POST {
            request_builder = request_builder.json(&request);
        }

        // Send the request
        let response = request_builder.send().await?;

        // Record the request for rate limiting
        self.rate_limiter.increment_request(endpoint_type).await;

        // Check for HTTP errors
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Errors::ApiError(format!("HTTP {status}: {error_text}")));
        }

        // Parse the response
        let response_text = response.text().await?;
        let parsed_response: T = serde_json::from_str(&response_text)?;

        Ok(parsed_response)
    }

    /// Generate HMAC-SHA256 signature for ByBit V5 API
    fn sign_payload(&self, payload: &str) -> Result<String, Errors> {
        let secret_key = self.api_secret.expose_secret();
        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).map_err(|e| Errors::AuthError(format!("Invalid secret key: {e}")))?;

        mac.update(payload.as_bytes());
        let result = mac.finalize();
        Ok(hex::encode(result.into_bytes()))
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
        let api_secret = Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://api.bybit.com",
            rate_limiter,
            client,
        );

        assert_eq!(rest_client.base_url, "https://api.bybit.com");
    }

    #[test]
    fn test_sign_payload() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://api.bybit.com",
            rate_limiter,
            client,
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
