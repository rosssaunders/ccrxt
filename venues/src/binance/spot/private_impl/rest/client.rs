//! Binance Spot API request handling module.
//!
//! This module provides functionality for making HTTP requests to the Binance Spot API.
//! It handles authentication, rate limiting headers, error responses, and request/response timing.
//!
//! ## Binance Exchange Behavior
//!
//! The Binance API has specific behaviors that this module handles:
//!
//! - **Dual Error Format**: Binance can return errors in two ways:
//!   1. HTTP error status codes with error JSON in the body
//!   2. HTTP 200 OK with error details in the response JSON (disguised errors)
//!
//! - **Rate Limiting Headers**: Binance includes rate limiting information in response headers:
//!   - `X-MBX-USED-WEIGHT-1M`: API weight used in the last minute
//!   - `X-MBX-ORDER-COUNT-10S`: Orders placed in the last 10 seconds  
//!   - `X-MBX-ORDER-COUNT-1D`: Orders placed in the last day
//!
//! - **Authentication**: Requires API key in `X-MBX-APIKEY` header for authenticated endpoints
//!
//! - **Timestamp Requirements**: Signed requests must include a timestamp parameter and signature
//!   based on the current UTC timestamp in milliseconds
//!
//! - **Request Signing**: For private endpoints, query parameters (including timestamp) must be
//!   signed using HMAC-SHA256 with the API secret
use std::borrow::Cow;

use hex;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use sha2::Sha256;

use crate::binance::spot::{Errors, RateLimiter, RestResult};

/// Signs a request using the decrypted API secret
/// Signs a query string using the decrypted API secret and returns the signature as a hex string.
///
/// # Arguments
/// * `query_string` - The query string to sign
///
/// # Returns
/// A result containing the signature as a hex string or a Hmac error if signing fails.
fn sign_request(api_secret: &dyn ExposableSecret, query_string: &str) -> Result<String, Errors> {
    let api_secret = api_secret.expose_secret();
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes()).map_err(|_| Errors::InvalidApiKey())?;
    mac.update(query_string.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// A client for interacting with the Binance Spot private REST API
///
/// This client handles encrypted API keys and secrets for enhanced security.
/// The API key and secret are stored in encrypted form and only decrypted when needed.
#[non_exhaustive]
pub struct RestClient {
    /// The underlying HTTP client used for making requests.
    pub(crate) client: Client,
    /// The rate limiter for this client.
    pub(crate) rate_limiter: RateLimiter,
    /// The encrypted API key.
    pub(crate) api_key: Box<dyn ExposableSecret>,
    /// The encrypted API secret.
    pub(crate) api_secret: Box<dyn ExposableSecret>,
    /// The base URL for the API.
    pub(crate) base_url: Cow<'static, str>,
}

impl RestClient {
    /// Creates a new RestClient with encrypted API credentials
    ///
    /// # Arguments
    /// * `api_key` - The encrypted API key
    /// * `api_secret` - The encrypted API secret
    /// * `base_url` - The base URL for the API
    /// * `rate_limiter` - The rate limiter instance
    /// * `client` - The HTTP client to use
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
            client,
            rate_limiter,
            api_key,
            api_secret,
            base_url: base_url.into(),
        }
    }

    /// Sends a request to the Binance Spot API
    ///
    /// This method encapsulates all the logic for making authenticated requests to the Binance API,
    /// including rate limiting, error handling, and response parsing.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/api/v3/order")
    /// * `method` - The HTTP method to use
    /// * `query_string` - Optional query string parameters (for GET or for URL params)
    /// * `body` - Optional body (for POST/PUT/DELETE with x-www-form-urlencoded)
    /// * `weight` - The request weight for this endpoint
    /// * `is_order` - Whether this is an order-related endpoint
    ///
    /// # Returns
    /// A result containing the parsed response data and metadata, or an error
    pub(super) async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        body: Option<&[(&str, &str)]>,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        // Add timestamp to query params for signing
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let mut query_params = vec![("timestamp".to_string(), timestamp)];

        // Add any additional query parameters
        if let Some(qs) = query_string {
            // Parse existing query string and add to params
            let parsed: Vec<(String, String)> = serde_urlencoded::from_str(qs).map_err(|e| Errors::Error(format!("Invalid query string: {}", e)))?;
            query_params.extend(parsed);
        }

        // Add body parameters to query params for signing
        if let Some(body_params) = body {
            for (k, v) in body_params {
                query_params.push((k.to_string(), v.to_string()));
            }
        }

        // Build query string for signing
        let query_for_signing = serde_urlencoded::to_string(&query_params).map_err(|e| Errors::Error(format!("Failed to encode query string: {}", e)))?;

        // Sign the request
        let signature = sign_request(self.api_secret.as_ref(), &query_for_signing)?;
        query_params.push(("signature".to_string(), signature));

        // Final query string with signature
        let final_query_string =
            serde_urlencoded::to_string(&query_params).map_err(|e| Errors::Error(format!("Failed to encode final query string: {}", e)))?;

        let url = crate::binance::spot::rest::common::build_url(&self.base_url, endpoint, Some(&final_query_string))?;

        let mut headers = vec![];
        let api_key = self.api_key.expose_secret();
        if !api_key.is_empty() {
            headers.push(("X-MBX-APIKEY", api_key));
        }

        let body_data = match body {
            Some(b) => Some(serde_urlencoded::to_string(b).map_err(|e| Errors::Error(format!("URL encoding error: {}", e)))?),
            None => None,
        };

        if body_data.is_some() {
            headers.push((
                "Content-Type",
                "application/x-www-form-urlencoded".to_string(),
            ));
        }

        let rest_response = crate::binance::spot::rest::common::send_rest_request(
            &self.client,
            &url,
            method,
            headers,
            body_data.as_deref(),
            &self.rate_limiter,
            weight,
            is_order,
        )
        .await?;

        Ok(crate::binance::spot::RestResponse {
            data: rest_response.data,
            headers: rest_response.headers,
        })
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;

    use super::*;

    // Create a simple test secret implementation
    #[derive(Clone)]
    struct TestSecret {
        value: String,
    }

    impl ExposableSecret for TestSecret {
        fn expose_secret(&self) -> String {
            self.value.clone()
        }
    }

    impl TestSecret {
        fn new(value: String) -> Self {
            Self { value }
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
            "https://api.binance.com",
            rate_limiter,
            client,
        );

        assert_eq!(rest_client.base_url, "https://api.binance.com");
    }

    #[test]
    fn test_request_signing() {
        let secret = TestSecret::new("NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j".to_string());
        let query_string = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";

        let result = sign_request(&secret, query_string);
        assert!(result.is_ok());

        let signature = result.unwrap();
        // This is the expected signature from Binance API documentation
        assert_eq!(
            signature,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71"
        );
    }
}
