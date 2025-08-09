use crate::binance::{
    shared::{Errors as SharedErrors, client::PublicBinanceClient},
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
        client: reqwest::Client,
        rate_limiter: crate::binance::shared::RateLimiter,
    ) -> Self {
        Self(PublicBinanceClient::new(
            base_url.into(),
            client,
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
        let shared_response = PublicBinanceClient::send_public_request::<T, R, SharedErrors>(
            &self.0,
            endpoint,
            reqwest::Method::GET,
            params,
            weight,
        )
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
        let shared_response = PublicBinanceClient::send_public_request::<T, R, SharedErrors>(
            &self.0,
            endpoint,
            reqwest::Method::POST,
            params,
            weight,
        )
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
        let shared_response = PublicBinanceClient::send_public_request::<T, R, SharedErrors>(
            &self.0,
            endpoint,
            reqwest::Method::DELETE,
            params,
            weight,
        )
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
            headers: crate::binance::spot::ResponseHeaders {
                values: std::collections::HashMap::new(),
            },
        })
    }
}
