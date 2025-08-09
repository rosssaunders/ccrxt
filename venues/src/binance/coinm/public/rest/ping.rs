use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Endpoint path for the test connectivity endpoint.
const PING_ENDPOINT: &str = "/dapi/v1/ping";

/// Response from the test connectivity endpoint.
///
/// This struct represents the response from the Binance Coin-Margined Futures REST API `/dapi/v1/ping` endpoint.
/// The endpoint returns an empty JSON object to verify API connectivity.
///
/// [Official API docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api#test-connectivity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResponse {
    // No fields; response is always an empty object.
}

impl RestClient {
    /// Test connectivity
    ///
    /// Sends a GET request to `/dapi/v1/ping` to verify API connectivity.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api#test-connectivity
    ///
    /// Weight: 1
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// `PingResponse` - An empty struct if connectivity is successful.
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

    #[test]
    fn test_ping_response_serialization() {
        let response = PingResponse {};
        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(json, "{}");
    }
}
