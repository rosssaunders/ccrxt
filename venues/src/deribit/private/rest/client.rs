use std::borrow::Cow;

use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use serde::{Serialize, de::DeserializeOwned};
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

    /// Signs a request for Deribit private endpoints using the deri-hmac-sha256 method
    ///
    /// The Deribit signing algorithm:
    /// 1. RequestData = UPPERCASE(HTTP_METHOD) + "\n" + URI + "\n" + RequestBody + "\n"
    /// 2. StringToSign = Timestamp + "\n" + Nonce + "\n" + RequestData
    /// 3. Signature = HEX_STRING( HMAC-SHA256( ClientSecret, StringToSign ) )
    ///
    /// # Arguments
    /// * `http_method` - The HTTP method (GET, POST, etc.)
    /// * `uri` - The request URI path
    /// * `body` - The request body (can be empty string)
    /// * `timestamp` - The timestamp in milliseconds
    /// * `nonce` - The nonce value
    ///
    /// # Returns
    /// A result containing the signature as a hex string or an error
    pub fn sign_request(
        &self,
        http_method: &str,
        uri: &str,
        body: &str,
        timestamp: u64,
        nonce: &str,
    ) -> Result<String, Errors> {
        // Create RequestData = UPPERCASE(HTTP_METHOD) + "\n" + URI + "\n" + RequestBody + "\n"
        let request_data = format!("{}\n{}\n{}\n", http_method.to_uppercase(), uri, body);
        
        // Create StringToSign = Timestamp + "\n" + Nonce + "\n" + RequestData
        let string_to_sign = format!("{}\n{}\n{}", timestamp, nonce, request_data);

        // Sign with HMAC-SHA256
        let api_secret = self.api_secret.expose_secret();
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
            .map_err(|_| Errors::InvalidApiKey())?;
        mac.update(string_to_sign.as_bytes());

        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    /// Send a signed private request to Deribit API, handling serialization and rate limiting.
    pub async fn send_signed_request<T, P>(
        &self,
        method: &str,
        params: &P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        // Rate limiting
        self.rate_limiter.check_limits(endpoint_type).await?;

        let timestamp = Utc::now().timestamp_millis() as u64;
        let request_id = 1;
        
        // Generate a nonce using timestamp + counter
        let nonce = format!("{}{}", timestamp, request_id);

        // Prepare the JSON-RPC request body
        let request_body = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": method,
            "params": params,
        });
        let body_str = serde_json::to_string(&request_body).map_err(Errors::SerdeJsonError)?;
        
        // Create the URI path
        let uri = format!("/api/v2/{}", method);
        
        // Generate the signature
        let signature = self.sign_request("POST", &uri, &body_str, timestamp, &nonce)?;
        
        // Create the Authorization header
        let auth_header = format!(
            "deri-hmac-sha256 id={},ts={},sig={},nonce={}",
            self.api_key.expose_secret(),
            timestamp,
            signature,
            nonce
        );

        // Send HTTP request with Authorization header
        let resp = self
            .client
            .post(format!("{}{}", self.base_url, uri))
            .header("Authorization", auth_header)
            .json(&request_body)
            .send()
            .await?;

        // Record request for rate limiting
        self.rate_limiter.record_request(endpoint_type).await;

        // Deserialize response directly to the expected type T (which is JsonRpcResult<SomeType>)
        // We need to handle potential JSON-RPC error responses
        let resp_text = resp.text().await?;
        
        
        // Check if the response contains an error field
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&resp_text) {
            if let Some(error_obj) = json_value.get("error") {
                // Parse the error
                if let Ok(error) = serde_json::from_value::<crate::deribit::JsonRpcError>(error_obj.clone()) {
                    let error_response = crate::deribit::errors::ErrorResponse {
                        code: error.code,
                        message: error.message,
                        data: error.data,
                    };
                    return Err(Errors::ApiError(error_response.into()));
                }
            }
        }
        
        // If not an error, parse as the expected response type
        let result: T = serde_json::from_str(&resp_text)
            .map_err(Errors::SerdeJsonError)?;
            
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
        let api_secret =
            Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
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
        let api_secret =
            Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
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
