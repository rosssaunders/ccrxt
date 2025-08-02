use std::time::Instant;

use crate::binance::{
    coinm::{Errors, RestResponse, RestResult},
    shared::{Errors as SharedErrors, client::PublicBinanceClient},
};

pub struct CoinmPublicRestClient(PublicBinanceClient);

pub type RestClient = CoinmPublicRestClient;

impl From<PublicBinanceClient> for CoinmPublicRestClient {
    fn from(client: PublicBinanceClient) -> Self {
        CoinmPublicRestClient(client)
    }
}

impl CoinmPublicRestClient {
    /// Create a new CoinM public REST client
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

    /// Helper method to match the existing CoinM public endpoint interface
    pub async fn send_request<Req, Resp>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: Option<Req>,
        weight: u32,
    ) -> RestResult<Resp>
    where
        Req: serde::ser::Serialize,
        Resp: serde::de::DeserializeOwned + Send + 'static,
    {
        let start = Instant::now();

        // Call the shared client's send_public_request
        let shared_response = PublicBinanceClient::send_public_request::<Resp, Req, SharedErrors>(
            &self.0, endpoint, method, params, weight,
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

        // Convert shared RestResponse to coinm RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            request_duration: start.elapsed(),
            headers: crate::binance::coinm::ResponseHeaders {
                values: std::collections::HashMap::new(), // TODO: Convert headers properly
            },
        })
    }
}
