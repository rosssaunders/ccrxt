use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Response from the server time endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    /// Server time in milliseconds since epoch.
    pub server_time: u64,
}

impl RestClient {
    /// Test connectivity to the REST API and get the current server time.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Check-Server-time)
    ///
    /// Weight: 1
    pub async fn get_server_time(&self) -> RestResult<ServerTimeResponse> {
        self.send_request("/dapi/v1/time", reqwest::Method::GET, None::<()>, 1)
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
    fn test_server_time_response_serialization() {
        let response = ServerTimeResponse {
            server_time: 1625097600000,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"serverTime\":1625097600000"));
    }
}
