use std::{borrow::Cow, collections::HashMap, sync::Arc, time::Duration};

use rest::{HttpClient, HttpRequest, Method as HttpMethod};
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

/// Convert reqwest::Method to HttpMethod
fn convert_method(method: &reqwest::Method) -> HttpMethod {
    match *method {
        reqwest::Method::GET => HttpMethod::Get,
        reqwest::Method::POST => HttpMethod::Post,
        reqwest::Method::PUT => HttpMethod::Put,
        reqwest::Method::DELETE => HttpMethod::Delete,
        reqwest::Method::PATCH => HttpMethod::Patch,
        _ => HttpMethod::Get, // Default fallback
    }
}

/// Generic Binance REST client for all venues (signed/private) using HttpClient trait
pub struct PrivateBinanceHttpClient {
    base_url: Cow<'static, str>,
    http_client: Arc<dyn HttpClient>,
    rate_limiter: RateLimiter,
    api_key: Box<dyn ExposableSecret>,
    api_secret: Box<dyn ExposableSecret>,
}

impl PrivateBinanceHttpClient {
    /// Create a new private Binance client with a custom HTTP client
    pub fn new(
        base_url: Cow<'static, str>,
        http_client: Arc<dyn HttpClient>,
        rate_limiter: RateLimiter,
        api_key: Box<dyn ExposableSecret>,
        api_secret: Box<dyn ExposableSecret>,
    ) -> Self {
        Self {
            base_url,
            http_client,
            rate_limiter,
            api_key,
            api_secret,
        }
    }

    /// Send a signed (authenticated) request, mapping errors to venue-specific error type
    pub async fn send_signed_request<T, R, E>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
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

        let http_method = convert_method(&method);
        let result = if method == reqwest::Method::GET {
            self.send_request_internal(
                endpoint,
                http_method,
                Some(&signed_params),
                None,
                weight,
                is_order,
            )
            .await
        } else {
            self.send_request_internal(
                endpoint,
                http_method,
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
        method: HttpMethod,
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

        // Send request with retry logic for rate limiting
        let mut attempts: u32 = 0;
        const MAX_ATTEMPTS: u32 = 3;

        loop {
            attempts = attempts.saturating_add(1u32);

            // Build the request
            let mut request = HttpRequest::new(method, url.clone());
            
            // Add API key header
            request = request.header("X-MBX-APIKEY", self.api_key.expose_secret());

            // Add body for non-GET requests
            if let Some(body_content) = body {
                request = request
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(body_content.as_bytes().to_vec());
            }

            let response = self.http_client.execute(request).await
                .map_err(|e| Errors::Error(format!("HTTP request failed: {e}")))?;

            let status = response.status;
            let headers = response.headers.clone();
            let response_text = response.text()
                .map_err(|e| Errors::Error(format!("Failed to read response text: {e}")))?;

            // Handle HTTP status codes
            if status == 429 && attempts < MAX_ATTEMPTS {
                // Extract retry-after from headers if available
                let retry_after = headers
                    .get("retry-after")
                    .and_then(|s| s.parse::<u64>().ok())
                    .map(Duration::from_secs)
                    .unwrap_or(Duration::from_secs(1));

                sleep(retry_after).await;
                continue;
            }

            // Convert status to reqwest::StatusCode for compatibility
            let status_code = reqwest::StatusCode::from_u16(status)
                .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR);
            handle_http_status(status_code, &response_text)?;

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

/// Public Binance REST client for unauthenticated endpoints using HttpClient trait
pub struct PublicBinanceHttpClient {
    base_url: Cow<'static, str>,
    http_client: Arc<dyn HttpClient>,
    rate_limiter: RateLimiter,
}

impl PublicBinanceHttpClient {
    /// Create a new public client with a custom HTTP client
    pub fn new(
        base_url: Cow<'static, str>, 
        http_client: Arc<dyn HttpClient>, 
        rate_limiter: RateLimiter
    ) -> Self {
        Self {
            base_url,
            http_client,
            rate_limiter,
        }
    }

    /// Send a public (unsigned) request, mapping errors to venue-specific error type
    pub async fn send_public_request<T, R, E>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        let query_string = if let Some(p) = params {
            Some(serde_urlencoded::to_string(&p).map_err(Errors::from)?)
        } else {
            None
        };

        let http_method = convert_method(&method);
        self.send_request_internal(endpoint, http_method, query_string.as_deref(), weight)
            .await
            .map_err(E::from)
    }

    /// Internal method to send public HTTP requests
    async fn send_request_internal<T>(
        &self,
        endpoint: &str,
        method: HttpMethod,
        query_string: Option<&str>,
        weight: u32,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
    {
        // Check rate limits before making request
        self.rate_limiter.check_limits(weight, false).await?;

        let url = if let Some(query) = query_string {
            format!("{}{endpoint}?{query}", self.base_url)
        } else {
            format!("{}{endpoint}", self.base_url)
        };

        // Build and send request
        let request = HttpRequest::new(method, url);
        let response = self.http_client.execute(request).await
            .map_err(|e| Errors::Error(format!("HTTP request failed: {e}")))?;

        let status = response.status;
        let headers = response.headers.clone();
        let response_text = response.text()
            .map_err(|e| Errors::Error(format!("Failed to read response text: {e}")))?;

        // Convert status to reqwest::StatusCode for compatibility
        let status_code = reqwest::StatusCode::from_u16(status)
            .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR);
        handle_http_status(status_code, &response_text)?;

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
        self.rate_limiter.record_usage(weight, false).await;

        // Update rate limiter from response headers
        self.rate_limiter.update_from_headers(&headers).await;

        // Extract rate limit info
        let rate_limit_info = self.extract_rate_limit_info(&headers);

        Ok(RestResponse {
            data,
            headers: ResponseHeaders { headers },
            rate_limit_info,
        })
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