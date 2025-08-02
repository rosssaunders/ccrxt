// REST client for Binance USD-M public endpoints.
//
// Provides access to all public REST API endpoints for Binance USD-M Futures.
// All requests are unauthenticated and do not require API credentials.
use crate::binance::{
    usdm::{Errors, rest::common::RestResponse},
    shared::{client::PublicBinanceClient, Errors as SharedErrors},
};
use rest::secrets::ExposableSecret;
use std::time::Instant;

pub struct UsdmPublicRestClient(PublicBinanceClient);

pub type RestClient = UsdmPublicRestClient;


impl From<PublicBinanceClient> for UsdmPublicRestClient {
    fn from(client: PublicBinanceClient) -> Self {
        UsdmPublicRestClient(client)
    }
}

impl UsdmPublicRestClient {
    /// Create a new USDM public REST client
    pub fn new(base_url: impl Into<std::borrow::Cow<'static, str>>, client: reqwest::Client, rate_limiter: crate::binance::shared::RateLimiter) -> Self {
        Self(PublicBinanceClient::new(base_url.into(), client, rate_limiter))
    }
    
    /// Send a public request with usdm-specific response type
    pub async fn send_public_request<T, R>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: serde::Serialize,
    {
        let start = Instant::now();
        
        // Call the shared client's send_public_request
        let shared_response = PublicBinanceClient::send_public_request::<T, R, SharedErrors>(
            &self.0,
            endpoint,
            method,
            params,
            weight
        )
        .await
        .map_err(|e| match e {
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => {
                Errors::Error(format!("Rate limit exceeded, retry after {:?}", retry_after))
            },
            SharedErrors::InvalidApiKey() => Errors::InvalidApiKey(),
            SharedErrors::HttpError(err) => Errors::HttpError(err),
            SharedErrors::SerializationError(msg) => Errors::Error(format!("Serialization error: {}", msg)),
            SharedErrors::Error(msg) => Errors::Error(msg),
        })?;
        
        // Convert shared RestResponse to usdm RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            headers: crate::binance::usdm::ResponseHeaders::from_shared(shared_response.headers),
            request_duration: start.elapsed(),
        })
    }

    /// Send a request with API key only (no signature) with usdm-specific response type
    pub async fn send_api_key_request<T, R>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        api_key: &dyn ExposableSecret,
        params: Option<R>,
        weight: u32,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: serde::Serialize,
    {
        let start = Instant::now();
        
        // Call the shared client's send_api_key_request
        let shared_response = self.0.send_api_key_request::<T, R, SharedErrors>(
            endpoint,
            method,
            api_key,
            params,
            weight
        )
        .await
        .map_err(|e| match e {
            SharedErrors::ApiError(_) => Errors::Error("API error occurred".to_string()),
            SharedErrors::RateLimitExceeded { retry_after } => {
                Errors::Error(format!("Rate limit exceeded, retry after {:?}", retry_after))
            },
            SharedErrors::InvalidApiKey() => Errors::InvalidApiKey(),
            SharedErrors::HttpError(err) => Errors::HttpError(err),
            SharedErrors::SerializationError(msg) => Errors::Error(format!("Serialization error: {}", msg)),
            SharedErrors::Error(msg) => Errors::Error(msg),
        })?;
        
        // Convert shared RestResponse to usdm RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            headers: crate::binance::usdm::ResponseHeaders::from_shared(shared_response.headers),
            request_duration: start.elapsed(),
        })
    }
}
