use std::sync::Arc;

use rest::{HttpClient, http_client::{Method as HttpMethod, RequestBuilder}};
use serde::de::DeserializeOwned;

use crate::kucoin::spot::{ApiError, RateLimiter, ResponseHeaders, RestResponse, Result};

/// Public REST client for KuCoin futures market
#[derive(Clone)]
pub struct RestClient {
    pub base_url: String,
    pub http_client: Arc<dyn HttpClient>,
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Create a new public futures REST client
    pub fn new(base_url: impl Into<String>, rate_limiter: RateLimiter, http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            base_url: base_url.into(),
            http_client,
            rate_limiter,
        }
    }

    /// Create a new public futures REST client with default settings
    pub fn new_default(http_client: Arc<dyn HttpClient>) -> Self {
        Self::new(
            "https://api-futures.kucoin.com",
            RateLimiter::new(),
            http_client,
        )
    }

    /// Make a GET request to the public futures API
    pub async fn send_request<T, R>(
        &self,
        endpoint: &str,
        request: Option<&R>,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
        R: serde::Serialize,
    {
        // Check rate limiter
        if !self.rate_limiter.can_proceed().await {
            return Err(ApiError::RateLimitExceeded {
                code: "429000".to_string(),
                message: "Rate limit exceeded".to_string(),
            }
            .into());
        }

        let url = format!("{}{}", self.base_url, endpoint);

        // Build URL with query parameters
        let full_url = if let Some(req_data) = request {
            let params = serde_urlencoded::to_string(req_data)
                .map_err(|e| ApiError::JsonParsing(format!("Failed to serialize request: {}", e)))?;
            if !params.is_empty() {
                format!("{}?{}", url, params)
            } else {
                url
            }
        } else {
            url
        };

        let request_builder = RequestBuilder::new(HttpMethod::Get, full_url).build();
        let response = self.http_client.execute(request_builder).await
            .map_err(|e| crate::kucoin::spot::KucoinError::NetworkError(format!("HTTP request failed: {}", e)))?;

        let status = response.status;
        let headers = response.headers.clone();

        let text = response.text()
            .map_err(|e| crate::kucoin::spot::KucoinError::NetworkError(format!("Failed to read response: {}", e)))?;

        if !(status >= 200 && status < 300) {
            // Try to parse as error response
            if let Ok(error_response) =
                serde_json::from_str::<crate::kucoin::spot::ErrorResponse>(&text)
            {
                return Err(ApiError::from(error_response).into());
            } else {
                return Err(ApiError::Http(format!("HTTP {}: {}", status, text)).into());
            }
        }

        // Parse successful response
        let response: RestResponse<T> = serde_json::from_str(&text)
            .map_err(|e| ApiError::JsonParsing(format!("Failed to parse response: {}", e)))?;

        // Check if KuCoin indicates success
        if !response.is_success() {
            return Err(ApiError::Other {
                code: response.code.clone(),
                message: "KuCoin API returned non-success code".to_string(),
            }
            .into());
        }

        let rate_limit_headers = ResponseHeaders::from_headers(&headers);

        Ok((response, rate_limit_headers))
    }
}
