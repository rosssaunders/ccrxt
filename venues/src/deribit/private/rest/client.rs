use crate::deribit::{Errors, RateLimiter, EndpointType};
use hmac::{Hmac, Mac};
use rest::secrets::ExposableSecret;
use serde_json::Value;
use sha2::Sha256;
use std::borrow::Cow;

/// A client for interacting with the Deribit private REST API
///
/// This client handles encrypted API keys and secrets for enhanced security.
/// The API key and secret are stored in encrypted form and only decrypted when needed.
pub struct RestClient {
    /// The underlying HTTP client used for making requests.
    pub(crate) client: reqwest::Client,
    /// The encrypted API key.
    pub(crate) api_key: Box<dyn ExposableSecret>,
    /// The encrypted API secret.
    pub(crate) api_secret: Box<dyn ExposableSecret>,
    /// The base URL for the API.
    pub(crate) base_url: Cow<'static, str>,
    /// The rate limiter for managing API request limits.
    pub(crate) rate_limiter: RateLimiter,
}

impl RestClient {
    /// Creates a new RestClient with encrypted API credentials
    ///
    /// # Arguments
    /// * `api_key` - The encrypted API key
    /// * `api_secret` - The encrypted API secret
    /// * `base_url` - The base URL for the API
    /// * `client` - The HTTP client to use
    /// * `rate_limiter` - The rate limiter instance
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        client: reqwest::Client,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            client,
            api_key,
            api_secret,
            base_url: base_url.into(),
            rate_limiter,
        }
    }

    /// Signs a request for Deribit private endpoints
    ///
    /// Deribit uses a signature-based authentication system:
    /// 1. Create the signature string: timestamp + method + request_uri + params_string
    /// 2. Sign with HMAC-SHA256 using the API secret
    /// 3. Return as hex string
    ///
    /// # Arguments
    /// * `timestamp` - The timestamp string (Unix timestamp in milliseconds)
    /// * `method` - The HTTP method (GET, POST, etc.)
    /// * `request_uri` - The request URI including path and query parameters
    /// * `params_string` - The request body or empty string for GET requests
    ///
    /// # Returns
    /// A result containing the signature as a hex string or an error
    pub fn sign_request(
        &self,
        timestamp: &str,
        method: &str,
        request_uri: &str,
        params_string: &str,
    ) -> Result<String, Errors> {
        // Create the string to be signed: timestamp + method + request_uri + params_string
        let string_to_sign = format!("{}{}{}{}", timestamp, method, request_uri, params_string);

        // Sign with HMAC SHA256
        let api_secret = self.api_secret.expose_secret();
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
            .map_err(|_| Errors::InvalidApiKey())?;
        mac.update(string_to_sign.as_bytes());

        // Return as hex string
        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    /// Send a request to a private endpoint
    ///
    /// # Arguments
    /// * `method` - The JSON-RPC method name
    /// * `params` - The request parameters as JSON Value
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the parsed response data or an error
    pub async fn send_request(
        &self,
        method: &str,
        params: Value,
        endpoint_type: EndpointType,
    ) -> Result<Value, Errors> {
        // Check rate limits
        self.rate_limiter.check_limits(endpoint_type).await
            .map_err(|e| Errors::Error(format!("Rate limit exceeded: {}", e)))?;

        // Get current timestamp in milliseconds
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        
        // Build the request URI
        let request_uri = "/api/v2/private".to_string();
        
        // Create the JSON-RPC request body
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        });

        let params_string = serde_json::to_string(&request_body)
            .map_err(|e| Errors::Error(format!("Failed to serialize request: {}", e)))?;

        // Generate signature
        let signature = self.sign_request(&timestamp, "POST", &request_uri, &params_string)?;

        // Build the full URL
        let url = format!("{}{}", self.base_url, request_uri);

        // Add authentication headers
        let api_key = self.api_key.expose_secret();
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("deri-hmac-sha256 id={},ts={},sig={}", api_key, timestamp, signature))
            .json(&request_body)
            .send()
            .await?;

        // Record the request for rate limiting
        self.rate_limiter.record_request(endpoint_type).await;

        // Parse the response
        let result: Value = response.json().await?;
        
        // Check for JSON-RPC errors
        if let Some(error) = result.get("error") {
            let error_detail: crate::deribit::ErrorDetail = serde_json::from_value(error.clone())
                .map_err(|e| Errors::Error(format!("Failed to parse error response: {}", e)))?;
            return Err(Errors::ApiError(error_detail.into()));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;
    use crate::deribit::{RateLimiter, AccountTier};

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_sign_request() {
        let api_key = Box::new(PlainTextSecret::new("test_api_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier1);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            client,
            rate_limiter,
        );

        let signature = rest_client.sign_request(
            "1640995200000",
            "POST",
            "/api/v2/private",
            r#"{"jsonrpc":"2.0","id":1,"method":"private/update_in_address_book","params":{}}"#,
        );

        assert!(signature.is_ok());
        let sig = signature.unwrap();
        // Verify the signature is a hex string of the expected length (64 chars for SHA256)
        assert_eq!(sig.len(), 64);
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_client_creation() {
        let api_key = Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier1);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://test.deribit.com",
            client,
            rate_limiter,
        );

        assert_eq!(rest_client.base_url, "https://test.deribit.com");
    }
}