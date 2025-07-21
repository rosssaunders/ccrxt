use crate::binance::{
    options::RestResult,
    shared::{client::PrivateBinanceClient, send_signed_request},
};
use serde::Serialize;

pub type RestClient = PrivateBinanceClient;

impl RestClient {
    /// Send a signed request using the shared signing logic
    ///
    /// This method provides a simplified interface that matches the new pattern.
    /// It automatically handles request signing and parameter serialization.
    ///
    /// # Arguments
    /// * `endpoint` - The API endpoint path
    /// * `method` - The HTTP method
    /// * `params` - The request parameters (will be serialized)
    /// * `weight` - The request weight
    /// * `is_order` - Whether this is an order endpoint
    ///
    /// # Returns
    /// The full RestResponse with data, headers, and metadata.
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
        send_signed_request(self, endpoint, method, params, weight, is_order).await
    }
}
