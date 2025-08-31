// REST client for Binance USD-M public endpoints.
//
// Provides access to all public REST API endpoints for Binance USD-M Futures.
// All requests are unauthenticated and do not require API credentials.

use std::sync::Arc;

use crate::binance::{
    shared::{
        Errors as SharedErrors, RestResponse, client::PublicBinanceClient,
        rate_limiter_trait::BinanceRateLimiter,
    },
    usdm::Errors,
};

pub struct UsdmPublicRestClient(PublicBinanceClient);

pub type RestClient = UsdmPublicRestClient;

impl From<PublicBinanceClient> for UsdmPublicRestClient {
    fn from(client: PublicBinanceClient) -> Self {
        UsdmPublicRestClient(client)
    }
}

impl UsdmPublicRestClient {
    /// Create a new USDM public REST client
    pub fn new(
        base_url: impl Into<std::borrow::Cow<'static, str>>,
        http_client: std::sync::Arc<dyn rest::HttpClient>,
        rate_limiter: Arc<dyn BinanceRateLimiter>,
    ) -> Self {
        Self(PublicBinanceClient::new(
            base_url.into(),
            http_client,
            rate_limiter,
        ))
    }

    /// Send GET request - optimized for query parameters
    pub async fn send_get_request<T, R>(
        &self,
        endpoint: &str,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: serde::Serialize,
    {
        let shared_response = self
            .0
            .send_public_get::<T, R, SharedErrors>(endpoint, params, weight)
            .await
            .map_err(|e| match e {
                SharedErrors::Api(_) => Errors::Error("API error occurred".to_string()),
                SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                    "Rate limit exceeded, retry after {:?}",
                    retry_after
                )),
                SharedErrors::InvalidApiKey => Errors::InvalidApiKey(),
                SharedErrors::Http { message: err } => Errors::HttpError(err),
                SharedErrors::Serialize { message: msg } => {
                    Errors::Error(format!("Serialization error: {}", msg))
                }
                SharedErrors::Deserialize { message: msg } => {
                    Errors::Error(format!("Deserialization error: {}", msg))
                }
                SharedErrors::Generic { message: msg } => Errors::Error(msg),
            })?;

        Ok(shared_response)
    }

    /// Send POST request - placeholder for public POST endpoints if any
    pub async fn send_post_request<T, R>(
        &self,
        endpoint: &str,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: serde::Serialize,
    {
        let shared_response = self
            .0
            .send_public_post::<T, R, SharedErrors>(endpoint, params, weight)
            .await
            .map_err(|e| match e {
                SharedErrors::Api(_) => Errors::Error("API error occurred".to_string()),
                SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                    "Rate limit exceeded, retry after {:?}",
                    retry_after
                )),
                SharedErrors::InvalidApiKey => Errors::InvalidApiKey(),
                SharedErrors::Http { message: err } => Errors::HttpError(err),
                SharedErrors::Serialize { message: msg } => {
                    Errors::Error(format!("Serialization error: {}", msg))
                }
                SharedErrors::Deserialize { message: msg } => {
                    Errors::Error(format!("Deserialization error: {}", msg))
                }
                SharedErrors::Generic { message: msg } => Errors::Error(msg),
            })?;

        Ok(shared_response)
    }

    /// Send DELETE request - placeholder for public DELETE endpoints if any
    pub async fn send_delete_request<T, R>(
        &self,
        endpoint: &str,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: serde::Serialize,
    {
        let shared_response = self
            .0
            .send_public_delete::<T, R, SharedErrors>(endpoint, params, weight)
            .await
            .map_err(|e| match e {
                SharedErrors::Api(_) => Errors::Error("API error occurred".to_string()),
                SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                    "Rate limit exceeded, retry after {:?}",
                    retry_after
                )),
                SharedErrors::InvalidApiKey => Errors::InvalidApiKey(),
                SharedErrors::Http { message: err } => Errors::HttpError(err),
                SharedErrors::Serialize { message: msg } => {
                    Errors::Error(format!("Serialization error: {}", msg))
                }
                SharedErrors::Deserialize { message: msg } => {
                    Errors::Error(format!("Deserialization error: {}", msg))
                }
                SharedErrors::Generic { message: msg } => Errors::Error(msg),
            })?;

        Ok(shared_response)
    }

    /// Send a request with API key only (no signature) - for MARKET_DATA security type endpoints
    pub async fn send_api_key_get_request<T, R>(
        &self,
        endpoint: &str,
        api_key: &dyn rest::secrets::ExposableSecret,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: serde::Serialize,
    {
        // Call the shared client's API-key GET wrapper
        let shared_response = self
            .0
            .send_api_key_get::<T, R, SharedErrors>(endpoint, api_key, params, weight)
            .await
            .map_err(|e| match e {
                SharedErrors::Api(_) => Errors::Error("API error occurred".to_string()),
                SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                    "Rate limit exceeded, retry after {:?}",
                    retry_after
                )),
                SharedErrors::InvalidApiKey => Errors::InvalidApiKey(),
                SharedErrors::Http { message: err } => Errors::HttpError(err),
                SharedErrors::Serialize { message: msg } => {
                    Errors::Error(format!("Serialization error: {}", msg))
                }
                SharedErrors::Deserialize { message: msg } => {
                    Errors::Error(format!("Deserialization error: {}", msg))
                }
                SharedErrors::Generic { message: msg } => Errors::Error(msg),
            })?;

        // Return the shared RestResponse directly
        Ok(shared_response)
    }
}
