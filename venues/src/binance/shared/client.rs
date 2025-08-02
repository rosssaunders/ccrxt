use std::{borrow::Cow, collections::HashMap, time::Duration};

use reqwest::{Client, Method};
use rest::secrets::ExposableSecret;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use super::{
    errors::{ApiError, ErrorResponse, Errors, handle_http_status},
    rate_limiter::RateLimiter,
    sign_request,
};

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

/// Generic Binance REST client for all venues (signed/private)
pub struct PrivateBinanceClient {
    base_url: Cow<'static, str>,
    client: Client,
    rate_limiter: RateLimiter,
    api_key: Box<dyn ExposableSecret>,
    api_secret: Box<dyn ExposableSecret>,
}

impl PrivateBinanceClient {
    /// Send a signed GET request (high-performance, no HTTP verb branching)
    pub async fn send_get_signed_request<T, R, E>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        // Add timestamp
        let mut params_with_timestamp =
            serde_urlencoded::to_string(&params).map_err(Errors::from)?;
        if !params_with_timestamp.is_empty() {
            params_with_timestamp.push('&');
        }
        params_with_timestamp.push_str(&format!(
            "timestamp={}",
            chrono::Utc::now().timestamp_millis()
        ));

        // Sign the request using the secret trait object
        let signature = sign_request(&*self.api_secret, &params_with_timestamp)
            .map_err(|e| Errors::Error(format!("Signing failed: {e}")))?;

        let signed_params = format!("{params_with_timestamp}&signature={signature}");

        // Optimized for GET requests - no branching
        self.send_request_internal(
            endpoint,
            Method::GET,
            Some(&signed_params),
            None,
            weight,
            is_order,
        )
        .await
        .map_err(E::from)
    }

    /// Send a signed POST request (high-performance, no HTTP verb branching)
    pub async fn send_post_signed_request<T, R, E>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        // Add timestamp
        let mut params_with_timestamp =
            serde_urlencoded::to_string(&params).map_err(Errors::from)?;
        if !params_with_timestamp.is_empty() {
            params_with_timestamp.push('&');
        }
        params_with_timestamp.push_str(&format!(
            "timestamp={}",
            chrono::Utc::now().timestamp_millis()
        ));

        // Sign the request using the secret trait object
        let signature = sign_request(&*self.api_secret, &params_with_timestamp)
            .map_err(|e| Errors::Error(format!("Signing failed: {e}")))?;

        let signed_params = format!("{params_with_timestamp}&signature={signature}");

        // Optimized for POST requests - no branching
        self.send_request_internal(
            endpoint,
            Method::POST,
            None,
            Some(&signed_params),
            weight,
            is_order,
        )
        .await
        .map_err(E::from)
    }

    /// Send a signed PUT request (high-performance, no HTTP verb branching)
    pub async fn send_put_signed_request<T, R, E>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        // Add timestamp
        let mut params_with_timestamp =
            serde_urlencoded::to_string(&params).map_err(Errors::from)?;
        if !params_with_timestamp.is_empty() {
            params_with_timestamp.push('&');
        }
        params_with_timestamp.push_str(&format!(
            "timestamp={}",
            chrono::Utc::now().timestamp_millis()
        ));

        // Sign the request using the secret trait object
        let signature = sign_request(&*self.api_secret, &params_with_timestamp)
            .map_err(|e| Errors::Error(format!("Signing failed: {e}")))?;

        let signed_params = format!("{params_with_timestamp}&signature={signature}");

        // Optimized for PUT requests - no branching
        self.send_request_internal(
            endpoint,
            Method::PUT,
            None,
            Some(&signed_params),
            weight,
            is_order,
        )
        .await
        .map_err(E::from)
    }

    /// Send a signed DELETE request (high-performance, no HTTP verb branching)
    pub async fn send_delete_signed_request<T, R, E>(
        &self,
        endpoint: &str,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        // Add timestamp
        let mut params_with_timestamp =
            serde_urlencoded::to_string(&params).map_err(Errors::from)?;
        if !params_with_timestamp.is_empty() {
            params_with_timestamp.push('&');
        }
        params_with_timestamp.push_str(&format!(
            "timestamp={}",
            chrono::Utc::now().timestamp_millis()
        ));

        // Sign the request using the secret trait object
        let signature = sign_request(&*self.api_secret, &params_with_timestamp)
            .map_err(|e| Errors::Error(format!("Signing failed: {e}")))?;

        let signed_params = format!("{params_with_timestamp}&signature={signature}");

        // Optimized for DELETE requests - no branching
        self.send_request_internal(
            endpoint,
            Method::DELETE,
            Some(&signed_params), // DELETE can have query params
            None,
            weight,
            is_order,
        )
        .await
        .map_err(E::from)
    }

    /// ⚠️ DEPRECATED: Use verb-specific functions instead for better performance
    ///
    /// This function remains for backward compatibility but creates branch prediction penalties.
    /// Use send_get_signed_request, send_post_signed_request, etc. instead.
    #[deprecated(
        note = "Use verb-specific functions (send_get_signed_request, send_post_signed_request, etc.) for better performance"
    )]
    pub async fn send_signed_request<T, R, E>(
        &self,
        endpoint: &str,
        method: Method,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        // Add timestamp
        let mut params_with_timestamp =
            serde_urlencoded::to_string(&params).map_err(Errors::from)?;
        if !params_with_timestamp.is_empty() {
            params_with_timestamp.push('&');
        }
        params_with_timestamp.push_str(&format!(
            "timestamp={}",
            chrono::Utc::now().timestamp_millis()
        ));

        // Sign the request using the secret trait object
        let signature = sign_request(&*self.api_secret, &params_with_timestamp)
            .map_err(|e| Errors::Error(format!("Signing failed: {e}")))?;

        let signed_params = format!("{params_with_timestamp}&signature={signature}");

        let result = if method == Method::GET {
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
        };
        result.map_err(E::from)
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
            format!("{}{endpoint}?{query}", self.base_url)
        } else {
            format!("{}{endpoint}", self.base_url)
        };

        let mut request_builder = self.client.request(method, &url);

        // Add API key header if available
        let api_key = &self.api_key;
        request_builder = request_builder.header("X-MBX-APIKEY", api_key.expose_secret());

        // Add body for non-GET requests
        if let Some(body_content) = body {
            request_builder = request_builder
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body_content.to_string());
        }

        // Send request with retry logic for rate limiting
        let mut attempts: u32 = 0;
        const MAX_ATTEMPTS: u32 = 3;

        loop {
            attempts = attempts.saturating_add(1u32);

            let response = request_builder
                .try_clone()
                .ok_or_else(|| Errors::Error("Failed to clone request".to_string()))?
                .send()
                .await
                .map_err(Errors::from)?;

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
}

/// Public Binance REST client for unauthenticated endpoints
pub struct PublicBinanceClient {
    base_url: Cow<'static, str>,
    client: Client,
    rate_limiter: RateLimiter,
}

impl PublicBinanceClient {
    /// Create a new public client
    pub fn new(base_url: Cow<'static, str>, client: Client, rate_limiter: RateLimiter) -> Self {
        Self {
            base_url,
            client,
            rate_limiter,
        }
    }

    /// Send a public (unsigned) request, mapping errors to venue-specific error type
    pub async fn send_public_request<T, R, E>(
        &self,
        endpoint: &str,
        method: Method,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        // Encode query parameters if provided
        let query_string = match params {
            Some(p) => Some(serde_urlencoded::to_string(&p).map_err(Errors::from)?),
            None => None,
        };

        // Rate limit check
        self.rate_limiter
            .check_limits(weight, false)
            .await
            .map_err(E::from)?;

        // Build URL
        let url = if let Some(query) = query_string {
            format!("{}{}?{}", self.base_url, endpoint, query)
        } else {
            format!("{}{}", self.base_url, endpoint)
        };

        let request_builder = self.client.request(method, &url);

        // Send request with retry logic
        let mut attempts: u32 = 0;
        const MAX_ATTEMPTS: u32 = 3;

        loop {
            attempts = attempts.saturating_add(1u32);
            let response = request_builder
                .try_clone()
                .ok_or_else(|| Errors::Error("Failed to clone request".to_string()))?
                .send()
                .await
                .map_err(Errors::from)?;

            let status = response.status();

            // Extract headers before consuming response
            let mut headers_map = HashMap::new();
            for (k, v) in response.headers() {
                if let Ok(s) = v.to_str() {
                    headers_map.insert(k.to_string(), s.to_string());
                }
            }

            let text = response.text().await.map_err(Errors::from)?;

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS && attempts < MAX_ATTEMPTS {
                let retry_secs = headers_map
                    .get("retry-after")
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(1);
                sleep(Duration::from_secs(retry_secs)).await;
                continue;
            }

            handle_http_status(status, &text)?;

            if text.trim().is_empty() {
                return Err(Errors::Error("Empty response from server".to_string()).into());
            }

            // Try error response
            if let Ok(err_resp) = serde_json::from_str::<ErrorResponse>(&text) {
                let api_error = ApiError::from_code(err_resp.code, err_resp.msg);
                return Err(Errors::ApiError(api_error).into());
            }

            // Parse as success
            let data: T = serde_json::from_str(&text)
                .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;

            // Record and update rate limiter
            self.rate_limiter.record_usage(weight, false).await;
            self.rate_limiter.update_from_headers(&headers_map).await;

            let rate_limit_info = {
                let mut weight_u = None;
                let mut order_c = None;
                let retry = headers_map
                    .get("retry-after")
                    .and_then(|s| s.parse().ok())
                    .map(Duration::from_secs);
                for (name, value) in &headers_map {
                    if name.starts_with("x-mbx-used-weight") {
                        weight_u = value.parse().ok();
                    } else if name.starts_with("x-mbx-order-count") {
                        order_c = value.parse().ok();
                    }
                }
                if weight_u.is_some() || order_c.is_some() || retry.is_some() {
                    Some(RateLimitInfo {
                        weight_used: weight_u,
                        order_count: order_c,
                        retry_after: retry,
                    })
                } else {
                    None
                }
            };

            return Ok(RestResponse {
                data,
                headers: ResponseHeaders {
                    headers: headers_map,
                },
                rate_limit_info,
            });
        }
    }

    /// Send an API-key-only request (MARKET_DATA security type)
    ///
    /// This method is for endpoints that require an API key in the header
    /// but do not require request signing (like historical trades).
    pub async fn send_api_key_request<T, R, E>(
        &self,
        endpoint: &str,
        method: Method,
        api_key: &dyn ExposableSecret,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        // Encode query parameters if provided
        let query_string = match params {
            Some(p) => Some(serde_urlencoded::to_string(&p).map_err(Errors::from)?),
            None => None,
        };

        // Rate limit check
        self.rate_limiter
            .check_limits(weight, false)
            .await
            .map_err(E::from)?;

        // Build URL
        let url = if let Some(query) = query_string {
            format!("{}{}?{}", self.base_url, endpoint, query)
        } else {
            format!("{}{}", self.base_url, endpoint)
        };

        let request_builder = self
            .client
            .request(method, &url)
            .header("X-MBX-APIKEY", api_key.expose_secret()); // Add API key header

        // Send request with retry logic
        let mut attempts: u32 = 0;
        const MAX_ATTEMPTS: u32 = 3;

        loop {
            attempts = attempts.saturating_add(1u32);
            let response = request_builder
                .try_clone()
                .ok_or_else(|| Errors::Error("Failed to clone request".to_string()))?
                .send()
                .await
                .map_err(Errors::from)?;

            let status = response.status();

            // Extract headers before consuming response
            let mut headers_map = HashMap::new();
            for (k, v) in response.headers() {
                if let Ok(s) = v.to_str() {
                    headers_map.insert(k.to_string(), s.to_string());
                }
            }

            let text = response.text().await.map_err(Errors::from)?;

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS && attempts < MAX_ATTEMPTS {
                let retry_secs = headers_map
                    .get("retry-after")
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(1);
                sleep(Duration::from_secs(retry_secs)).await;
                continue;
            }

            handle_http_status(status, &text)?;

            if text.trim().is_empty() {
                return Err(Errors::Error("Empty response from server".to_string()).into());
            }

            // Try error response
            if let Ok(err_resp) = serde_json::from_str::<ErrorResponse>(&text) {
                let api_error = ApiError::from_code(err_resp.code, err_resp.msg);
                return Err(Errors::ApiError(api_error).into());
            }

            // Parse as success
            let data: T = serde_json::from_str(&text)
                .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;

            // Record and update rate limiter
            self.rate_limiter.record_usage(weight, false).await;
            self.rate_limiter.update_from_headers(&headers_map).await;

            let rate_limit_info = {
                let mut weight_u = None;
                let mut order_c = None;
                let retry = headers_map
                    .get("retry-after")
                    .and_then(|s| s.parse().ok())
                    .map(Duration::from_secs);
                for (name, value) in &headers_map {
                    if name.starts_with("x-mbx-used-weight") {
                        weight_u = value.parse().ok();
                    } else if name.starts_with("x-mbx-order-count") {
                        order_c = value.parse().ok();
                    }
                }
                if weight_u.is_some() || order_c.is_some() || retry.is_some() {
                    Some(RateLimitInfo {
                        weight_used: weight_u,
                        order_count: order_c,
                        retry_after: retry,
                    })
                } else {
                    None
                }
            };

            return Ok(RestResponse {
                data,
                headers: ResponseHeaders {
                    headers: headers_map,
                },
                rate_limit_info,
            });
        }
    }
}
