// REST client for OKX private endpoints.
//
// Provides access to all private REST API endpoints for OKX Exchange.
// All requests are authenticated and require API credentials.
use std::borrow::Cow;

use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use serde::{Serialize, de::DeserializeOwned};
use sha2::Sha256;

use crate::okx::{EndpointType, Errors, RateLimiter, RestResult};

/// Private REST client for OKX exchange
///
/// This client handles all private API endpoints that require authentication.
/// It provides automatic rate limiting, error handling, and request signing.
pub struct RestClient {
    /// The base URL for the OKX private REST API (e.g., "https://www.okx.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with OKX's rate limits for private endpoints.
    pub rate_limiter: RateLimiter,

    /// The encrypted API key.
    pub(crate) api_key: Box<dyn ExposableSecret>,

    /// The encrypted API secret.
    pub(crate) api_secret: Box<dyn ExposableSecret>,

    /// The encrypted API passphrase.
    pub(crate) api_passphrase: Box<dyn ExposableSecret>,
}

impl RestClient {
    /// Creates a new OKX private REST client.
    ///
    /// # Arguments
    /// * `api_key` - The encrypted API key
    /// * `api_secret` - The encrypted API secret  
    /// * `api_passphrase` - The encrypted API passphrase
    /// * `base_url` - The base URL for the OKX private REST API (e.g., "https://www.okx.com")
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter for managing API limits
    pub fn new(
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
        api_passphrase: Box<dyn ExposableSecret>,
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
            api_key,
            api_secret,
            api_passphrase,
        }
    }

    /// Sign a request for OKX private endpoints
    ///
    /// Creates the signature according to OKX's signing algorithm:
    /// 1. Create pre-hash string: timestamp + method + requestPath + body
    /// 2. Sign with HMAC SHA256 using the API secret
    /// 3. Encode as Base64
    ///
    /// # Arguments
    /// * `timestamp` - The timestamp string (ISO format)
    /// * `method` - The HTTP method (uppercase)
    /// * `request_path` - The request path including query parameters
    /// * `body` - The request body (empty string for GET requests)
    ///
    /// # Returns
    /// A result containing the signature as a base64 string or an error
    pub fn sign_request(
        &self,
        timestamp: &str,
        method: &str,
        request_path: &str,
        body: &str,
    ) -> Result<String, Errors> {
        // Create the pre-hash string: timestamp + method + requestPath + body
        let pre_hash = format!("{timestamp}{method}{request_path}{body}");

        // Sign with HMAC SHA256
        let api_secret = self.api_secret.expose_secret();
        let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
            .map_err(|_| Errors::InvalidApiKey())?;
        mac.update(pre_hash.as_bytes());

        // Encode as Base64
        Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
    }

    /// Send a request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "api/v5/trade/order")
    /// * `method` - The HTTP method to use
    /// * `params` - Optional struct of query/body parameters (must implement Serialize)
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_request<T, P>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        // Check rate limits
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        // Build URL
        let url = format!("{}/{}", self.base_url.trim_end_matches('/'), endpoint);

        // Create timestamp
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

        // Prepare request
        let mut request_builder = self.client.request(method.clone(), &url);

        // Handle query parameters for GET requests or body for POST/PUT/DELETE
        let (request_path, body) = if method == reqwest::Method::GET {
            if let Some(params) = params {
                let query_string = serde_urlencoded::to_string(params).map_err(|e| {
                    Errors::Error(format!("Failed to serialize query parameters: {e}"))
                })?;
                if !query_string.is_empty() {
                    request_builder = request_builder.query(&query_string);
                    (format!("/{endpoint}?{query_string}"), String::new())
                } else {
                    (format!("/{endpoint}"), String::new())
                }
            } else {
                (format!("/{endpoint}"), String::new())
            }
        } else {
            let body = if let Some(params) = params {
                serde_json::to_string(params)
                    .map_err(|e| Errors::Error(format!("Failed to serialize request body: {e}")))?
            } else {
                String::new()
            };

            if !body.is_empty() {
                request_builder = request_builder.body(body.clone());
                request_builder = request_builder.header("Content-Type", "application/json");
            }

            (format!("/{endpoint}"), body)
        };

        // Create signature
        let signature = self.sign_request(&timestamp, method.as_str(), &request_path, &body)?;

        // Add required headers
        let api_key = self.api_key.expose_secret();
        let api_passphrase = self.api_passphrase.expose_secret();

        request_builder = request_builder
            .header("OK-ACCESS-KEY", api_key.as_str())
            .header("OK-ACCESS-SIGN", &signature)
            .header("OK-ACCESS-TIMESTAMP", &timestamp)
            .header("OK-ACCESS-PASSPHRASE", api_passphrase.as_str());

        // Send request
        let response = request_builder.send().await?;

        // Record request for rate limiting
        self.rate_limiter.increment_request(endpoint_type).await;

        // Handle response
        if response.status().is_success() {
            let response_text = response.text().await?;

            // Parse the response
            let parsed: crate::okx::response::OkxApiResponse<T> = serde_json::from_str(&response_text)
                .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;

            Ok(parsed)
        } else {
            let status = response.status();
            let error_text = response.text().await?;
            Err(Errors::Error(format!("HTTP {status}: {error_text}")))
        }
    }

    /// Send a GET request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "api/v5/account/balance")
    /// * `params` - Optional query parameters
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_get_request<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_request(endpoint, reqwest::Method::GET, Some(&params), endpoint_type)
            .await
    }

    /// Send a POST request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "api/v5/trade/order")
    /// * `params` - Request body parameters
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_post_request<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_request(
            endpoint,
            reqwest::Method::POST,
            Some(&params),
            endpoint_type,
        )
        .await
    }

    /// Send a PUT request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path
    /// * `params` - Request body parameters
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_put_request<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_request(endpoint, reqwest::Method::PUT, Some(&params), endpoint_type)
            .await
    }

    /// Send a DELETE request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path
    /// * `params` - Optional query parameters or request body
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_delete_request<T, P>(
        &self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_request(
            endpoint,
            reqwest::Method::DELETE,
            Some(&params),
            endpoint_type,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let api_secret =
            Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let api_passphrase =
            Box::new(TestSecret::new("test_passphrase".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            api_passphrase,
            "https://www.okx.com",
            client,
            rate_limiter,
        );

        assert_eq!(rest_client.base_url, "https://www.okx.com");
    }

    #[test]
    fn test_signature_generation() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let api_passphrase =
            Box::new(TestSecret::new("test_passphrase".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            api_passphrase,
            "https://www.okx.com",
            client,
            rate_limiter,
        );

        let timestamp = "2020-12-08T09:08:57.715Z";
        let method = "GET";
        let request_path = "/api/v5/account/balance?ccy=BTC";
        let body = "";

        let signature = rest_client
            .sign_request(timestamp, method, request_path, body)
            .unwrap();

        // Verify the signature is a valid base64 string
        assert!(general_purpose::STANDARD.decode(&signature).is_ok());
    }

    #[tokio::test]
    async fn test_rate_limiting_integration() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        let api_secret =
            Box::new(TestSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
        let api_passphrase =
            Box::new(TestSecret::new("test_passphrase".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            api_passphrase,
            "https://www.okx.com",
            client,
            rate_limiter,
        );

        // Verify rate limiting works
        let result = rest_client
            .rate_limiter
            .check_limits(EndpointType::PrivateTrading)
            .await;
        assert!(result.is_ok());
    }
}
