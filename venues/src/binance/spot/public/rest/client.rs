use crate::binance::{
    spot::{Errors, RestResponse, RestResult},
    shared::{client::PublicBinanceClient, Errors as SharedErrors},
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
    pub fn new(base_url: impl Into<std::borrow::Cow<'static, str>>, client: reqwest::Client, rate_limiter: crate::binance::shared::RateLimiter) -> Self {
        Self(PublicBinanceClient::new(base_url.into(), client, rate_limiter))
    }

    /// Send a public request with spot-specific response type
    pub async fn send_public_request<T, R>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: Option<R>,
        weight: u32,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: serde::Serialize,
    {
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
        
        // Convert shared RestResponse to spot RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            headers: crate::binance::spot::ResponseHeaders {
                values: std::collections::HashMap::new(), // TODO: Convert headers properly
            },
        })
    }
}
