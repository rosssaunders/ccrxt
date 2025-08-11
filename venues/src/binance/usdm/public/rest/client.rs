// REST client for Binance USD-M public endpoints.
//
// Provides access to all public REST API endpoints for Binance USD-M Futures.
// All requests are unauthenticated and do not require API credentials.

use crate::binance::{
    shared::{Errors as SharedErrors, RestResponse, client::PublicBinanceClient},
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
        rate_limiter: crate::binance::shared::RateLimiter,
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
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                "Rate limit exceeded, retry after {:?}",
                retry_after
            )),
            SharedErrors::InvalidApiKey() => Errors::InvalidApiKey(),
            SharedErrors::HttpError(err) => Errors::HttpError(err),
            SharedErrors::SerializationError(msg) => {
                Errors::Error(format!("Serialization error: {}", msg))
            }
            SharedErrors::Error(msg) => Errors::Error(msg),
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
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                "Rate limit exceeded, retry after {:?}",
                retry_after
            )),
            SharedErrors::InvalidApiKey() => Errors::InvalidApiKey(),
            SharedErrors::HttpError(err) => Errors::HttpError(err),
            SharedErrors::SerializationError(msg) => {
                Errors::Error(format!("Serialization error: {}", msg))
            }
            SharedErrors::Error(msg) => Errors::Error(msg),
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
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => Errors::Error(format!(
                "Rate limit exceeded, retry after {:?}",
                retry_after
            )),
            SharedErrors::InvalidApiKey() => Errors::InvalidApiKey(),
            SharedErrors::HttpError(err) => Errors::HttpError(err),
            SharedErrors::SerializationError(msg) => {
                Errors::Error(format!("Serialization error: {}", msg))
            }
            SharedErrors::Error(msg) => Errors::Error(msg),
        })?;

        Ok(shared_response)
    }

    // TODO: Implement API-key-only endpoints
    // Some USDM endpoints require API key but no signature (MARKET_DATA security type)
    // This should be moved to a separate client or use PrivateBinanceClient
    // For now, commenting out to complete WASM migration
    /*
    pub async fn send_api_key_get_request<T, R>(
        &self,
        endpoint: &str,
        api_key: &dyn ExposableSecret,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: serde::Serialize,
    {
        unimplemented!("API-key-only endpoints need to be migrated to use PrivateBinanceClient")
    }
    */
}
