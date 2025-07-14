//! Gate.io public REST API client
#![allow(clippy::redundant_closure)]

use std::sync::Arc;

use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};

use crate::gateio::shared::{RateLimiter, Result};

const LIVE_URL: &str = "https://api.gateio.ws/api/v4";
const TESTNET_URL: &str = "https://api-testnet.gateapi.io/api/v4";

/// Public REST API client for Gate.io
#[derive(Clone)]
pub struct RestClient {
    client: Client,
    base_url: String,
    rate_limiter: Arc<RateLimiter>,
}

impl RestClient {
    /// Create a new public REST client
    pub fn new(testnet: bool) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| crate::gateio::options::GateIoError::Http(e))?;

        Ok(Self {
            client,
            base_url: if testnet { TESTNET_URL } else { LIVE_URL }.to_string(),
            rate_limiter: Arc::new(RateLimiter::new()),
        })
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Make a GET request to the API without query parameters
    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.get_with_query(endpoint, None::<&()>).await
    }

    /// Make a GET request to the API with query parameters
    pub async fn get_with_query<T, Q>(&self, endpoint: &str, query: Option<&Q>) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        // Apply rate limiting
        let _permit = self.rate_limiter.get_permit(endpoint).await.map_err(|_| {
            crate::gateio::options::GateIoError::RateLimitExceeded {
                message: "Rate limit exceeded".to_string(),
            }
        })?;

        let url = format!("{}{}", self.base_url, endpoint);
        let mut request_builder = self.client.get(&url);

        // Add query parameters
        if let Some(params) = query {
            request_builder = request_builder.query(params);
        }

        let response = request_builder
            .send()
            .await
            .map_err(|e| crate::gateio::options::GateIoError::Http(e))?;

        let status = response.status();
        let headers =
            crate::gateio::options::rate_limit::RateLimitHeader::from_headers(response.headers());

        // Update rate limiter with response headers
        if let Some(status) = self.rate_limiter.update_from_headers(&headers, endpoint) {
            tracing::debug!("Rate limit status for {}: {:?}", endpoint, status);
        }

        let body = response
            .text()
            .await
            .map_err(|e| crate::gateio::options::GateIoError::Http(e))?;

        if status.is_success() {
            let data: T = serde_json::from_str(&body)
                .map_err(|e| crate::gateio::options::GateIoError::Json(e))?;
            Ok(data)
        } else {
            let error: crate::gateio::options::errors::ErrorResponse = serde_json::from_str(&body)
                .map_err(|e| crate::gateio::options::GateIoError::Json(e))?;
            Err(crate::gateio::options::GateIoError::Api(
                crate::gateio::options::ApiError {
                    label: error.label,
                    message: error.message,
                    detail: error.detail,
                },
            ))
        }
    }
}
