use std::time::Instant;

use crate::binance::{
    spot::{Errors, ResponseHeaders, RestResponse, RestResult},
    shared::{Errors as SharedErrors, client::PublicBinanceClient},
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
        rate_limiter: crate::binance::shared::RateLimiter,
    ) -> Self {
        Self(PublicBinanceClient::new(
            base_url.into(),
            http_client,
            rate_limiter,
        ))
    }

    /// Send GET request - optimized for query parameters
    pub async fn send_get_request<Req, Resp>(
        &self,
        endpoint: &str,
        params: Option<Req>,
        weight: u32,
    ) -> RestResult<Resp>
    where
        Req: serde::ser::Serialize,
        Resp: serde::de::DeserializeOwned + Send + 'static,
    {
        let _start = Instant::now();

        let shared_response = self
            .0
            .send_public_get::<Resp, Req, SharedErrors>(endpoint, params, weight)
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

        Ok(RestResponse {
            data: shared_response.data,
            headers: ResponseHeaders {
                values: std::collections::HashMap::new(),
            },
        })
    }

    /// Send POST request - placeholder for venues with public POST endpoints
    pub async fn send_post_request<Req, Resp>(
        &self,
        endpoint: &str,
        params: Option<Req>,
        weight: u32,
    ) -> RestResult<Resp>
    where
        Req: serde::ser::Serialize,
        Resp: serde::de::DeserializeOwned + Send + 'static,
    {
        let _start = Instant::now();

        let shared_response = self
            .0
            .send_public_post::<Resp, Req, SharedErrors>(endpoint, params, weight)
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

        Ok(RestResponse {
            data: shared_response.data,
            headers: ResponseHeaders {
                values: std::collections::HashMap::new(),
            },
        })
    }

    /// Send PUT request - placeholder for venues with public PUT endpoints
    pub async fn send_put_request<Req, Resp>(
        &self,
        endpoint: &str,
        params: Option<Req>,
        weight: u32,
    ) -> RestResult<Resp>
    where
        Req: serde::ser::Serialize,
        Resp: serde::de::DeserializeOwned + Send + 'static,
    {
        let _start = Instant::now();

        let shared_response = self
            .0
            .send_public_put::<Resp, Req, SharedErrors>(endpoint, params, weight)
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

        Ok(RestResponse {
            data: shared_response.data,
            headers: ResponseHeaders {
                values: std::collections::HashMap::new(),
            },
        })
    }

    /// Send DELETE request - placeholder for venues with public DELETE endpoints
    pub async fn send_delete_request<Req, Resp>(
        &self,
        endpoint: &str,
        params: Option<Req>,
        weight: u32,
    ) -> RestResult<Resp>
    where
        Req: serde::ser::Serialize,
        Resp: serde::de::DeserializeOwned + Send + 'static,
    {
        let _start = Instant::now();

        let shared_response = self
            .0
            .send_public_delete::<Resp, Req, SharedErrors>(endpoint, params, weight)
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

        Ok(RestResponse {
            data: shared_response.data,
            headers: ResponseHeaders {
                values: std::collections::HashMap::new(),
            },
        })
    }

    /// Send PATCH request - placeholder for venues with public PATCH endpoints
    pub async fn send_patch_request<Req, Resp>(
        &self,
        endpoint: &str,
        params: Option<Req>,
        weight: u32,
    ) -> RestResult<Resp>
    where
        Req: serde::ser::Serialize,
        Resp: serde::de::DeserializeOwned + Send + 'static,
    {
        let _start = Instant::now();

        let shared_response = self
            .0
            .send_public_patch::<Resp, Req, SharedErrors>(endpoint, params, weight)
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

        Ok(RestResponse {
            data: shared_response.data,
            headers: ResponseHeaders {
                values: std::collections::HashMap::new(),
            },
        })
    }
}