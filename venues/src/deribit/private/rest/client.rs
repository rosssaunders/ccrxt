use std::{borrow::Cow, sync::Arc};

use chrono::Utc;
use hmac::{Hmac, Mac};
use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
    secrets::ExposableSecret,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;
use sha2::Sha256;

use super::credentials::Credentials;
use crate::deribit::{EndpointType, Errors, RateLimiter, RestResult};

/// Private REST client for Deribit exchange
///
/// This client handles all private API endpoints that require authentication.
/// It provides automatic rate limiting, error handling, and request signing.
pub struct RestClient {
    /// The base URL for the Deribit private REST API
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests
    pub http_client: Arc<dyn HttpClient>,

    /// The rate limiter used to manage request rates
    pub rate_limiter: RateLimiter,

    /// The API credentials for authentication
    pub(crate) credentials: Credentials,
}

impl RestClient {
    /// Creates a new RestClient with encrypted API credentials
    ///
    /// # Arguments
    /// * `credentials` - The API credentials for authentication
    /// * `base_url` - The base URL for the API
    /// * `rate_limiter` - The rate limiter instance
    /// * `http_client` - The HTTP client instance
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        credentials: Credentials,
        base_url: impl Into<Cow<'static, str>>,
        rate_limiter: RateLimiter,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            http_client,
            rate_limiter,
            credentials,
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
    pub fn sign_request(
        &self,
        request_data: &str,
        nonce: u64,
        request_id: u64,
    ) -> Result<String, Errors> {
        // Create the signature payload: request_data + nonce + request_id
        let sig_payload = format!("{request_data}{nonce}{request_id}");

        // Sign with HMAC-SHA256
        let api_secret = self.credentials.api_secret.expose_secret();
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
            .map_err(|_| Errors::InvalidApiKey())?;
        mac.update(sig_payload.as_bytes());

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

        let nonce = Utc::now().timestamp_millis() as u64;
        let request_id = 1;

        // Prepare the JSON-RPC request body for signing
        let request_data = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": method,
            "params": params,
        });
        let request_data_str =
            serde_json::to_string(&request_data).map_err(Errors::SerdeJsonError)?;
        let signature = self.sign_request(&request_data_str, nonce, request_id)?;

        // Prepare authenticated request
        let authenticated_request = json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "method": method,
            "params": params,
            "sig": signature,
            "nonce": nonce,
            "api_key": self.credentials.api_key.expose_secret(),
        });

        // Send HTTP request
        let url = format!("{}/api/v2/{}", self.base_url, method);
        let body = serde_json::to_string(&authenticated_request)
            .map_err(|e| Errors::Error(format!("Failed to serialize body: {e}")))?;

        let request = RequestBuilder::new(HttpMethod::Post, url)
            .header("Content-Type", "application/json")
            .body(body.into_bytes())
            .build();

        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| Errors::NetworkError(format!("HTTP request failed: {e}")))?;

        // Record request for rate limiting
        self.rate_limiter.record_request(endpoint_type).await;

        // Check if the response was successful
        if !(response.status >= 200 && response.status < 300) {
            let status = response.status;
            let error_text = response
                .text()
                .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;
            return Err(Errors::Error(format!("HTTP {status}: {error_text}")));
        }

        // Parse the response
        let response_text = response
            .text()
            .map_err(|e| Errors::NetworkError(format!("Failed to read response: {e}")))?;

        let result: T = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::deribit::AccountTier;
    use rest::secrets::SecretString;

    #[test]
    fn test_private_client_creation() {
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        assert_eq!(rest_client.base_url, "https://test.deribit.com");
    }

    #[test]
    fn test_signature_generation() {
        let credentials = Credentials {
            api_key: SecretString::from("test_key".to_string()),
            api_secret: SecretString::from("test_secret".to_string()),
        };
        let http_client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new(AccountTier::Tier4);

        let rest_client = RestClient::new(
            credentials,
            "https://test.deribit.com",
            rate_limiter,
            http_client,
        );

        let result = rest_client.sign_request("test_data", 1234567890, 1);

        assert!(result.is_ok());
        let signature = result.unwrap();
        assert!(!signature.is_empty());
    }
}
