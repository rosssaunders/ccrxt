use std::collections::HashMap;
use std::time::Duration;

use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use rest::secrets::ExposableSecret;

use super::errors::{ApiError, ErrorResponse, Errors, handle_http_status};
use super::rate_limiter::RateLimiter;
use super::sign_request;
use super::venue_trait::VenueConfig;

/// Response structure containing data, headers, and metadata
#[derive(Debug)]
pub struct RestResponse<T> {
    pub data: T,
    pub headers: ResponseHeaders,
    pub rate_limit_info: Option<RateLimitInfo>,
}

/// Response headers from Binance API
#[derive(Debug, Default)]
pub struct ResponseHeaders {
    pub headers: HashMap<String, String>,
}

/// Rate limit information from response headers
#[derive(Debug)]
pub struct RateLimitInfo {
    pub weight_used: Option<u32>,
    pub order_count: Option<u32>,
    pub retry_after: Option<Duration>,
}

/// Generic Binance REST client for all venues
pub struct BinanceClient<V: VenueConfig> {
    venue: V,
    client: Client,
    rate_limiter: RateLimiter,
    api_key: Option<Box<dyn ExposableSecret>>,
    api_secret: Option<Box<dyn ExposableSecret>>,
}

impl<V: VenueConfig> BinanceClient<V> {
    /// Create a new Binance client for the specified venue
    pub fn new(venue: V) -> Self {
        let rate_limiter = RateLimiter::new(venue.rate_limits());

        Self {
            venue,
            client: Client::new(),
            rate_limiter,
            api_key: None,
            api_secret: None,
        }
    }

    /// Create a new authenticated Binance client
    pub fn new_authenticated(
        venue: V,
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
    ) -> Self {
        let rate_limiter = RateLimiter::new(venue.rate_limits());

        Self {
            venue,
            client: Client::new(),
            rate_limiter,
            api_key: Some(api_key),
            api_secret: Some(api_secret),
        }
    }

    /// Send a public (unsigned) request
    pub async fn send_public_request<T, R>(
        &self,
        endpoint: &str,
        method: Method,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
    {
        let query_string = match params {
            Some(p) => Some(serde_urlencoded::to_string(&p)?),
            None => None,
        };

        self.send_request_internal(
            endpoint,
            method,
            query_string.as_deref(),
            None,
            weight,
            false,
        )
        .await
    }

    /// Send a signed (authenticated) request
    pub async fn send_signed_request<T, R>(
        &self,
        endpoint: &str,
        method: Method,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
    {
        let api_secret = self
            .api_secret
            .as_ref()
            .ok_or_else(|| Errors::Error("API secret required for signed requests".to_string()))?;

        // Add timestamp
        let mut params_with_timestamp = serde_urlencoded::to_string(&params)?;
        if !params_with_timestamp.is_empty() {
            params_with_timestamp.push('&');
        }
        params_with_timestamp.push_str(&format!(
            "timestamp={}",
            chrono::Utc::now().timestamp_millis()
        ));

        // Sign the request
        let signature = sign_request(api_secret.as_ref(), &params_with_timestamp)
            .map_err(|e| Errors::Error(format!("Signing failed: {e}")))?;

        let signed_params = format!("{params_with_timestamp}&signature={signature}");

        if method == Method::GET {
            self.send_request_internal(
                endpoint,
                method,
                Some(&signed_params),
                None,
                weight,
                is_order,
            )
            .await
        } else {
            self.send_request_internal(
                endpoint,
                method,
                None,
                Some(&signed_params),
                weight,
                is_order,
            )
            .await
        }
    }

    /// Internal method to send HTTP requests with rate limiting and error handling
    async fn send_request_internal<T>(
        &self,
        endpoint: &str,
        method: Method,
        query_string: Option<&str>,
        body: Option<&str>,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
    {
        // Check rate limits before making request
        self.rate_limiter.check_limits(weight, is_order).await?;

        let url = if let Some(query) = query_string {
            format!("{}{endpoint}?{query}", self.venue.base_url())
        } else {
            format!("{}{endpoint}", self.venue.base_url())
        };

        let mut request_builder = self.client.request(method, &url);

        // Add API key header if available
        if let Some(api_key) = &self.api_key {
            request_builder = request_builder.header("X-MBX-APIKEY", api_key.expose_secret());
        }

        // Add body for non-GET requests
        if let Some(body_content) = body {
            request_builder = request_builder
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body_content.to_string());
        }

        // Send request with retry logic for rate limiting
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 3;

        loop {
            attempts += 1;

            let response = request_builder
                .try_clone()
                .ok_or_else(|| Errors::Error("Failed to clone request".to_string()))?
                .send()
                .await?;

            // Extract headers
            let mut headers = HashMap::new();
            for (name, value) in response.headers() {
                if let Ok(value_str) = value.to_str() {
                    headers.insert(name.to_string(), value_str.to_string());
                }
            }

            let status = response.status();
            let response_text = response.text().await?;

            // Handle HTTP status codes
            if status == reqwest::StatusCode::TOO_MANY_REQUESTS && attempts < MAX_ATTEMPTS {
                // Extract retry-after from headers if available
                let retry_after = headers
                    .get("retry-after")
                    .and_then(|s| s.parse::<u64>().ok())
                    .map(Duration::from_secs)
                    .unwrap_or(Duration::from_secs(1));

                sleep(retry_after).await;
                continue;
            }

            handle_http_status(status, &response_text)?;

            // Parse response
            if response_text.trim().is_empty() {
                return Err(Errors::Error("Empty response from server".to_string()));
            }

            // Try to parse as error first
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
                let api_error = ApiError::from_code(error_response.code, error_response.msg);
                return Err(Errors::ApiError(api_error));
            }

            // Parse as successful response
            let data: T = serde_json::from_str(&response_text)
                .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;

            // Record successful usage
            self.rate_limiter.record_usage(weight, is_order).await;

            // Update rate limiter from response headers
            self.rate_limiter.update_from_headers(&headers).await;

            // Extract rate limit info
            let rate_limit_info = self.extract_rate_limit_info(&headers);

            return Ok(RestResponse {
                data,
                headers: ResponseHeaders { headers },
                rate_limit_info,
            });
        }
    }

    /// Extract rate limit information from response headers
    fn extract_rate_limit_info(&self, headers: &HashMap<String, String>) -> Option<RateLimitInfo> {
        let mut weight_used = None;
        let mut order_count = None;
        let retry_after = headers
            .get("retry-after")
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs);

        // Look for weight headers
        for (name, value) in headers {
            if name.starts_with("x-mbx-used-weight") {
                weight_used = value.parse().ok();
            } else if name.starts_with("x-mbx-order-count") {
                order_count = value.parse().ok();
            }
        }

        if weight_used.is_some() || order_count.is_some() || retry_after.is_some() {
            Some(RateLimitInfo {
                weight_used,
                order_count,
                retry_after,
            })
        } else {
            None
        }
    }

    /// Get current rate limit usage statistics
    pub async fn get_usage_stats(&self) -> super::rate_limiter::UsageStats {
        self.rate_limiter.get_usage_stats().await
    }

    /// Get the venue configuration
    pub fn venue(&self) -> &V {
        &self.venue
    }

    /// Check if client is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.api_key.is_some() && self.api_secret.is_some()
    }
}

/// Convenience type aliases for each venue
pub mod clients {
    use super::super::venue_trait::configs::*;
    use super::*;

    pub type SpotClient = BinanceClient<SpotConfig>;
    pub type UsdmClient = BinanceClient<UsdmConfig>;
    pub type CoinmClient = BinanceClient<CoinmConfig>;
    pub type OptionsClient = BinanceClient<OptionsConfig>;
    pub type PortfolioClient = BinanceClient<PortfolioConfig>;
}

#[cfg(test)]
mod tests {
    use super::super::venue_trait::configs::{SpotConfig, UsdmConfig};
    use super::*;

    #[tokio::test]
    async fn test_public_request() {
        let client = BinanceClient::new(SpotConfig);

        // This is just a compilation test - we don't want to make actual API calls in tests
        assert!(!client.is_authenticated());
        assert_eq!(client.venue().venue_name(), "spot");
    }

    #[test]
    fn test_venue_configs() {
        let spot = SpotConfig;
        assert_eq!(spot.base_url(), "https://api.binance.com");
        assert!(!spot.supports_futures());
        assert!(spot.supports_margin());

        let usdm = UsdmConfig;
        assert_eq!(usdm.base_url(), "https://fapi.binance.com");
        assert!(usdm.supports_futures());
        assert!(!usdm.supports_margin());
    }
}
