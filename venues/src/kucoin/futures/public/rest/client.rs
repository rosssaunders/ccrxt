use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::kucoin::spot::{ApiError, RateLimiter, ResponseHeaders, RestResponse, Result};

/// Public REST client for KuCoin futures market
#[derive(Debug, Clone)]
pub struct RestClient {
    pub base_url: String,
    pub client: Client,
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Create a new public futures REST client
    pub fn new(base_url: impl Into<String>, rate_limiter: RateLimiter, client: Client) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
        }
    }

    /// Create a new public futures REST client with default settings
    pub fn new_default() -> Self {
        Self::new(
            "https://api-futures.kucoin.com",
            RateLimiter::new(),
            Client::new(),
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

        let mut req = self.client.get(&url);

        if let Some(req_data) = request {
            req = req.query(req_data);
        }

        let response = req.send().await?;

        let status = response.status();
        let headers = response.headers().clone();

        let text = response.text().await?;

        if !status.is_success() {
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
