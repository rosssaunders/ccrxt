/// REST client for Deribit private endpoints.
///
/// Provides access to all private REST API endpoints for Deribit Exchange.
/// All requests are authenticated and require API credentials.
/// Uses JSON-RPC 2.0 protocol.

use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use sha2::Sha256;
use std::borrow::Cow;
use std::sync::atomic::{AtomicU64, Ordering};
use chrono::Utc;

use crate::deribit::{Errors, ErrorResponse, RateLimiter};

/// Private REST client for Deribit exchange
///
/// This client handles all private API endpoints that require authentication.
/// It provides automatic rate limiting, error handling, and request signing
/// using JSON-RPC 2.0 protocol.
pub struct RestClient {
    /// The base URL for the Deribit REST API (e.g., "https://www.deribit.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with Deribit's rate limits for private endpoints.
    pub rate_limiter: RateLimiter,

    /// The encrypted API key.
    pub(crate) api_key: Box<dyn ExposableSecret>,

    /// The encrypted API secret.
    pub(crate) api_secret: Box<dyn ExposableSecret>,

    /// Counter for generating unique request IDs
    request_id_counter: AtomicU64,
}

impl RestClient {
    /// Create a new REST client for Deribit private endpoints
    ///
    /// # Arguments
    /// * `api_key` - Your Deribit API key
    /// * `api_secret` - Your Deribit API secret
    /// * `base_url` - The base URL for the Deribit API (e.g., "https://www.deribit.com")
    /// * `rate_limiter` - Rate limiter for managing request frequency
    /// * `client` - HTTP client for making requests
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        rate_limiter: RateLimiter,
        client: Client,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
            api_key,
            api_secret,
            request_id_counter: AtomicU64::new(1),
        }
    }

    /// Generate the next unique request ID
    pub(crate) async fn next_request_id(&self) -> u64 {
        self.request_id_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Generate authentication signature for Deribit API
    ///
    /// The signature is created by:
    /// 1. Creating the string: timestamp + nonce + requestData
    /// 2. HMAC SHA256 with the API secret  
    /// 3. Hex encoding the result
    ///
    /// # Arguments
    /// * `timestamp` - The timestamp in milliseconds
    /// * `nonce` - A unique nonce string
    /// * `request_data` - The request data string
    ///
    /// # Returns
    /// A result containing the signature as a hex string or an error
    fn generate_signature(
        &self,
        timestamp: i64,
        nonce: &str,
        request_data: &str,
    ) -> Result<String, Errors> {
        let secret_key = self.api_secret.expose_secret();
        let message = format!("{}{}{}", timestamp, nonce, request_data);

        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
            .map_err(|e| Errors::AuthError(format!("Invalid secret key: {}", e)))?;

        mac.update(message.as_bytes());
        let result = mac.finalize();
        Ok(hex::encode(result.into_bytes()))
    }

    /// Send a JSON-RPC request to a private endpoint
    ///
    /// # Arguments
    /// * `method` - The API method name (e.g., "private/verify_block_trade")
    /// * `id` - The request ID
    /// * `params` - Optional request parameters
    ///
    /// # Returns
    /// A result containing the response JSON value or an error
    pub(crate) async fn send_request(
        &self,
        method: &str,
        id: u64,
        params: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Errors> {
        let timestamp = Utc::now().timestamp_millis();
        let nonce = format!("{}{}", timestamp, id); // Simple nonce generation
        
        // Build request data for signature
        let request_data = if let Some(ref p) = params {
            serde_json::to_string(p)?
        } else {
            "{}".to_string()
        };

        let signature = self.generate_signature(timestamp, &nonce, &request_data)?;

        // Build JSON-RPC request body
        let mut request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method
        });

        // Build params object with authentication
        let mut params_obj = if let Some(params) = params {
            params
        } else {
            serde_json::json!({})
        };

        // Add authentication parameters to params
        if let Some(params_map) = params_obj.as_object_mut() {
            params_map.insert("api_key".to_string(), serde_json::Value::String(self.api_key.expose_secret()));
            params_map.insert("timestamp".to_string(), serde_json::Value::Number(timestamp.into()));
            params_map.insert("nonce".to_string(), serde_json::Value::String(nonce));
            params_map.insert("signature".to_string(), serde_json::Value::String(signature));
        }

        request_body["params"] = params_obj;

        let url = format!("{}/api/v2/private/{}", self.base_url, method.trim_start_matches("private/"));

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(Errors::Unknown(format!("HTTP {}: {}", status, response_text)));
        }

        // Parse JSON response
        let json_response: serde_json::Value = serde_json::from_str(&response_text)?;

        // Check for JSON-RPC error
        if json_response.get("error").is_some() {
            let error_response: ErrorResponse = serde_json::from_value(json_response)?;
            return Err(error_response.into());
        }

        Ok(json_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;
    use crate::deribit::rate_limit::AccountTier;

    #[derive(Clone)]
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
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://www.deribit.com",
            rate_limiter,
            client,
        );

        assert_eq!(rest_client.base_url, "https://www.deribit.com");
    }

    #[tokio::test]
    async fn test_request_id_generation() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://www.deribit.com",
            rate_limiter,
            client,
        );

        let id1 = rest_client.next_request_id().await;
        let id2 = rest_client.next_request_id().await;
        let id3 = rest_client.next_request_id().await;

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
    }

    #[test]
    fn test_signature_generation() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://www.deribit.com",
            rate_limiter,
            client,
        );

        let timestamp = 1672534800000;
        let nonce = "test_nonce";
        let request_data = r#"{"test":"data"}"#;

        let signature = rest_client.generate_signature(timestamp, nonce, request_data);
        assert!(signature.is_ok());
        
        let sig = signature.unwrap();
        assert!(!sig.is_empty());
        assert_eq!(sig.len(), 64); // SHA256 hex string length
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_signature_consistency() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret = Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            "https://www.deribit.com",
            rate_limiter,
            client,
        );

        let timestamp = 1672534800000;
        let nonce = "test_nonce";
        let request_data = r#"{"test":"data"}"#;

        let sig1 = rest_client.generate_signature(timestamp, nonce, request_data).unwrap();
        let sig2 = rest_client.generate_signature(timestamp, nonce, request_data).unwrap();

        assert_eq!(sig1, sig2); // Same inputs should produce same signature
    }
}