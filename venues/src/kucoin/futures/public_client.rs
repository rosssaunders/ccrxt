use std::{collections::HashMap, sync::Arc};

use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
};
use serde::{Serialize, de::DeserializeOwned};

use super::{
    ApiError, KucoinError, ResponseHeaders, RestResponse, Result, rate_limit::RateLimiter,
};

/// Public REST client for KuCoin spot market
#[derive(Clone)]
pub struct RestClient {
    pub base_url: String,
    pub http_client: Arc<dyn HttpClient>,
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Create a new public REST client
    pub fn new(
        base_url: impl Into<String>,
        rate_limiter: RateLimiter,
        http_client: Arc<dyn HttpClient>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            http_client,
            rate_limiter,
        }
    }

    /// Create a new public REST client with default settings
    pub fn new_default(http_client: Arc<dyn HttpClient>) -> Self {
        Self::new(
            "https://api-futures.kucoin.com",
            RateLimiter::new(),
            http_client,
        )
    }

    /// Make a GET request to the public API
    pub async fn get<T>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        T: DeserializeOwned,
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
        let full_url = if let Some(ref params) = params {
            if params.is_empty() {
                url
            } else {
                let query_string = params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&");
                format!("{}?{}", url, query_string)
            }
        } else {
            url
        };

        let request = RequestBuilder::new(HttpMethod::Get, full_url).build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| KucoinError::NetworkError(format!("HTTP request failed: {}", e)))?;

        let status = response.status;
        let headers = response.headers.clone();

        let text = response
            .text()
            .map_err(|e| KucoinError::NetworkError(format!("Failed to read response: {}", e)))?;

        if !(200..300).contains(&status) {
            // Try to parse as error response
            if let Ok(error_response) = serde_json::from_str::<super::ErrorResponse>(&text) {
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

    /// Make a GET request to the public API with a serializable request
    pub async fn get_with_request<P, T>(
        &self,
        endpoint: &str,
        request: &P,
    ) -> Result<(RestResponse<T>, ResponseHeaders)>
    where
        P: Serialize,
        T: DeserializeOwned,
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

        // Serialize the request as query parameters
        let params = serde_urlencoded::to_string(request)
            .map_err(|e| ApiError::JsonParsing(format!("Failed to serialize request: {}", e)))?;

        let full_url = if !params.is_empty() {
            format!("{}?{}", url, params)
        } else {
            url
        };

        let request_builder = RequestBuilder::new(HttpMethod::Get, full_url).build();
        let response = self
            .http_client
            .execute(request_builder)
            .await
            .map_err(|e| KucoinError::NetworkError(format!("HTTP request failed: {}", e)))?;

        let status = response.status;
        let headers = response.headers.clone();

        let text = response
            .text()
            .map_err(|e| KucoinError::NetworkError(format!("Failed to read response: {}", e)))?;

        if !(200..300).contains(&status) {
            // Try to parse as error response
            if let Ok(error_response) = serde_json::from_str::<super::ErrorResponse>(&text) {
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
