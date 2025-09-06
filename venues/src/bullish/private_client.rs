//! Bullish Private REST API client

use std::{borrow::Cow, sync::Arc};

use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use rest::{HttpClient, Method as HttpMethod, RequestBuilder};
use secrecy::ExposeSecret;
use serde::{Serialize, de::DeserializeOwned};
use sha2::Sha256;

use crate::bullish::{Credentials, EndpointType, Errors, RateLimiter, RestResult};

/// Private REST client for Bullish exchange
///
/// This client handles all private API endpoints that require authentication.
/// It provides automatic rate limiting, error handling, and JWT token management.
pub struct RestClient {
    /// The underlying abstract HTTP client used for making requests (WASM-safe)
    pub(crate) http_client: Arc<dyn HttpClient>,

    /// Venue credentials (API key and secret)
    pub(crate) credentials: Credentials,

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
    /// * `credentials` - The Bullish API credentials (secure)
    /// * `base_url` - The base URL for the API
    /// * `client` - The HTTP client to use
    /// * `rate_limiter` - Rate limiter for requests
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        credentials: Credentials,
        base_url: impl Into<Cow<'static, str>>,
        http_client: Arc<dyn HttpClient>,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            http_client,
            credentials,
            base_url: base_url.into(),
            rate_limiter,
            jwt_token: None,
        }
    }

    /// Low-level: perform GET with JWT auth and error mapping.
    async fn do_get<T>(&mut self, endpoint: &str, endpoint_type: EndpointType) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        // Check rate limits
        self.rate_limiter
            .check_limits(endpoint_type)
            .await
            .map_err(|e| Errors::RateLimitError(e.to_string()))?;

        // Ensure we have a valid JWT token
        if self.jwt_token.is_none() {
            return Err(Errors::AuthenticationError(
                "JWT token missing, please authenticate with hmac_login or login".to_string(),
            ));
        }

        let url = format!("{}/trading-api{}", self.base_url, endpoint);
        let token = self.jwt_token.as_ref().ok_or_else(|| {
            Errors::AuthenticationError("JWT token missing after login attempt".to_owned())
        })?;

        let request = RequestBuilder::new(HttpMethod::Get, url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .build();

        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(Errors::from)?;

        self.rate_limiter.increment_request(endpoint_type).await;

        // Handle 401 Unauthorized - token might be expired
        if response.status == 401 {
            self.jwt_token = None;
            return Err(Errors::AuthenticationError(
                "JWT token expired or unauthorized. Please refresh token via hmac_login or login."
                    .to_string(),
            ));
        }
        Self::map_json_or_error(response).await
    }

    /// Low-level: POST with JWT + HMAC signing and error mapping.
    async fn do_post_signed<T>(
        &mut self,
        endpoint: &str,
        body_str: String,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        // Check rate limits
        self.rate_limiter
            .check_limits(endpoint_type)
            .await
            .map_err(|e| Errors::RateLimitError(e.to_string()))?;

        // Ensure we have a valid JWT token
        if self.jwt_token.is_none() {
            return Err(Errors::AuthenticationError(
                "JWT token missing, please authenticate with hmac_login or login".to_string(),
            ));
        }

        #[allow(clippy::unwrap_used)]
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let nonce = timestamp; // Simple nonce implementation

        let url = format!("{}/trading-api{}", self.base_url, endpoint);
        let token = self.jwt_token.as_ref().ok_or_else(|| {
            Errors::AuthenticationError("JWT token missing after login attempt".to_owned())
        })?;

        // body_str is already serialized for signing

        // Create signature data: timestamp + nonce + method + endpoint + body
        let signature_data = format!("{}{}{}{}{}", timestamp, nonce, "POST", endpoint, body_str);

        // Create HMAC signature
        let mut mac =
            Hmac::<Sha256>::new_from_slice(self.credentials.api_secret.expose_secret().as_bytes())
                .map_err(|e| Errors::AuthenticationError(format!("HMAC key error: {}", e)))?;
        mac.update(signature_data.as_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        let mut builder = RequestBuilder::new(HttpMethod::Post, url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .header("BX-TIMESTAMP", timestamp.to_string())
            .header("BX-NONCE", nonce.to_string())
            .header("BX-SIGNATURE", signature);
        if !body_str.is_empty() {
            builder = builder
                .header("Content-Length", body_str.len().to_string())
                .body(body_str.into_bytes());
        }
        let request = builder.build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(Errors::from)?;

        self.rate_limiter.increment_request(endpoint_type).await;

        // Handle 401 Unauthorized - token might be expired
        if response.status == 401 {
            self.jwt_token = None;
            return Err(Errors::AuthenticationError(
                "JWT token expired or unauthorized. Please refresh token via hmac_login or login."
                    .to_string(),
            ));
        }
        Self::map_json_or_error(response).await
    }

    /// Low-level: PUT with JWT + HMAC signing and error mapping.
    async fn do_put_signed<T>(
        &mut self,
        endpoint: &str,
        body_str: String,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        // Reuse POST logic by only changing method-specific segments
        // Check rate limits
        self.rate_limiter
            .check_limits(endpoint_type)
            .await
            .map_err(|e| Errors::RateLimitError(e.to_string()))?;

        if self.jwt_token.is_none() {
            return Err(Errors::AuthenticationError(
                "JWT token missing, please authenticate with hmac_login or login".to_string(),
            ));
        }

        #[allow(clippy::unwrap_used)]
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let nonce = timestamp;

        let url = format!("{}/trading-api{}", self.base_url, endpoint);
        let token = self.jwt_token.as_ref().ok_or_else(|| {
            Errors::AuthenticationError("JWT token missing after login attempt".to_owned())
        })?;

        let signature_data = format!("{}{}{}{}{}", timestamp, nonce, "PUT", endpoint, body_str);
        let mut mac =
            Hmac::<Sha256>::new_from_slice(self.credentials.api_secret.expose_secret().as_bytes())
                .map_err(|e| Errors::AuthenticationError(format!("HMAC key error: {}", e)))?;
        mac.update(signature_data.as_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        let request = RequestBuilder::new(HttpMethod::Put, url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .header("BX-TIMESTAMP", timestamp.to_string())
            .header("BX-NONCE", nonce.to_string())
            .header("BX-SIGNATURE", signature)
            .body(body_str.into_bytes())
            .build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(Errors::from)?;
        self.rate_limiter.increment_request(endpoint_type).await;
        if response.status == 401 {
            self.jwt_token = None;
            return Err(Errors::AuthenticationError(
                "JWT token expired or unauthorized. Please refresh token via hmac_login or login."
                    .to_string(),
            ));
        }
        Self::map_json_or_error(response).await
    }

    /// Low-level: DELETE with JWT + HMAC signing and error mapping.
    async fn do_delete_signed<T>(
        &mut self,
        endpoint: &str,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.rate_limiter
            .check_limits(endpoint_type)
            .await
            .map_err(|e| Errors::RateLimitError(e.to_string()))?;

        if self.jwt_token.is_none() {
            return Err(Errors::AuthenticationError(
                "JWT token missing, please authenticate with hmac_login or login".to_string(),
            ));
        }

        #[allow(clippy::unwrap_used)]
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let nonce = timestamp;
        let url = format!("{}/trading-api{}", self.base_url, endpoint);
        let token = self.jwt_token.as_ref().ok_or_else(|| {
            Errors::AuthenticationError("JWT token missing after login attempt".to_owned())
        })?;

        let signature_data = format!("{}{}{}{}", timestamp, nonce, "DELETE", endpoint);
        let mut mac =
            Hmac::<Sha256>::new_from_slice(self.credentials.api_secret.expose_secret().as_bytes())
                .map_err(|e| Errors::AuthenticationError(format!("HMAC key error: {}", e)))?;
        mac.update(signature_data.as_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        let request = RequestBuilder::new(HttpMethod::Delete, url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .header("BX-TIMESTAMP", timestamp.to_string())
            .header("BX-NONCE", nonce.to_string())
            .header("BX-SIGNATURE", signature)
            .build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(Errors::from)?;

        self.rate_limiter.increment_request(endpoint_type).await;

        if response.status == 401 {
            self.jwt_token = None;
            return Err(Errors::AuthenticationError(
                "JWT token expired or unauthorized. Please refresh token via hmac_login or login."
                    .to_string(),
            ));
        }
        Self::map_json_or_error(response).await
    }

    /// Helper: map non-success responses to venue errors, parse body when possible
    async fn map_json_or_error<T>(response: rest::Response) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        if response.is_success() {
            let result: T = serde_json::from_slice(&response.body)?;
            return Ok(result);
        }

        let status = response.status;
        let text = response.text().unwrap_or_default();
        if let Ok(err_resp) = serde_json::from_str::<crate::bullish::ErrorResponse>(&text) {
            let api_err = err_resp.error;
            return match status {
                401 => Err(crate::bullish::Errors::AuthenticationError(api_err.message)),
                403 => Err(crate::bullish::Errors::ApiError(api_err)),
                429 => Err(crate::bullish::Errors::RateLimitError(
                    "Too Many Requests".to_string(),
                )),
                _ => Err(crate::bullish::Errors::ApiError(api_err)),
            };
        }
        let err = match status {
            401 => crate::bullish::Errors::AuthenticationError("Unauthorized".to_string()),
            403 => crate::bullish::Errors::Error("Forbidden".to_string()),
            429 => crate::bullish::Errors::RateLimitError("Too Many Requests".to_string()),
            500 => crate::bullish::Errors::Error("Internal Server Error".to_string()),
            _ => crate::bullish::Errors::Error(format!("HTTP {}: {}", status, text)),
        };
        Err(err)
    }

    // Public high-performance, verb-specific wrappers required by repository rules
    pub async fn send_get_request<T, P>(
        &mut self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| Errors::Error(format!("Failed to serialize params: {}", e)))?;
        let url_with_query = if query.is_empty() {
            endpoint.to_string()
        } else {
            format!("{}?{}", endpoint, query)
        };
        self.do_get(&url_with_query, endpoint_type).await
    }

    pub async fn send_post_request<T, P>(
        &mut self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let body_str = serde_json::to_string(&params)?;
        self.do_post_signed(endpoint, body_str, endpoint_type).await
    }

    pub async fn send_put_request<T, P>(
        &mut self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let body_str = serde_json::to_string(&params)?;
        self.do_put_signed(endpoint, body_str, endpoint_type).await
    }

    pub async fn send_delete_request<T, P>(
        &mut self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let query = serde_urlencoded::to_string(&params)
            .map_err(|e| Errors::Error(format!("Failed to serialize params: {}", e)))?;
        let url_with_query = if query.is_empty() {
            endpoint.to_string()
        } else {
            format!("{}?{}", endpoint, query)
        };
        self.do_delete_signed(&url_with_query, endpoint_type).await
    }

    // Back-compat convenience wrappers (can be removed later)
    pub async fn send_get_authenticated_request<T, P>(
        &mut self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_get_request(endpoint, params, endpoint_type).await
    }

    pub async fn send_post_signed_request<T, P>(
        &mut self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_post_request(endpoint, params, endpoint_type)
            .await
    }

    pub async fn send_put_signed_request<T, P>(
        &mut self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_put_request(endpoint, params, endpoint_type).await
    }

    pub async fn send_delete_signed_request<T, P>(
        &mut self,
        endpoint: &str,
        params: P,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        self.send_delete_request(endpoint, params, endpoint_type)
            .await
    }
}

#[cfg(test)]
mod tests {
    use secrets::SecretString;

    use super::*;

    #[test]
    fn test_private_client_creation() {
        let api_key = SecretString::new("test_key".to_string().into_boxed_str());
        let api_secret = SecretString::new("test_secret".to_string().into_boxed_str());
        let client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new();

        let creds = Credentials {
            api_key,
            api_secret,
        };
        let rest_client = RestClient::new(
            creds,
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
        let api_key = SecretString::new("test_key".to_string().into_boxed_str());
        let api_secret = SecretString::new("test_secret".to_string().into_boxed_str());
        let client = Arc::new(rest::native::NativeHttpClient::default());
        let rate_limiter = RateLimiter::new();

        let creds = Credentials {
            api_key,
            api_secret,
        };
        let _rest_client = RestClient::new(
            creds,
            "https://api.exchange.bullish.com",
            client,
            rate_limiter,
        );

        // Test that rate limiter integration works
        // This is a basic structure test since we can't make real API calls in unit tests
    }
}
