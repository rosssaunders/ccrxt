use serde::{Deserialize, Serialize};

use crate::binance::usdm::PublicRestClient as RestClient;
use crate::binance::usdm::RestResult;

const SERVER_TIME_ENDPOINT: &str = "/fapi/v1/time";

/// Request parameters for the server time endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ServerTimeRequest {
    // This endpoint has no parameters, but we need a struct per project rules
}

/// Response from the Binance USDM server time endpoint.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ServerTimeResponse {
    /// The current server time in milliseconds since epoch.
    #[serde(rename = "serverTime")]
    pub server_time: u64,
}

impl RestClient {
    /// Check Server Time
    ///
    /// Test connectivity to the Rest API and get the current server time.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Check-Server-Time)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `request` - The server time request parameters (empty for this endpoint)
    ///
    /// # Returns
    /// A result containing the server time response.
    pub async fn server_time(&self, request: ServerTimeRequest) -> RestResult<ServerTimeResponse> {
        self.send_get_request(SERVER_TIME_ENDPOINT, Some(request), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_time_request_default() {
        let request = ServerTimeRequest::default();
        let serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(serialized, "{}");
    }

    #[test]
    fn test_server_time_response_deserialization() {
        let json = r#"{
            "serverTime": 1625184000000
        }"#;

        let response: ServerTimeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.server_time, 1625184000000);
    }

    #[test]
    fn test_server_time_response_large_value() {
        let json = r#"{
            "serverTime": 9999999999999
        }"#;

        let response: ServerTimeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.server_time, 9999999999999);
    }

    #[test]
    fn test_server_time_response_with_extra_fields() {
        // Test that extra fields are ignored
        let json = r#"{
            "serverTime": 1625184000000,
            "timezone": "UTC",
            "extra": "field"
        }"#;

        let response: ServerTimeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.server_time, 1625184000000);
    }

    #[test]
    fn test_server_time_response_equality() {
        let response1 = ServerTimeResponse {
            server_time: 1625184000000,
        };
        let response2 = ServerTimeResponse {
            server_time: 1625184000000,
        };
        assert_eq!(response1, response2);
    }
}
