use std::borrow::Cow;

use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::json;
use sha2::Sha256;

use crate::deribit::{EndpointType, Errors, RateLimiter, RestResult};

/// Private REST client for Deribit exchange
///
/// This client handles all private API endpoints that require authentication.
/// It provides automatic rate limiting, error handling, and request signing.
pub struct RestClient {
    /// The base URL for the Deribit private REST API
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests
    pub client: Client,

    /// The rate limiter used to manage request rates
    pub rate_limiter: RateLimiter,

    /// The encrypted API key
    pub(crate) api_key: Box<dyn ExposableSecret>,

    /// The encrypted API secret
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
            base_url: base_url.into(),
            client,
            rate_limiter,
            api_key,
            api_secret,
        }
    }

    /// Signs a request for Deribit private endpoints
    ///
    /// The Deribit signing algorithm:
    /// 1. Create a message string: request_data + nonce + request_id
    /// 2. Use HMAC-SHA256 to hash using API Secret as cryptographic key
    /// 3. Encode output as hex string
    ///
    /// # Arguments
    /// * `request_data` - The request data as JSON string
    /// * `nonce` - The nonce value
    /// * `request_id` - The request ID
    ///
    /// # Returns
    /// A result containing the signature as a hex string or an error
    pub fn sign_request(&self, request_data: &str, nonce: u64, request_id: u64) -> Result<String, Errors> {
        // Create the signature payload: request_data + nonce + request_id
        let sig_payload = format!("{}{}{}", request_data, nonce, request_id);

        // Sign with HMAC-SHA256
        let api_secret = self.api_secret.expose_secret();
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).map_err(|_| Errors::InvalidApiKey())?;
        mac.update(sig_payload.as_bytes());

        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    /// Send a signed private request to Deribit API, handling serialization and rate limiting.
    pub async fn send_signed_request<T, P>(&self, method: &str, params: &P, endpoint_type: EndpointType) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        // Rate limiting
        self.rate_limiter.check_limits(endpoint_type).await?;

        let nonce = Utc::now().timestamp_millis() as u64;
        let request_id = 1;

        // Prepare the JSON-RPC request body for signing
        let request_data = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": method,
            "params": params,
        });
        let request_data_str = serde_json::to_string(&request_data).map_err(Errors::SerdeJsonError)?;
        let signature = self.sign_request(&request_data_str, nonce, request_id)?;

        // Prepare authenticated request
        let authenticated_request = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": method,
            "params": params,
            "sig": signature,
            "nonce": nonce,
            "api_key": self.api_key.expose_secret(),
        });

        // Send HTTP request
        let resp = self
            .client
            .post(format!("{}/api/v2/{}", self.base_url, method))
            .json(&authenticated_request)
            .send()
            .await?;

        // Record request for rate limiting
        self.rate_limiter.record_request(endpoint_type).await;

        // Deserialize response
        let result = resp.json::<T>().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deribit::AccountTier;

    // Test secret implementation
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
            "https://test.deribit.com",
            rate_limiter,
            client,
        );

        assert_eq!(rest_client.base_url, "https://test.deribit.com");
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
            "https://test.deribit.com",
            rate_limiter,
            client,
        );

        let result = rest_client.sign_request("test_data", 1234567890, 1);

        assert!(result.is_ok());
        let signature = result.unwrap();
        assert!(!signature.is_empty());
    }
}
