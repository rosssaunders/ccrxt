//! Server time endpoint for Bullish Exchange API

use serde::Deserialize;

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL path for server time
const SERVER_TIME_ENDPOINT: &str = "/trading-api/v1/time";

/// Server time response
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    /// Server timestamp in milliseconds
    pub timestamp: u64,

    /// Server time in ISO 8601 format
    pub datetime: String,
}

impl RestClient {
    /// Get server time
    ///
    /// Retrieve the current server time. This endpoint can be used to synchronize
    /// client time with the exchange server time.
    ///
    /// [docs]: https://api.exchange.bullish.com/docs/api/rest/trading-api/v2/#get-/v1/time
    ///
    /// # Returns
    /// Current server timestamp and datetime
    pub async fn get_time(&self) -> RestResult<ServerTime> {
        self.send_get_request(SERVER_TIME_ENDPOINT, EndpointType::PublicTime)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_time_deserialization() {
        let json = r#"{
            "timestamp": 1640995200000,
            "datetime": "2022-01-01T00:00:00Z"
        }"#;

        let server_time: ServerTime = serde_json::from_str(json).expect("Deserialization failed");
        assert_eq!(server_time.timestamp, 1640995200000);
        assert_eq!(server_time.datetime, "2022-01-01T00:00:00Z");
    }
}
