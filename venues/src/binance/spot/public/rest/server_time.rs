use serde::Deserialize;

use super::client::RestClient;
use crate::binance::spot::RestResult;

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
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#check-server-time)
    /// Method: GET /api/v3/time
    /// Weight: 1
    /// Security: None
    pub async fn get_server_time(&self) -> RestResult<ServerTimeResponse> {
        self.send_request("/api/v3/time", reqwest::Method::GET, None, 1)
            .await
    }
}
