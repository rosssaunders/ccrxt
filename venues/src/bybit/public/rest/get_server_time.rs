use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult};

/// Endpoint URL path for server time
const ENDPOINT_PATH: &str = "/v5/market/time";

#[derive(Debug, Clone, Serialize)]
pub struct GetServerTimeRequest;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTimeData {
    pub time_second: String,
    pub time_nano: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetServerTimeResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: ServerTimeData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get Bybit server time
    ///
    /// Returns the current server time in seconds and nanoseconds.
    ///
    /// # Returns
    /// A result containing the server time response or an error
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
