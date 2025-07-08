use serde::Deserialize;

use super::client::RestClient;
use crate::binance::options::RestResult;

/// Response for server time endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct ServerTimeResponse {
    /// Server time in milliseconds
    #[serde(rename = "serverTime")]
    pub server_time: u64,
}

impl RestClient {
    /// Check server time
    ///
    /// Test connectivity to the Rest API and get the current server time.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data)
    /// Method: GET /eapi/v1/time
    /// Weight: 1
    /// Security: None
    pub async fn get_server_time(&self) -> RestResult<ServerTimeResponse> {
        self.send_request("/eapi/v1/time", reqwest::Method::GET, None, None, 1)
            .await
    }
}
