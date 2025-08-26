use serde::Deserialize;

use super::{RestClient, RestResult};

/// Endpoint path for the server time API.
const SERVER_TIME_ENDPOINT: &str = "/dapi/v1/time";

/// Response from the server time endpoint.
///
/// Returned by the `/dapi/v1/time` endpoint. Contains the current server time in milliseconds since epoch.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeResponse {
    /// Server time in milliseconds since epoch.
    /// This value is returned directly from the Binance API.
    pub server_time: u64,
}

impl RestClient {
    /// Check Server time
    ///
    /// Test connectivity to the REST API and get the current server time.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Check-Server-time)
    ///
    /// Rate limit: 1
    ///
    /// # Returns
    /// * `ServerTimeResponse` - Contains the current server time in milliseconds since epoch.
    pub async fn get_server_time(&self) -> RestResult<ServerTimeResponse> {
        self.send_get_request(SERVER_TIME_ENDPOINT, None::<()>, 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_time_response_deserialization() {
        // Example response from Binance API
        let json = r#"{"serverTime": 1625097600000}"#;
        let response: ServerTimeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.server_time, 1625097600000);
    }
}
