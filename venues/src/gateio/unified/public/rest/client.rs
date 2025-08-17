//! Gate.io public REST API client
#![allow(clippy::redundant_closure)]

use std::sync::Arc;

use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::gateio::unified::{RestResult, rate_limit::RateLimiter};

const LIVE_URL: &str = "https://api.gateio.ws/api/v4";
const TESTNET_URL: &str = "https://api-testnet.gateapi.io/api/v4";

/// Public REST API client for Gate.io
#[derive(Clone)]
pub struct RestClient {
    http_client: Arc<dyn HttpClient>,
    base_url: String,
    rate_limiter: Arc<RateLimiter>,
}

impl RestClient {
    /// Create a new public REST client
    pub fn new(http_client: Arc<dyn HttpClient>, testnet: bool) -> RestResult<Self> {
        Ok(Self {
            http_client,
            base_url: if testnet { TESTNET_URL } else { LIVE_URL }.to_string(),
            rate_limiter: Arc::new(RateLimiter::new()),
        })
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Make a GET request to the API without query parameters
    pub async fn get<T>(&self, endpoint: &str) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query(endpoint, None::<&()>).await
    }

    /// Make a GET request to the API with query parameters
    pub async fn get_with_query<T, Q>(&self, endpoint: &str, query: Option<&Q>) -> RestResult<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        // Apply rate limiting
        let _permit = self.rate_limiter.get_permit(endpoint).await.map_err(|_| {
            crate::gateio::unified::GateIoError::RateLimitExceeded {
                message: "Rate limit exceeded".to_string(),
            }
        })?;

        let url = format!("{}{}", self.base_url, endpoint);

        // Build URL with query parameters
        let full_url = if let Some(params) = query {
            let query_string = serde_urlencoded::to_string(params).map_err(|e| {
                crate::gateio::unified::GateIoError::InvalidParameter(format!(
                    "Failed to serialize query: {}",
                    e
                ))
            })?;
            if query_string.is_empty() {
                url
            } else {
                format!("{}?{}", url, query_string)
            }
        } else {
            url
        };

        let request = RequestBuilder::new(HttpMethod::Get, full_url).build();
        let response = self.http_client.execute(request).await.map_err(|e| {
            crate::gateio::unified::GateIoError::Network(format!("HTTP request failed: {}", e))
        })?;

        let status = response.status;
        let headers =
            crate::gateio::unified::rate_limit::RateLimitHeader::from_headers(&response.headers);

        // Update rate limiter with response headers
        if let Some(status) = self.rate_limiter.update_from_headers(&headers, endpoint) {
            tracing::debug!("Rate limit status for {}: {:?}", endpoint, status);
        }

        let body = response.text().map_err(|e| {
            crate::gateio::unified::GateIoError::Network(format!("Failed to read response: {}", e))
        })?;

        if (200..300).contains(&status) {
            let data: T = serde_json::from_str(&body)
                .map_err(|e| crate::gateio::unified::GateIoError::Json(e))?;
            Ok(data)
        } else {
            let error: crate::gateio::unified::errors::ErrorResponse = serde_json::from_str(&body)
                .map_err(|e| crate::gateio::unified::GateIoError::Json(e))?;
            Err(crate::gateio::unified::GateIoError::Api(
                crate::gateio::unified::ApiError {
                    label: error.label,
                    message: error.message,
                    detail: error.detail,
                },
            ))
        }
    }
}
