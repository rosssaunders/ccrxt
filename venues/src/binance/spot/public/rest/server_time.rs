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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_time_response_deserialization() {
        let json = r#"{
            "serverTime": 1625097600000
        }"#;

        let response: ServerTimeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.server_time, 1625097600000);
    }

    #[test]
    fn test_server_time_response_deserialization_recent() {
        let json = r#"{
            "serverTime": 1700000000000
        }"#;

        let response: ServerTimeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.server_time, 1700000000000);
    }
}
