// Implements the Binance USDM public REST API endpoint: time.
//
// - GET /fapi/v1/time
//
// See: https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md

use super::RestClient;
use crate::binance::usdm::RestResult;

use serde::Deserialize;

/// Response from the Binance USDM server time endpoint.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ServerTimeResponse {
    /// The current server time in milliseconds since epoch.
    #[serde(rename = "serverTime")]
    pub server_time: u64,
}

impl RestClient {
    /// Get the current server time.
    ///
    /// Endpoint: GET /fapi/v1/time
    /// [Binance API Docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Check-Server-Time)
    pub async fn get_server_time(&self) -> RestResult<ServerTimeResponse> {
        self.send_request("/fapi/v1/time", reqwest::Method::GET, None, None, 1)
            .await
    }
}
