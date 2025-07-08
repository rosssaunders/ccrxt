use serde::Deserialize;

use super::client::RestClient;
use crate::binance::options::RestResult;

/// Ping response (empty object)
#[derive(Debug, Clone, Deserialize)]
pub struct PingResponse {}

impl RestClient {
    /// Test connectivity
    ///
    /// Test connectivity to the Rest API.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/derivatives/option/market-data/Test-Connectivity)
    /// Method: GET /eapi/v1/ping
    /// Weight: 1
    /// Security: None
    pub async fn ping(&self) -> RestResult<PingResponse> {
        self.send_request("/eapi/v1/ping", reqwest::Method::GET, None, None, 1)
            .await
    }
}
