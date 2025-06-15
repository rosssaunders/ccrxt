// REST client for Deribit public endpoints.
//
// Provides access to all public REST API endpoints for Deribit.
// All requests are unauthenticated and do not require API credentials.

use reqwest::Client;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::sync::Arc;

use crate::deribit::{EndpointType, Errors, RateLimiter, RestResult};

/// Public REST client for Deribit exchange
///
/// This client handles all public API endpoints that don't require authentication.
/// It provides automatic rate limiting and error handling.
#[derive(Debug, Clone)]
pub struct RestClient {
    /// The base URL for the Deribit public REST API (e.g., "https://deribit.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with Deribit's rate limits for public endpoints.
    pub rate_limiter: Arc<RateLimiter>,
}

impl RestClient {
    /// Creates a new Deribit public REST client.
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Deribit public REST API (e.g., "https://deribit.com")
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter for managing API limits
    pub fn new(
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: Arc<RateLimiter>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
        }
    }

    /// Send a request to a public endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "public/get_combo_details")
    /// * `method` - The HTTP method to use
    /// * `params` - Optional struct of query/body parameters (must implement Serialize)
    /// * `endpoint_type` - The endpoint type for rate limiting
    ///
    /// # Returns
    /// A result containing the response data or an error
    pub async fn send_request<T, P>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: serde::Serialize + ?Sized,
    {
        // Check rate limits before making the request
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await?;

        // Build the URL
        let url = if endpoint.starts_with("http") {
            endpoint.to_string()
        } else {
            format!("{}/api/v2/{}", self.base_url, endpoint)
        };

        // Build the request
        let mut request_builder = self.client.request(method.clone(), &url);

        // Add parameters based on method
        if let Some(params) = params {
            let params_value = serde_json::to_value(params)
                .map_err(|e| Errors::Error(format!("Failed to serialize params: {}", e)))?;
            if method == reqwest::Method::GET {
                // For GET requests, add parameters as query string
                if let Some(params_obj) = params_value.as_object() {
                    for (key, value) in params_obj {
                        let value_str = match value {
                            serde_json::Value::String(s) => s.clone(),
                            serde_json::Value::Number(n) => n.to_string(),
                            serde_json::Value::Bool(b) => b.to_string(),
                            _ => value.to_string(),
                        };
                        request_builder = request_builder.query(&[(key, value_str)]);
                    }
                }
            } else {
                // For POST requests, add parameters as JSON body
                request_builder = request_builder.json(&params_value);
            }
        }

        // Add required headers
        request_builder = request_builder.header("Content-Type", "application/json");

        // Send the request
        let response = request_builder.send().await?;

        // Record the request after successful rate limit check
        self.rate_limiter.record_request(endpoint_type).await;

        // Check if the response was successful
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unable to get error text".to_string());
            return Err(Errors::Error(format!("HTTP {} - {}", status, error_text)));
        }

        // Parse the response
        let response_text = response.text().await?;
        
        // Try to parse as error response first
        if let Ok(error_response) = serde_json::from_str::<crate::deribit::ErrorResponse>(&response_text) {
            return Err(Errors::ApiError(error_response.error));
        }

        // Parse as successful response
        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| Errors::Error(format!("Failed to parse response: {}", e)))?;

        Ok(parsed_response)
    }
}