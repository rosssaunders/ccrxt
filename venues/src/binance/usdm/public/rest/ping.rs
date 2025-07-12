// Implements the Binance USDM public REST API endpoint: ping.
//
// - GET /fapi/v1/ping
//
// See: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api

use serde::Deserialize;

use super::RestClient;
use crate::binance::usdm::RestResult;

#[derive(Debug, Deserialize)]
pub struct PingResponse {}

impl RestClient {
    /// Test connectivity to the Rest API.
    /// GET /fapi/v1/ping
    pub async fn ping(&self) -> RestResult<PingResponse> {
        self.send_request::<PingResponse>("/fapi/v1/ping", reqwest::Method::GET, None, None, 1)
            .await
    }
}
