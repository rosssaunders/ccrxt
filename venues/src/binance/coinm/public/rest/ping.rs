use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

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

    #[test]
    fn test_ping_response_serialization() {
        let response = PingResponse {};

        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(json, "{}");
    }
}
