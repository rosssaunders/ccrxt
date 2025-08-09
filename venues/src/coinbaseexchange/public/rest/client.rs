//! REST client for Coinbase Exchange public endpoints.
//!
//! Provides access to all public REST API endpoints for Coinbase Exchange.
//! Public endpoints do not require authentication and provide market data.

use std::borrow::Cow;

use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};

use crate::coinbaseexchange::{EndpointType, Errors, RateLimiter, RestResult};

/// Public REST client for Coinbase Exchange
///
/// This client handles all public API endpoints that provide market data.
/// It provides automatic rate limiting and error handling but does not require authentication.
pub struct RestClient {
    /// The base URL for the Coinbase Exchange REST API
    pub base_url: Cow<'static, str>,

    /// The underlying HTTP client used for making requests
    pub client: Client,

    /// The rate limiter used to manage request rates and prevent hitting API limits
    pub rate_limiter: RateLimiter,
}

impl RestClient {
    /// Create a new REST client for Coinbase Exchange public endpoints
    ///
    /// # Arguments
    /// * `base_url` - The base URL for the Coinbase Exchange API
    /// * `client` - HTTP client for making requests
    /// * `rate_limiter` - Rate limiter for managing request frequency
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

    /// Send a GET request to a public endpoint (optimized for HFT)
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `params` - Optional query parameters
    ///
    /// # Returns
    /// A result containing the deserialized response or an error
    pub async fn send_get_request<T, P>(
        &self,
        endpoint: &str,
        params: Option<&P>,
    ) -> RestResult<T>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        // Check rate limit before making request
        self.rate_limiter.check_limit(EndpointType::Public).await?;

        // Build URL and request
        let url = format!("{}/{}", self.base_url, endpoint);
        let mut request_builder = self.client.get(&url);

        // Handle query parameters for GET requests
        if let Some(params) = params {
            let query_string = serde_urlencoded::to_string(params).map_err(|e| {
                Errors::Error(format!("Failed to serialize query parameters: {e}"))
            })?;
            if !query_string.is_empty() {
                // Parse the query string and add individual parameters
                let parsed_params: Vec<(String, String)> =
                    serde_urlencoded::from_str(&query_string).map_err(|e| {
                        Errors::Error(format!("Failed to parse query parameters: {e}"))
                    })?;
                for (key, value) in &parsed_params {
                    request_builder = request_builder.query(&[(key, value)]);
                }
            }
        }

        // Add headers
        request_builder = request_builder.header("User-Agent", "ccrxt/0.1.0");

        // Send request
        let response = request_builder.send().await?;

        // Check response status
        let status = response.status();
        let response_text = response.text().await?;

        if status.is_success() {
            // Parse successful response
            let data = serde_json::from_str::<T>(&response_text)
                .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;
            Ok(data)
        } else {
            // Parse error response
            if let Ok(error_response) =
                serde_json::from_str::<crate::coinbaseexchange::ErrorResponse>(&response_text)
            {
                match status.as_u16() {
                    400 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::BadRequest {
                            msg: error_response.message,
                        },
                    )),
                    401 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::Unauthorized {
                            msg: error_response.message,
                        },
                    )),
                    403 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::Forbidden {
                            msg: error_response.message,
                        },
                    )),
                    404 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::NotFound {
                            msg: error_response.message,
                        },
                    )),
                    429 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::TooManyRequests {
                            msg: error_response.message,
                        },
                    )),
                    500 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::InternalServerError {
                            msg: error_response.message,
                        },
                    )),
                    _ => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::UnknownApiError {
                            code: Some(status.as_u16() as i32),
                            msg: error_response.message,
                        },
                    )),
                }
            } else {
                Err(Errors::Error(format!("HTTP {status}: {response_text}")))
            }
        }
    }

    /// Send a GET request to a public endpoint with headers (optimized for HFT)
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path (without leading slash)
    /// * `params` - Optional query parameters
    ///
    /// # Returns
    /// A result containing the deserialized response and headers or an error
    pub async fn send_get_request_with_headers<T, P>(
        &self,
        endpoint: &str,
        params: Option<&P>,
    ) -> RestResult<(T, reqwest::header::HeaderMap)>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        // Check rate limit before making request
        self.rate_limiter.check_limit(EndpointType::Public).await?;

        // Build URL and request
        let url = format!("{}/{}", self.base_url, endpoint);
        let mut request_builder = self.client.get(&url);

        // Handle query parameters for GET requests
        if let Some(params) = params {
            let query_string = serde_urlencoded::to_string(params).map_err(|e| {
                Errors::Error(format!("Failed to serialize query parameters: {e}"))
            })?;
            if !query_string.is_empty() {
                // Parse the query string and add individual parameters
                let parsed_params: Vec<(String, String)> =
                    serde_urlencoded::from_str(&query_string).map_err(|e| {
                        Errors::Error(format!("Failed to parse query parameters: {e}"))
                    })?;
                for (key, value) in &parsed_params {
                    request_builder = request_builder.query(&[(key, value)]);
                }
            }
        }

        // Add headers
        request_builder = request_builder.header("User-Agent", "ccrxt/0.1.0");

        // Send request
        let response = request_builder.send().await?;

        // Check response status and capture headers
        let status = response.status();
        let headers = response.headers().clone();
        let response_text = response.text().await?;

        if status.is_success() {
            // Parse successful response
            let data = serde_json::from_str::<T>(&response_text)
                .map_err(|e| Errors::Error(format!("Failed to parse response: {e}")))?;
            Ok((data, headers))
        } else {
            // Parse error response
            if let Ok(error_response) =
                serde_json::from_str::<crate::coinbaseexchange::ErrorResponse>(&response_text)
            {
                match status.as_u16() {
                    400 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::BadRequest {
                            msg: error_response.message,
                        },
                    )),
                    401 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::Unauthorized {
                            msg: error_response.message,
                        },
                    )),
                    403 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::Forbidden {
                            msg: error_response.message,
                        },
                    )),
                    404 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::NotFound {
                            msg: error_response.message,
                        },
                    )),
                    429 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::TooManyRequests {
                            msg: error_response.message,
                        },
                    )),
                    500 => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::InternalServerError {
                            msg: error_response.message,
                        },
                    )),
                    _ => Err(Errors::ApiError(
                        crate::coinbaseexchange::ApiError::UnknownApiError {
                            code: Some(status.as_u16() as i32),
                            msg: error_response.message,
                        },
                    )),
                }
            } else {
                Err(Errors::Error(format!("HTTP {status}: {response_text}")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_client_creation() {
        let client = Client::new();
        let rate_limiter = RateLimiter::new();

        let rest_client =
            RestClient::new("https://api.exchange.coinbase.com", client, rate_limiter);

        assert_eq!(rest_client.base_url, "https://api.exchange.coinbase.com");
    }
}