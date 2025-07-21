use crate::binance::{
    coinm::RestResult,
    shared::client::PublicBinanceClient,
};

pub type RestClient = PublicBinanceClient;

impl RestClient {
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
        self.send_public_request(endpoint, method, params, weight)
            .await
    }
}
