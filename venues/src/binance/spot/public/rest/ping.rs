use serde::Deserialize;

use super::client::RestClient;
use crate::binance::spot::RestResult;

const PING_ENDPOINT: &str = "/api/v3/ping";

/// Ping response (empty object)
#[derive(Debug, Clone, Deserialize)]
pub struct PingResponse {}

impl RestClient {
    /// Test connectivity
    ///
    /// Test connectivity to the Rest API.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#test-connectivity)
    /// Method: GET /api/v3/ping
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
