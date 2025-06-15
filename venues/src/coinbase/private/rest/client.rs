//! REST client for Coinbase Exchange private endpoints.
//!
//! Provides access to all private REST API endpoints for Coinbase Exchange.
//! All requests are authenticated and require API credentials.

use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::Sha256;
use std::borrow::Cow;

use crate::coinbase::{EndpointType, Errors, RateLimiter, RestResult};

/// Private REST client for Coinbase Exchange
///
/// This client handles all private API endpoints that require authentication.
/// It provides automatic rate limiting, error handling, and request signing.
pub struct RestClient {
    /// The base URL for the Coinbase Exchange REST API
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits
    pub rate_limiter: RateLimiter,

    /// The encrypted API key
    pub(crate) api_key: Box<dyn ExposableSecret>,

    /// The encrypted API secret (base64 encoded)
    pub(crate) api_secret: Box<dyn ExposableSecret>,

    /// The encrypted API passphrase
    pub(crate) api_passphrase: Box<dyn ExposableSecret>,
}

impl RestClient {
    /// Create a new REST client for Coinbase Exchange private endpoints
    ///
    /// # Arguments
    /// * `api_key` - Your Coinbase Exchange API key
    /// * `api_secret` - Your Coinbase Exchange API secret (base64 encoded)
    /// * `api_passphrase` - Your Coinbase Exchange API passphrase
    /// * `base_url` - The base URL for the Coinbase Exchange API
    /// * `client` - HTTP client for making requests
    /// * `rate_limiter` - Rate limiter for managing request frequency
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

    /// Create request signature for Coinbase Exchange authentication
    ///
    /// The signature is created by:
    /// 1. Create prehash string: timestamp + method + requestPath + body
    /// 2. Sign with HMAC SHA256 using the API secret (base64 decoded)
    /// 3. Encode as Base64
    ///
    /// # Arguments
    /// * `timestamp` - The timestamp string (Unix timestamp as string)
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
        // Create the prehash string: timestamp + method + requestPath + body
        let prehash = format!("{}{}{}{}", timestamp, method, request_path, body);

        // Decode the base64 API secret
        let api_secret = self.api_secret.expose_secret();
        let secret_bytes = general_purpose::STANDARD
            .decode(&api_secret)
            .map_err(|e| Errors::Error(format!("Failed to decode API secret: {}", e)))?;

        // Sign with HMAC SHA256
        let mut mac = Hmac::<Sha256>::new_from_slice(&secret_bytes)
            .map_err(|_| Errors::InvalidApiKey())?;
        mac.update(prehash.as_bytes());

        // Encode as Base64
        Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
    }

    /// Send a request to a private endpoint and return both data and headers
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `method` - The HTTP method to use
    /// * `params` - Optional query parameters or request body
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    /// A result containing the deserialized response and headers or an error
    pub async fn send_request_with_headers<T, P>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<(T, reqwest::header::HeaderMap)>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        // Check rate limit before making request
        self.rate_limiter.check_limit(endpoint_type).await?;

        // Create timestamp
        let timestamp = Utc::now().timestamp().to_string();

        // Build URL and request
        let url = format!("{}/{}", self.base_url, endpoint);
        let mut request_builder = self.client.request(method.clone(), &url);

        // Handle request body and path
        let (request_path, body) = if method == reqwest::Method::GET {
            // For GET requests, add query parameters
            if let Some(params) = params {
                let query_string = serde_urlencoded::to_string(params)
                    .map_err(|e| Errors::Error(format!("Failed to serialize query parameters: {}", e)))?;
                if !query_string.is_empty() {
                    // Parse the query string and add individual parameters
                    let parsed_params: Vec<(String, String)> = serde_urlencoded::from_str(&query_string)
                        .map_err(|e| Errors::Error(format!("Failed to parse query parameters: {}", e)))?;
                    for (key, value) in &parsed_params {
                        request_builder = request_builder.query(&[(key, value)]);
                    }
                    (format!("/{}?{}", endpoint, query_string), String::new())
                } else {
                    (format!("/{}", endpoint), String::new())
                }
            } else {
                (format!("/{}", endpoint), String::new())
            }
        } else {
            // For POST/PUT/DELETE requests, add JSON body
            let body = if let Some(params) = params {
                serde_json::to_string(params)
                    .map_err(|e| Errors::Error(format!("Failed to serialize request body: {}", e)))?
            } else {
                String::new()
            };

            if !body.is_empty() {
                request_builder = request_builder.body(body.clone());
                request_builder = request_builder.header("Content-Type", "application/json");
            }

            (format!("/{}", endpoint), body)
        };

        // Create signature
        let signature = self.sign_request(&timestamp, method.as_str(), &request_path, &body)?;

        // Add required headers
        let api_key = self.api_key.expose_secret();
        let api_passphrase = self.api_passphrase.expose_secret();

        request_builder = request_builder
            .header("CB-ACCESS-KEY", api_key)
            .header("CB-ACCESS-SIGN", signature)
            .header("CB-ACCESS-TIMESTAMP", timestamp)
            .header("CB-ACCESS-PASSPHRASE", api_passphrase)
            .header("User-Agent", "ccrxt/0.1.0");

        // Send request
        let response = request_builder.send().await?;

        // Check response status and capture headers
        let status = response.status();
        let headers = response.headers().clone();
        let response_text = response.text().await?;

        if status.is_success() {
            // Parse successful response
            let data = serde_json::from_str::<T>(&response_text)
                .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;
            Ok((data, headers))
        } else {
            // Parse error response
            if let Ok(error_response) = serde_json::from_str::<crate::coinbase::ErrorResponse>(&response_text) {
                match status.as_u16() {
                    400 => Err(Errors::ApiError(crate::coinbase::ApiError::BadRequest { msg: error_response.message })),
                    401 => Err(Errors::ApiError(crate::coinbase::ApiError::Unauthorized { msg: error_response.message })),
                    403 => Err(Errors::ApiError(crate::coinbase::ApiError::Forbidden { msg: error_response.message })),
                    404 => Err(Errors::ApiError(crate::coinbase::ApiError::NotFound { msg: error_response.message })),
                    429 => Err(Errors::ApiError(crate::coinbase::ApiError::TooManyRequests { msg: error_response.message })),
                    500 => Err(Errors::ApiError(crate::coinbase::ApiError::InternalServerError { msg: error_response.message })),
                    _ => Err(Errors::ApiError(crate::coinbase::ApiError::UnknownApiError { 
                        code: Some(status.as_u16() as i32), 
                        msg: error_response.message 
                    })),
                }
            } else {
                Err(Errors::Error(format!("HTTP {}: {}", status, response_text)))
            }
        }
    }

    /// Send a request to a private endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `method` - The HTTP method to use
    /// * `params` - Optional query parameters or request body
    /// * `endpoint_type` - The type of endpoint for rate limiting
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
        // Check rate limit before making request
        self.rate_limiter.check_limit(endpoint_type).await?;

        // Create timestamp
        let timestamp = Utc::now().timestamp().to_string();

        // Build URL and request
        let url = format!("{}/{}", self.base_url, endpoint);
        let mut request_builder = self.client.request(method.clone(), &url);

        // Handle request body and path
        let (request_path, body) = if method == reqwest::Method::GET {
            // For GET requests, add query parameters
            if let Some(params) = params {
                let query_string = serde_urlencoded::to_string(params)
                    .map_err(|e| Errors::Error(format!("Failed to serialize query parameters: {}", e)))?;
                if !query_string.is_empty() {
                    // Parse the query string and add individual parameters
                    let parsed_params: Vec<(String, String)> = serde_urlencoded::from_str(&query_string)
                        .map_err(|e| Errors::Error(format!("Failed to parse query parameters: {}", e)))?;
                    for (key, value) in &parsed_params {
                        request_builder = request_builder.query(&[(key, value)]);
                    }
                    (format!("/{}?{}", endpoint, query_string), String::new())
                } else {
                    (format!("/{}", endpoint), String::new())
                }
            } else {
                (format!("/{}", endpoint), String::new())
            }
        } else {
            // For POST/PUT/DELETE requests, add JSON body
            let body = if let Some(params) = params {
                serde_json::to_string(params)
                    .map_err(|e| Errors::Error(format!("Failed to serialize request body: {}", e)))?
            } else {
                String::new()
            };

            if !body.is_empty() {
                request_builder = request_builder.body(body.clone());
                request_builder = request_builder.header("Content-Type", "application/json");
            }

            (format!("/{}", endpoint), body)
        };

        // Create signature
        let signature = self.sign_request(&timestamp, method.as_str(), &request_path, &body)?;

        // Add required headers
        let api_key = self.api_key.expose_secret();
        let api_passphrase = self.api_passphrase.expose_secret();

        request_builder = request_builder
            .header("CB-ACCESS-KEY", api_key)
            .header("CB-ACCESS-SIGN", signature)
            .header("CB-ACCESS-TIMESTAMP", timestamp)
            .header("CB-ACCESS-PASSPHRASE", api_passphrase)
            .header("User-Agent", "ccrxt/0.1.0");

        // Send request
        let response = request_builder.send().await?;

        // Check response status
        let status = response.status();
        let response_text = response.text().await?;

        if status.is_success() {
            // Parse successful response
            serde_json::from_str::<T>(&response_text)
                .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))
        } else {
            // Parse error response
            if let Ok(error_response) = serde_json::from_str::<crate::coinbase::ErrorResponse>(&response_text) {
                match status.as_u16() {
                    400 => Err(Errors::ApiError(crate::coinbase::ApiError::BadRequest { msg: error_response.message })),
                    401 => Err(Errors::ApiError(crate::coinbase::ApiError::Unauthorized { msg: error_response.message })),
                    403 => Err(Errors::ApiError(crate::coinbase::ApiError::Forbidden { msg: error_response.message })),
                    404 => Err(Errors::ApiError(crate::coinbase::ApiError::NotFound { msg: error_response.message })),
                    429 => Err(Errors::ApiError(crate::coinbase::ApiError::TooManyRequests { msg: error_response.message })),
                    500 => Err(Errors::ApiError(crate::coinbase::ApiError::InternalServerError { msg: error_response.message })),
                    _ => Err(Errors::ApiError(crate::coinbase::ApiError::UnknownApiError { 
                        code: Some(status.as_u16() as i32), 
                        msg: error_response.message 
                    })),
                }
            } else {
                Err(Errors::Error(format!("HTTP {}: {}", status, response_text)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Create a simple test secret implementation
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
        let api_secret = Box::new(TestSecret::new("dGVzdF9zZWNyZXQ=".to_string())) as Box<dyn ExposableSecret>;
        let api_passphrase = Box::new(TestSecret::new("test_passphrase".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            api_passphrase,
            "https://api.exchange.coinbase.com",
            client,
            rate_limiter,
        );

        assert_eq!(rest_client.base_url, "https://api.exchange.coinbase.com");
    }

    #[test]
    fn test_signature_generation() {
        let api_key = Box::new(TestSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
        // Base64 encoded test secret
        let api_secret = Box::new(TestSecret::new("dGVzdF9zZWNyZXQ=".to_string())) as Box<dyn ExposableSecret>;
        let api_passphrase = Box::new(TestSecret::new("test_passphrase".to_string())) as Box<dyn ExposableSecret>;
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client = RestClient::new(
            api_key,
            api_secret,
            api_passphrase,
            "https://api.exchange.coinbase.com",
            client,
            rate_limiter,
        );

        let result = rest_client.sign_request(
            "1640995200",
            "GET",
            "/accounts",
            "",
        );

        assert!(result.is_ok());
        let signature = result.unwrap();
        assert!(!signature.is_empty());
    }
}