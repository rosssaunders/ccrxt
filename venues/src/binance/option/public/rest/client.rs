use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use std::time::Instant;

use crate::binance::option::{RateLimiter, RestResult, RestResponse};
use crate::binance::option::request::execute_request;

/// REST client for Binance Options public endpoints
#[derive(Debug, Clone)]
pub struct RestClient {
    http_client: Client,
    base_url: String,
    rate_limiter: Option<RateLimiter>,
}

impl RestClient {
    /// Create a new REST client for Binance Options
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
            base_url: "https://eapi.binance.com".to_string(),
            rate_limiter: None,
        }
    }

    /// Create a new REST client for Binance Options testnet
    pub fn new_testnet() -> Self {
        Self {
            http_client: Client::new(),
            base_url: "https://testnet.binanceops.com".to_string(),
            rate_limiter: None,
        }
    }

    /// Create a new REST client with custom base URL
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            rate_limiter: None,
        }
    }

    /// Send a request to the Binance Options API
    pub(crate) async fn send_request<T>(
        &self,
        endpoint: &str,
        method: Method,
        query_params: Option<&str>,
        _body: Option<&str>,
        _weight: u32,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
    {
        let start = Instant::now();
        
        // Build URL
        let mut url = format!("{}{}", self.base_url, endpoint);
        if let Some(query) = query_params {
            url.push('?');
            url.push_str(query);
        }

        // Build request
        let request_builder = match method {
            Method::GET => self.http_client.get(&url),
            Method::POST => self.http_client.post(&url),
            Method::PUT => self.http_client.put(&url),
            Method::DELETE => self.http_client.delete(&url),
            _ => return Err(crate::binance::option::Errors::Error("Unsupported HTTP method".to_string())),
        };

        // Send request
        let response = request_builder
            .send()
            .await
            .map_err(crate::binance::option::Errors::HttpError)?;

        let request_duration = start.elapsed();

        // Parse response
        let parsed = execute_request(response, request_duration).await?;

        Ok(RestResponse {
            data: parsed.data,
            request_duration: parsed.duration,
            headers: parsed.headers,
        })
    }
}

impl Default for RestClient {
    fn default() -> Self {
        Self::new()
    }
}