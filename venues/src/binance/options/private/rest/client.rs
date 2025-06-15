//! Binance Options Private REST API client
//!
//! This module provides a client for authenticated Binance Options API endpoints.
//! The Options API uses the `/eapi/v1/` prefix and requires HMAC-SHA256 signed requests.

use hex;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rest::secrets::ExposableSecret;
use sha2::Sha256;
use std::borrow::Cow;

use crate::binance::options::{Errors, RateLimiter, ResponseHeaders, RestResult};

/// Signs a request using the API secret
fn sign_request(api_secret: &dyn ExposableSecret, query_string: &str) -> Result<String, Errors> {
    let api_secret = api_secret.expose_secret();
    let mut mac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
        .map_err(|_| Errors::InvalidApiKey())?;
    mac.update(query_string.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// A client for interacting with the Binance Options private REST API
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
    /// * `base_url` - The base URL for the API (should be https://eapi.binance.com)
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

    /// Sends a signed request to the Binance Options API
    ///
    /// This method automatically handles timestamp generation and request signing for private endpoints.
    /// It appends the current timestamp and generates the required signature.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/eapi/v1/account")
    /// * `method` - The HTTP method to use
    /// * `request` - The request parameters implementing Serialize
    /// * `weight` - The request weight for this endpoint
    /// * `is_order` - Whether this is an order-related endpoint
    ///
    /// # Returns
    /// A result containing the parsed response data and metadata, or an error
    pub async fn send_signed_request<T, R>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        request: R,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned,
        R: serde::Serialize,
    {
        // Check rate limits before making the request
        self.rate_limiter.check_limits(weight, is_order).await?;

        // Add timestamp to request
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| Errors::Error("Failed to get timestamp".to_string()))?
            .as_millis() as u64;

        // Serialize the request parameters
        let mut serialized = serde_urlencoded::to_string(&request).map_err(|e| {
            Errors::Error(format!("Failed to encode params: {}", e))
        })?;

        // Add timestamp to query string
        if serialized.is_empty() {
            serialized = format!("timestamp={}", timestamp);
        } else {
            serialized = format!("{}&timestamp={}", serialized, timestamp);
        }

        // Sign the request
        let signature = sign_request(self.api_secret.as_ref(), &serialized)?;
        let signed_query = format!("{}&signature={}", serialized, signature);

        // Build the URL
        let url = if method == reqwest::Method::GET {
            format!("{}{}", self.base_url, endpoint)
        } else {
            format!("{}{}", self.base_url, endpoint)
        };

        // Prepare headers
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "X-MBX-APIKEY",
            reqwest::header::HeaderValue::from_str(&self.api_key.expose_secret())
                .map_err(|_| Errors::Error("Invalid API key format".to_string()))?,
        );

        // Make the request
        let request_builder = if method == reqwest::Method::GET {
            self.client.get(&format!("{}?{}", url, signed_query))
        } else {
            let mut builder = self.client.request(method, &url);
            builder = builder.header("Content-Type", "application/x-www-form-urlencoded");
            builder.body(signed_query)
        };

        let start_time = std::time::Instant::now();
        let response = request_builder
            .headers(headers)
            .send()
            .await
            .map_err(|e| Errors::Error(format!("Request failed: {}", e)))?;

        let request_duration = start_time.elapsed();

        // Parse response headers
        let response_headers = self.parse_response_headers(&response);

        // Update rate limiter with response headers
        self.rate_limiter.update_from_headers(&response_headers).await;

        // Handle the response
        let status = response.status();
        let response_text = response
            .text()
            .await
            .map_err(|e| Errors::Error(format!("Failed to read response: {}", e)))?;

        if !status.is_success() {
            return Err(Errors::Error(format!(
                "API request failed with status {}: {}",
                status, response_text
            )));
        }

        // Parse the response
        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;

        // Increment counters after successful request
        self.rate_limiter.increment_raw_request().await;
        if is_order {
            self.rate_limiter.increment_order().await;
        }

        Ok(crate::binance::options::RestResponse {
            data: parsed_response,
            request_duration,
            headers: response_headers,
        })
    }

    /// Parse rate limiting headers from the response
    fn parse_response_headers(&self, response: &reqwest::Response) -> ResponseHeaders {
        let mut headers = ResponseHeaders::default();

        for (name, value) in response.headers() {
            let name_str = name.as_str();
            let name_lower = name_str.to_lowercase();
            if let Some(header) = crate::binance::options::RateLimitHeader::parse(&name_lower) {
                if let Ok(value_str) = value.to_str() {
                    if let Ok(value_int) = value_str.parse::<u32>() {
                        headers.values.insert(header, value_int);
                    }
                }
            }
        }

        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use std::borrow::Cow;

    // Create a simple test secret implementation
    struct TestSecret(String);

    impl rest::secrets::ExposableSecret for TestSecret {
        fn expose_secret(&self) -> String {
            self.0.clone()
        }
    }

    fn create_test_client() -> RestClient {
        let api_key = Box::new(TestSecret("test_api_key".to_string()));
        let api_secret = Box::new(TestSecret("test_api_secret".to_string()));
        let base_url: Cow<'static, str> = "https://eapi.binance.com".into();
        let rate_limiter = crate::binance::options::RateLimiter::new();
        let client = Client::new();

        RestClient::new(api_key, api_secret, base_url, rate_limiter, client)
    }

    #[test]
    fn test_client_creation() {
        let client = create_test_client();
        // Just verify that the client can be created without panicking
        assert_eq!(client.base_url, "https://eapi.binance.com");
    }
}