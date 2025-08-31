use std::sync::Arc;

use crate::binance::{
    shared::{
        Errors as SharedErrors, client::PublicBinanceClient, rate_limiter_trait::BinanceRateLimiter,
    },
    spot::{Errors, RestResponse, RestResult},
};

pub struct SpotPublicRestClient(PublicBinanceClient);

pub type RestClient = SpotPublicRestClient;

impl From<PublicBinanceClient> for SpotPublicRestClient {
    fn from(client: PublicBinanceClient) -> Self {
        SpotPublicRestClient(client)
    }
}

impl SpotPublicRestClient {
    /// Create a new Spot public REST client
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
        let shared_response = self
            .0
            .send_public_get::<T, R, SharedErrors>(endpoint, params, weight)
            .await
            .map_err(|e| match e {
                SharedErrors::Api(_) => Errors::Generic {
                    message: "API error occurred".to_string(),
                },
                SharedErrors::RateLimitExceeded { retry_after } => Errors::Generic {
                    message: format!("Rate limit exceeded, retry after {:?}", retry_after),
                },
                SharedErrors::InvalidApiKey => Errors::InvalidApiKey,
                SharedErrors::Http { message } => Errors::Http { message },
                SharedErrors::Serialize { message } => Errors::Generic {
                    message: format!("Serialization error: {}", message),
                },
                SharedErrors::Deserialize { message } => Errors::Generic {
                    message: format!("Deserialization error: {}", message),
                },
                SharedErrors::Generic { message } => Errors::Generic { message },
            })?;

        Ok(RestResponse {
            data: shared_response.data,
            headers: crate::binance::spot::ResponseHeaders {
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
        let shared_response = self
            .0
            .send_public_post::<T, R, SharedErrors>(endpoint, params, weight)
            .await
            .map_err(|e| match e {
                SharedErrors::Api(_) => Errors::Generic {
                    message: "API error occurred".to_string(),
                },
                SharedErrors::RateLimitExceeded { retry_after } => Errors::Generic {
                    message: format!("Rate limit exceeded, retry after {:?}", retry_after),
                },
                SharedErrors::InvalidApiKey => Errors::InvalidApiKey,
                SharedErrors::Http { message } => Errors::Http { message },
                SharedErrors::Serialize { message } => Errors::Generic {
                    message: format!("Serialization error: {}", message),
                },
                SharedErrors::Deserialize { message } => Errors::Generic {
                    message: format!("Deserialization error: {}", message),
                },
                SharedErrors::Generic { message } => Errors::Generic { message },
            })?;

        Ok(RestResponse {
            data: shared_response.data,
            headers: crate::binance::spot::ResponseHeaders {
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
        let shared_response = self
            .0
            .send_public_delete::<T, R, SharedErrors>(endpoint, params, weight)
            .await
            .map_err(|e| match e {
                SharedErrors::Api(_) => Errors::Generic {
                    message: "API error occurred".to_string(),
                },
                SharedErrors::RateLimitExceeded { retry_after } => Errors::Generic {
                    message: format!("Rate limit exceeded, retry after {:?}", retry_after),
                },
                SharedErrors::InvalidApiKey => Errors::InvalidApiKey,
                SharedErrors::Http { message } => Errors::Http { message },
                SharedErrors::Serialize { message } => Errors::Generic {
                    message: format!("Serialization error: {}", message),
                },
                SharedErrors::Deserialize { message } => Errors::Generic {
                    message: format!("Deserialization error: {}", message),
                },
                SharedErrors::Generic { message } => Errors::Generic { message },
            })?;

        Ok(RestResponse {
            data: shared_response.data,
            headers: crate::binance::spot::ResponseHeaders {
                values: std::collections::HashMap::new(),
            },
        })
    }
}
