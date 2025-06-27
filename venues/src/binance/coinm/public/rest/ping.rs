use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::public::rest::RestClient;

/// Response from the test connectivity endpoint.
/// This endpoint returns an empty JSON object to test API connectivity.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PingResponse {}

impl RestClient {
    /// Test connectivity to the REST API.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api#test-connectivity)
    ///
    /// Weight: 1
    pub async fn ping(&self) -> RestResult<PingResponse> {
        self.send_request("/dapi/v1/ping", reqwest::Method::GET, None::<()>, 1)
            .await
    }
}
