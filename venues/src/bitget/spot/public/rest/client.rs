use std::{collections::HashMap, sync::Arc};

use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
};
use serde::de::DeserializeOwned;

use crate::bitget::spot::{ApiError, RateLimiter, ResponseHeaders, RestResponse};

/// Public REST client for Bitget spot market
#[derive(Clone)]
pub struct RestClient {
    pub base_url: String,
    pub http_client: Arc<dyn HttpClient>,
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Create a new public REST client
    pub fn new(base_url: impl Into<String>, rate_limiter: RateLimiter, http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            base_url: base_url.into(),
            http_client,
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
        let mut url = format!("{}{}", self.base_url, endpoint);

        // Add query parameters if provided
        if let Some(params) = params {
            if !params.is_empty() {
                let query_string = params.iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&");
                url.push('?');
                url.push_str(&query_string);
            }
        }

        // Build and execute request
        let request = RequestBuilder::new(HttpMethod::Get, url).build();
        let response = self.http_client.execute(request).await
            .map_err(|e| ApiError::Http(format!("HTTP request failed: {e}")))?;

        let status = response.status;
        
        // Extract rate limit headers
        let mut response_headers = ResponseHeaders::default();
        for (name, value) in response.headers.iter() {
            if let Some(header_type) =
                crate::bitget::spot::rate_limit::RateLimitHeader::from_str(name.as_str())
            {
                if let Ok(value_u32) = value.parse::<u32>() {
                    response_headers.values.insert(header_type, value_u32);
                }
            }
        }

        let body = response.text()
            .map_err(|e| ApiError::Http(format!("Failed to read response: {e}")))?;

        if status != 200 && status != 201 {
            return Err(ApiError::Http(format!("HTTP {}: {}", status, body)));
        }

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
