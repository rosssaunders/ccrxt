use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::public::rest::RestClient;

/// Response from the server time endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    /// Server time in milliseconds since epoch.
    pub server_time: u64,
}

impl RestClient {
    /// Test connectivity to the REST API and get the current server time.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Check-Server-time)
    ///
    /// Weight: 1
    pub async fn get_server_time(&self) -> RestResult<ServerTimeResponse> {
        self.send_request("/dapi/v1/time", reqwest::Method::GET, None::<()>, 1)
            .await
    }
}
