use serde::Deserialize;

use crate::binance::options::PublicRestClient as RestClient;
use crate::binance::options::RestResult;

const PING_ENDPOINT: &str = "/eapi/v1/ping";

/// Ping response (empty object)
#[derive(Debug, Clone, Deserialize)]
pub struct PingResponse {}

impl RestClient {
    /// Test connectivity
    ///
    /// Test connectivity to the Rest API.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/option/market-data/Test-Connectivity)
    ///
    /// Method: GET /eapi/v1/ping
    /// Weight: 1
    /// Security: None
    pub async fn ping(&self) -> RestResult<PingResponse> {
        self.send_get_request(PING_ENDPOINT, None::<()>, 1).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_response_deserialization() {
        let json = r#"{}"#;

        let response: PingResponse = serde_json::from_str(json).unwrap();
        // PingResponse is an empty struct, so there's nothing to assert
        // This test just ensures deserialization works correctly
        let _ = response;
    }
}
