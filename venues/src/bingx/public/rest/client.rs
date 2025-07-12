use std::borrow::Cow;

use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};

use crate::bingx::{ApiResponse, EndpointType, Errors, RateLimiter, RestResult};

/// Public REST client for BingX exchange
///
/// This client handles all public API endpoints that do not require authentication.
/// It provides automatic rate limiting and error handling for public market data.
#[derive(Debug, Clone)]
pub struct RestClient {
    /// The base URL for the BingX public REST API (e.g., "https://open-api.bingx.com")
    ///
    /// This is used as the prefix for all endpoint requests.
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests.
    ///
    /// This is reused for connection pooling and performance.
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits.
    ///
    /// This is used to ensure compliance with BingX's rate limits for public endpoints.
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Create a new BingX public REST client
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the BingX API (e.g., "https://open-api.bingx.com")
    /// * `client` - The HTTP client to use for requests
    /// * `rate_limiter` - The rate limiter to use for request throttling
    ///
    /// # Returns
    /// A new RestClient instance
    pub fn new(
        base_url: impl Into<Cow<'static, str>>,
        client: Client,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            rate_limiter,
        }
    }

    /// Send a GET request to a public endpoint
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (e.g., "/openApi/spot/v1/server/time")
    /// * `params` - Optional query parameters
    /// * `endpoint_type` - The type of endpoint for rate limiting
    ///
    /// # Returns
    /// The deserialized response of type T
    pub async fn send_request<T, P>(
        &self,
        endpoint: &str,
        params: Option<&P>,
        endpoint_type: EndpointType,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        // Check rate limits first
        self.rate_limiter
            .check_limits(endpoint_type.clone())
            .await
            .map_err(|e| Errors::RateLimitExceeded(e.to_string()))?;

        // Build the full URL
        let url = format!("{}{}", self.base_url, endpoint);

        // Build the request
        let mut request = self.client.get(&url);

        // Add query parameters if provided
        if let Some(params) = params {
            request = request.query(params);
        }

        // Send the request
        let response = request.send().await?;

        // Record the request for rate limiting
        self.rate_limiter.increment_request(endpoint_type).await;

        // Check if request was successful
        if response.status().is_success() {
            let response_text = response.text().await?;

            // Parse the API response wrapper
            match serde_json::from_str::<ApiResponse<T>>(&response_text) {
                Ok(api_response) => {
                    // Check if the API returned an error
                    if api_response.code != 0 {
                        return Err(Errors::ApiError {
                            code: api_response.code,
                            msg: api_response.msg,
                        });
                    }

                    // Return the unwrapped data
                    match api_response.data {
                        Some(data) => Ok(data),
                        None => Err(Errors::ParseError("Response data is missing".to_string())),
                    }
                }
                Err(e) => Err(Errors::ParseError(e.to_string())),
            }
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Try to parse as BingX error response
            if let Ok(error_response) =
                serde_json::from_str::<crate::bingx::ErrorResponse>(&error_text)
            {
                Err(Errors::from(error_response))
            } else {
                Err(Errors::Error(format!("HTTP {status}: {error_text}")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = RestClient::new(
            "https://open-api.bingx.com",
            Client::new(),
            RateLimiter::new(),
        );

        assert_eq!(client.base_url, "https://open-api.bingx.com");
    }
}
