use std::collections::HashMap;

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::bitget::spot::{ApiError, RateLimiter, ResponseHeaders, RestResponse};

/// Public REST client for Bitget spot market
#[derive(Debug, Clone)]
pub struct RestClient {
    pub base_url: String,
    pub client: Client,
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Create a new public REST client
    pub fn new(base_url: impl Into<String>, rate_limiter: RateLimiter, client: Client) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
        }
    }

    /// Make a GET request to the public API
    pub async fn get<T>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<RestResponse<T>, ApiError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);

        let mut request = self.client.get(&url);

        if let Some(params) = params {
            request = request.query(&params);
        }

        let response = request
            .send()
            .await
            .map_err(|e| ApiError::Http(e.to_string()))?;

        let status = response.status();
        let headers = response.headers().clone();

        // Extract rate limit headers
        let mut response_headers = ResponseHeaders::default();
        for (name, value) in headers.iter() {
            if let Some(header_type) =
                crate::bitget::spot::rate_limit::RateLimitHeader::from_str(name.as_str())
            {
                if let Ok(value_str) = value.to_str() {
                    if let Ok(value_u32) = value_str.parse::<u32>() {
                        response_headers.values.insert(header_type, value_u32);
                    }
                }
            }
        }

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .map_err(|e| ApiError::Http(e.to_string()))?;
            return Err(ApiError::Http(format!("HTTP {}: {}", status, error_text)));
        }

        let body = response
            .text()
            .await
            .map_err(|e| ApiError::Http(e.to_string()))?;

        let parsed: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| ApiError::Parse(format!("Failed to parse JSON: {}", e)))?;

        // Check for API error in response
        if let Some(code) = parsed.get("code").and_then(|c| c.as_str()) {
            if code != "00000" {
                let message = parsed
                    .get("msg")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error");
                return Err(ApiError::Api(format!("API Error {}: {}", code, message)));
            }
        }

        let data = parsed
            .get("data")
            .ok_or_else(|| ApiError::Parse("Missing 'data' field".to_string()))?;

        let result: T = serde_json::from_value(data.clone())
            .map_err(|e| ApiError::Parse(format!("Failed to parse data: {}", e)))?;

        Ok(RestResponse {
            data: result,
            headers: response_headers,
        })
    }
}
