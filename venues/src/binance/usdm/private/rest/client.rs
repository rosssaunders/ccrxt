use crate::binance::{
    usdm::{Errors, rest::common::RestResponse},
    shared::{client::PrivateBinanceClient, Errors as SharedErrors},
};
use serde::Serialize;
use std::time::Instant;

pub struct UsdmPrivateRestClient(PrivateBinanceClient);

pub type UsdmClient = UsdmPrivateRestClient;


impl From<PrivateBinanceClient> for UsdmPrivateRestClient {
    fn from(client: PrivateBinanceClient) -> Self {
        UsdmPrivateRestClient(client)
    }
}

impl UsdmPrivateRestClient {
    /// Send a signed request with usdm-specific response type
    pub async fn send_signed_request<T, R>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> Result<RestResponse<T>, Errors>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
        let start = Instant::now();
        
        // Call the shared client's send_signed_request
        let shared_response = PrivateBinanceClient::send_signed_request::<T, R, SharedErrors>(
            &self.0,
            endpoint,
            method,
            params,
            weight,
            is_order
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
