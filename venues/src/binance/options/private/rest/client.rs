use std::borrow::Cow;

use reqwest::Client;

use rest::secrets::ExposableSecret;

use crate::binance::options::{Errors, RateLimiter, ResponseHeaders, RestResult};
use crate::binance::shared::BinanceRestClient;

/// Private REST client for Binance Options API (EAPI)
///
/// This client provides access to all private endpoints for the Binance Options API,
/// including account information, order management, position queries, and market maker
/// functionality.
pub struct RestClient {
    /// The underlying HTTP client used for making requests
    pub(crate) client: Client,

    /// The rate limiter for this client
    pub(crate) rate_limiter: RateLimiter,

    /// The encrypted API key
    pub(crate) api_key: Box<dyn ExposableSecret>,

    /// The encrypted API secret
    pub(crate) api_secret: Box<dyn ExposableSecret>,

    /// The base URL for the API
    pub(crate) base_url: Cow<'static, str>,
}

impl RestClient {
    /// Create a new private REST client for Binance Options API
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Binance Options API (typically "https://eapi.binance.com")
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter for this client
    /// * `api_key` - The encrypted API key
    /// * `api_secret` - The encrypted API secret
    pub fn new(
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: RateLimiter,
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
    ) -> Self {
        Self {
            client,
            rate_limiter,
            api_key,
            api_secret,
            base_url: base_url.into(),
        }
    }

    /// Internal method to send HTTP requests with rate limiting and error handling
    pub(super) async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        body: Option<&str>,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        let start = std::time::Instant::now();

        // Check rate limits before making request
        self.rate_limiter.check_limits(weight, is_order).await?;

        let url = if let Some(query) = query_string {
            format!("{}{endpoint}?{query}", self.base_url)
        } else {
            format!("{}{endpoint}", self.base_url)
        };

        let mut request_builder = self.client.request(method, &url);

        // Add API key header
        request_builder = request_builder.header("X-MBX-APIKEY", self.api_key.expose_secret());

        // Add body for non-GET requests
        if let Some(body_content) = body {
            request_builder = request_builder
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body_content.to_string());
        }

        let response = request_builder.send().await.map_err(Errors::HttpError)?;

        // Parse response headers
        let headers = ResponseHeaders::from_reqwest_headers(response.headers());

        let status = response.status();
        let response_text = response.text().await.map_err(Errors::HttpError)?;

        // Handle HTTP status codes
        if !status.is_success() {
            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(Errors::Error("Rate limit exceeded".to_string()));
            }
            if status.is_client_error() || status.is_server_error() {
                // Try to parse as error response
                if let Ok(error_response) =
                    serde_json::from_str::<crate::binance::options::ErrorResponse>(&response_text)
                {
                    return Err(Errors::ApiError(
                        crate::binance::options::ApiError::from_code(
                            error_response.code,
                            error_response.msg,
                        ),
                    ));
                }
                return Err(Errors::Error(format!("HTTP {}: {}", status, response_text)));
            }
        }

        // Parse successful response
        let data: T = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;

        // Update rate limiter from response headers
        self.rate_limiter.update_from_headers(&headers).await;

        // Record successful usage
        self.rate_limiter.increment_raw_request().await;
        if is_order {
            self.rate_limiter.increment_order().await;
        }

        Ok(crate::binance::options::RestResponse {
            data,
            request_duration: start.elapsed(),
            headers,
        })
    }
}

impl BinanceRestClient for RestClient {
    type Error = Errors;
    type RestResponse<T> = crate::binance::options::RestResponse<T>;

    fn api_secret(&self) -> &dyn ExposableSecret {
        self.api_secret.as_ref()
    }

    async fn send_request<T>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        query_string: Option<&str>,
        body: Option<&str>,
        weight: u32,
        is_order: bool,
    ) -> Result<Self::RestResponse<T>, Self::Error>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        RestClient::send_request(self, endpoint, method, query_string, body, weight, is_order).await
    }

    fn extract_data<T>(response: Self::RestResponse<T>) -> T {
        response.data
    }

    fn from_serialize(e: serde_urlencoded::ser::Error) -> Self::Error {
        Errors::Error(format!("Serialization error: {e}"))
    }

    fn from_signature(e: String) -> Self::Error {
        Errors::Error(format!("Signature error: {e}"))
    }
}
