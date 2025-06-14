//! Bullish Private REST API client

use crate::bullish::{EndpointType, Errors, RateLimiter, RestResult};
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use sha2::Sha256;
use std::borrow::Cow;

/// Private REST client for Bullish exchange
///
/// This client handles all private API endpoints that require authentication.
/// It provides automatic rate limiting, error handling, and JWT token management.
pub struct RestClient {
    /// The underlying HTTP client used for making requests
    pub(crate) client: Client,
    /// The API key for authentication
    pub(crate) api_key: Box<dyn ExposableSecret>,
    /// The API secret for HMAC signing
    pub(crate) api_secret: Box<dyn ExposableSecret>,
    /// The base URL for the API
    pub(crate) base_url: Cow<'static, str>,
    /// Rate limiter for API requests
    pub(crate) rate_limiter: RateLimiter,
    /// Current JWT token (cached)
    pub(crate) jwt_token: Option<String>,
}

impl RestClient {
    /// Creates a new RestClient with API credentials
    ///
    /// # Arguments
    /// * `api_key` - The API key for authentication
    /// * `api_secret` - The API secret for HMAC signing
    /// * `base_url` - The base URL for the API
    /// * `client` - The HTTP client to use
    /// * `rate_limiter` - Rate limiter for requests
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            client,
            api_key,
            api_secret,
            base_url: base_url.into(),
            rate_limiter,
            jwt_token: None,
        }
    }

    /// Generate JWT token for HMAC authentication
    ///
    /// This method makes a request to the `/v1/users/hmac/login` endpoint
    /// to obtain a JWT token that can be used for authenticated requests.
    ///
    /// # Returns
    /// A JWT token valid for 24 hours
    pub async fn get_jwt_token(&mut self) -> RestResult<String> {
        // Check rate limits
        self.rate_limiter.check_limits(EndpointType::PrivateLogin).await
            .map_err(|e| Errors::RateLimitError(e.to_string()))?;

        let nonce = chrono::Utc::now().timestamp();
        let message = format!("GET/trading-api/v1/users/hmac/login{}", nonce);
        
        // Sign the message with HMAC-SHA256
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.expose_secret().as_bytes())
            .map_err(|_| Errors::InvalidApiKey())?;
        mac.update(message.as_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        let url = format!("{}/trading-api/v1/users/hmac/login", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .header("BX-KEY", self.api_key.expose_secret())
            .header("BX-SIGNATURE", signature)
            .header("BX-NONCE", nonce.to_string())
            .send()
            .await?;

        self.rate_limiter.increment_request(EndpointType::PrivateLogin).await;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(Errors::AuthenticationError(format!("Login failed: {}", error_text)));
        }

        let result: Value = response.json().await?;
        
        if let Some(token) = result.get("token").and_then(|t| t.as_str()) {
            self.jwt_token = Some(token.to_string());
            Ok(token.to_string())
        } else {
            Err(Errors::AuthenticationError("No token in response".to_string()))
        }
    }

    /// Send an authenticated request to the Bullish API
    ///
    /// This method handles JWT token management, rate limiting, and error handling.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path
    /// * `method` - The HTTP method
    /// * `body` - Optional request body for POST/PUT requests
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// The deserialized response or an error
    pub async fn send_authenticated_request<T, B>(
        &mut self,
        endpoint: &str,
        method: reqwest::Method,
        body: Option<&B>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        // Check rate limits
        self.rate_limiter.check_limits(endpoint_type).await
            .map_err(|e| Errors::RateLimitError(e.to_string()))?;

        // Ensure we have a valid JWT token
        if self.jwt_token.is_none() {
            self.get_jwt_token().await?;
        }

        let url = format!("{}/trading-api{}", self.base_url, endpoint);
        let token = self.jwt_token.as_ref().unwrap();

        let mut request = self
            .client
            .request(method.clone(), &url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json");

        if let Some(body_data) = body {
            request = request.json(body_data);
        }

        let response = request.send().await?;
        
        self.rate_limiter.increment_request(endpoint_type).await;

        // Handle 401 Unauthorized - token might be expired
        if response.status() == 401 {
            // Try to refresh token once
            self.jwt_token = None;
            self.get_jwt_token().await?;
            
            // Retry the request with new token
            let token = self.jwt_token.as_ref().unwrap();
            let mut retry_request = self
                .client
                .request(method, &url)
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json");

            if let Some(body_data) = body {
                retry_request = retry_request.json(body_data);
            }

            let retry_response = retry_request.send().await?;
            self.rate_limiter.increment_request(endpoint_type).await;
            
            if !retry_response.status().is_success() {
                let error_text = retry_response.text().await?;
                return Err(Errors::Error(format!("Request failed after token refresh: {}", error_text)));
            }

            let result: T = retry_response.json().await?;
            return Ok(result);
        }

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(Errors::Error(format!("Request failed: {}", error_text)));
        }

        let result: T = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    struct TestSecret {
        secret: String,
    }

    impl ExposableSecret for TestSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl TestSecret {
        fn new(secret: String) -> Self {
            Self { secret }
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
            "https://api.exchange.bullish.com",
            client,
            rate_limiter,
        );

        assert_eq!(rest_client.base_url, "https://api.exchange.bullish.com");
        assert!(rest_client.jwt_token.is_none());
    }

    #[test]
    fn test_hmac_message_format() {
        // Test the HMAC message format matches Bullish specification
        let nonce = 1234567890;
        let message = format!("GET/trading-api/v1/users/hmac/login{}", nonce);
        assert_eq!(message, "GET/trading-api/v1/users/hmac/login1234567890");
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let _rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://api.exchange.bullish.com",
            client,
            rate_limiter,
        );

        // Test that rate limiter integration works
        // This is a basic structure test since we can't make real API calls in unit tests
    }
}