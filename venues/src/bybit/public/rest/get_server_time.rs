use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult};

/// Endpoint URL path for server time
const ENDPOINT_PATH: &str = "/v5/market/time";

/// Request parameters for getting server time (no parameters required)
#[derive(Debug, Clone, Serialize)]
pub struct GetServerTimeRequest;

/// Server time data
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeData {
    /// Server time in seconds
    pub time_second: String,

    /// Server time in nanoseconds
    pub time_nano: String,
}

/// Response from the server time endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct GetServerTimeResponse {
    /// Success/Error code (0: success, 1: error)
    #[serde(rename = "retCode")]
    pub ret_code: i32,

    /// Success/Error message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,

    /// Business data result
    pub result: ServerTimeData,

    /// Extended information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,

    /// Current timestamp in milliseconds
    pub time: u64,
}

impl RestClient {
    /// Get Bybit server time
    ///
    /// Returns the current server time in seconds and nanoseconds.
    ///
    /// [docs](https://bybit-exchange.github.io/docs/v5/market/time)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// None - This endpoint does not require any parameters
    ///
    /// # Returns
    /// A result containing the server time response with time in seconds and nanoseconds or an error
    pub async fn get_server_time(&self) -> RestResult<GetServerTimeResponse> {
        self.send_public_request(
            ENDPOINT_PATH,
            None::<&GetServerTimeRequest>,
            EndpointType::Market,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_time_response_deserialization() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "timeSecond": "1688639403",
                "timeNano": "1688639403423213947"
            },
            "retExtInfo": {},
            "time": 1688639403423
        }"#;

        let response: GetServerTimeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.ret_code, 0);
        assert_eq!(response.ret_msg, "OK");
        assert_eq!(response.result.time_second, "1688639403");
        assert_eq!(response.result.time_nano, "1688639403423213947");
        assert_eq!(response.time, 1688639403423);
    }
}
