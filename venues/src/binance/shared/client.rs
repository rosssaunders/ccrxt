use std::{borrow::Cow, collections::HashMap, sync::Arc, time::Duration};

use rest::{
    HttpClient,
    http_client::{Method as HttpMethod, RequestBuilder},
    secrets::ExposableSecret,
};
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

/// Generic Binance REST client for all venues (signed/private) using HttpClient trait
pub struct PrivateBinanceClient {
    base_url: Cow<'static, str>,
    http_client: Arc<dyn HttpClient>,
    rate_limiter: RateLimiter,
    api_key: Box<dyn ExposableSecret>,
    api_secret: Box<dyn ExposableSecret>,
}

impl PrivateBinanceClient {
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
            HttpMethod::Get,
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
            HttpMethod::Post,
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
            HttpMethod::Put,
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
            HttpMethod::Delete,
            Some(&signed_params), // DELETE can have query params
            None,
            weight,
            is_order,
        )
        .await
        .map_err(E::from)
    }

    /// Send a request with API key only (no signature) - GET method
    pub async fn send_get_api_key_request<T, R, E>(
        &self,
        endpoint: &str,
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

        self.send_api_key_request_internal(
            endpoint,
            HttpMethod::Get,
            query_string.as_deref(),
            None,
            weight,
        )
        .await
        .map_err(E::from)
    }

    /// Send a request with API key only (no signature) - POST method
    pub async fn send_post_api_key_request<T, R, E>(
        &self,
        endpoint: &str,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        let body = if let Some(p) = params {
            Some(serde_urlencoded::to_string(&p).map_err(Errors::from)?)
        } else {
            None
        };

        self.send_api_key_request_internal(
            endpoint,
            HttpMethod::Post,
            None,
            body.as_deref(),
            weight,
        )
        .await
        .map_err(E::from)
    }

    /// Send a request with API key only (no signature) - DELETE method
    pub async fn send_delete_api_key_request<T, R, E>(
        &self,
        endpoint: &str,
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

        self.send_api_key_request_internal(
            endpoint,
            HttpMethod::Delete,
            query_string.as_deref(),
            None,
            weight,
        )
        .await
        .map_err(E::from)
    }

    /// Internal method to send API-key-only requests (no signature)
    async fn send_api_key_request_internal<T>(
        &self,
        endpoint: &str,
        method: HttpMethod,
        query_string: Option<&str>,
        body: Option<&str>,
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

        // Build the request
        let mut builder = RequestBuilder::new(method, url.clone());

        // Add API key header
        builder = builder.header("X-MBX-APIKEY", self.api_key.expose_secret());

        // Add body for non-GET requests
        if let Some(body_content) = body {
            builder = builder
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body_content.as_bytes().to_vec());
        }

        let request = builder.build();

        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| Errors::Error(format!("HTTP request failed: {e}")))?;

        let status = response.status;
        let headers = response.headers.clone();
        let response_text = response
            .text()
            .map_err(|e| Errors::Error(format!("Failed to read response text: {e}")))?;

        // Handle HTTP status codes
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
            let mut builder = RequestBuilder::new(method, url.clone());

            // Add API key header
            builder = builder.header("X-MBX-APIKEY", self.api_key.expose_secret());

            // Add body for non-GET requests
            if let Some(body_content) = body {
                builder = builder
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(body_content.as_bytes().to_vec());
            }

            let request = builder.build();

            let response = self
                .http_client
                .execute(request)
                .await
                .map_err(|e| Errors::Error(format!("HTTP request failed: {e}")))?;

            let status = response.status;
            let headers = response.headers.clone();
            let response_text = response
                .text()
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

            // Handle HTTP status codes
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

/// Public Binance REST client for unauthenticated endpoints using HttpClient trait
pub struct PublicBinanceClient {
    base_url: Cow<'static, str>,
    http_client: Arc<dyn HttpClient>,
    rate_limiter: RateLimiter,
}

impl PublicBinanceClient {
    /// Create a new public client with a custom HTTP client
    pub fn new(
        base_url: Cow<'static, str>,
        http_client: Arc<dyn HttpClient>,
        rate_limiter: RateLimiter,
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
        method: HttpMethod,
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

        self.send_request_internal(endpoint, method, query_string.as_deref(), weight)
            .await
            .map_err(E::from)
    }

    /// Method-specific GET wrapper to avoid passing HTTP method from call sites
    pub async fn send_public_get<T, R, E>(
        &self,
        endpoint: &str,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        self.send_public_request::<T, R, E>(endpoint, HttpMethod::Get, params, weight)
            .await
    }

    /// Method-specific POST wrapper to avoid passing HTTP method from call sites
    pub async fn send_public_post<T, R, E>(
        &self,
        endpoint: &str,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        self.send_public_request::<T, R, E>(endpoint, HttpMethod::Post, params, weight)
            .await
    }

    /// Method-specific DELETE wrapper to avoid passing HTTP method from call sites
    pub async fn send_public_delete<T, R, E>(
        &self,
        endpoint: &str,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        self.send_public_request::<T, R, E>(endpoint, HttpMethod::Delete, params, weight)
            .await
    }

    /// Method-specific PUT wrapper to avoid passing HTTP method from call sites
    pub async fn send_public_put<T, R, E>(
        &self,
        endpoint: &str,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        self.send_public_request::<T, R, E>(endpoint, HttpMethod::Put, params, weight)
            .await
    }

    /// Method-specific PATCH wrapper to avoid passing HTTP method from call sites
    pub async fn send_public_patch<T, R, E>(
        &self,
        endpoint: &str,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, E>
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
        R: Serialize,
        E: From<Errors>,
    {
        self.send_public_request::<T, R, E>(endpoint, HttpMethod::Patch, params, weight)
            .await
    }

    /// Send a GET request with API key only (no signature) - for MARKET_DATA endpoints
    pub async fn send_api_key_get<T, R, E>(
        &self,
        endpoint: &str,
        api_key: &dyn ExposableSecret,
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

        self.send_api_key_request_internal(
            endpoint,
            HttpMethod::Get,
            api_key,
            query_string.as_deref(),
            None,
            weight,
        )
        .await
        .map_err(E::from)
    }

    /// Internal method to send API-key-only requests (no signature)
    async fn send_api_key_request_internal<T>(
        &self,
        endpoint: &str,
        method: HttpMethod,
        api_key: &dyn ExposableSecret,
        query_string: Option<&str>,
        body: Option<&str>,
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

        // Build the request with API key header
        let mut builder = RequestBuilder::new(method, url.clone());

        // Add API key header for MARKET_DATA endpoints
        builder = builder.header("X-MBX-APIKEY", api_key.expose_secret());

        // Add body for non-GET requests
        if let Some(body_content) = body {
            builder = builder
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body_content.as_bytes().to_vec());
        }

        let request = builder.build();

        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| Errors::Error(format!("HTTP request failed: {e}")))?;

        let status = response.status;
        let headers = response.headers.clone();
        let response_text = response
            .text()
            .map_err(|e| Errors::Error(format!("Failed to read response text: {e}")))?;

        // Handle HTTP status codes
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
        let request = RequestBuilder::new(method, url).build();
        let response = self
            .http_client
            .execute(request)
            .await
            .map_err(|e| Errors::Error(format!("HTTP request failed: {e}")))?;

        let status = response.status;
        let headers = response.headers.clone();
        let response_text = response
            .text()
            .map_err(|e| Errors::Error(format!("Failed to read response text: {e}")))?;

        // Handle HTTP status codes
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
