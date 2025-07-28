use crate::binance::{
    spot::{Errors, RestResponse, RestResult},
    shared::{client::PrivateBinanceClient, Errors as SharedErrors},
};
use serde::Serialize;

pub struct SpotPrivateRestClient(PrivateBinanceClient);

pub type RestClient = SpotPrivateRestClient;


impl From<PrivateBinanceClient> for SpotPrivateRestClient {
    fn from(client: PrivateBinanceClient) -> Self {
        SpotPrivateRestClient(client)
    }
}

impl SpotPrivateRestClient {
    /// Send a signed request with spot-specific response type
    pub async fn send_signed_request<T, R>(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: R,
        weight: u32,
        is_order: bool,
    ) -> RestResult<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
        R: Serialize,
    {
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
        
        // Convert shared RestResponse to spot RestResponse
        Ok(RestResponse {
            data: shared_response.data,
            headers: crate::binance::spot::ResponseHeaders {
                values: std::collections::HashMap::new(), // TODO: Convert headers properly
            },
        })
    }
}
