use std::{sync::Arc, time::Instant};

use crate::binance::{
    options::{Errors, RestResponse, RestResult},
    shared::{
        Errors as SharedErrors, client::PublicBinanceClient, rate_limiter_trait::BinanceRateLimiter,
    },
};

pub struct OptionsPublicRestClient(PublicBinanceClient);

pub type RestClient = OptionsPublicRestClient;

impl From<PublicBinanceClient> for OptionsPublicRestClient {
    fn from(client: PublicBinanceClient) -> Self {
        OptionsPublicRestClient(client)
    }
}

impl OptionsPublicRestClient {
    /// Create a new Options public REST client
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
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: serde::Serialize,
    {
        let start = Instant::now();
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

        Ok(RestResponse {
            data: shared_response.data,
            request_duration: start.elapsed(),
            headers: crate::binance::options::ResponseHeaders {
                values: std::collections::HashMap::new(),
            },
        })
    }

    /// Send POST request - placeholder for public POST endpoints if any
    pub async fn send_post_request<T, R>(
        &self,
        endpoint: &str,
        params: Option<R>,
        weight: u32,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: serde::Serialize,
    {
        let start = Instant::now();
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

        Ok(RestResponse {
            data: shared_response.data,
            request_duration: start.elapsed(),
            headers: crate::binance::options::ResponseHeaders {
                values: std::collections::HashMap::new(),
            },
        })
    }

    /// Send DELETE request - placeholder for public DELETE endpoints if any
    pub async fn send_delete_request<T, R>(
        &self,
        endpoint: &str,
        params: Option<R>,
        weight: u32,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: serde::Serialize,
    {
        let start = Instant::now();
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

        Ok(RestResponse {
            data: shared_response.data,
            request_duration: start.elapsed(),
            headers: crate::binance::options::ResponseHeaders {
                values: std::collections::HashMap::new(),
            },
        })
    }
}
