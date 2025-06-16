use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Time data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeData {
    /// System time, Unix timestamp format in milliseconds
    pub ts: String,
}

/// Response for getting system time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTimeResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Time data
    pub data: Vec<TimeData>,
}

impl RestClient {
    /// Get system time
    ///
    /// Retrieve API server time.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-system-time
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Returns
    /// Response containing the current system time as Unix timestamp in milliseconds
    pub async fn get_time(&self) -> RestResult<GetTimeResponse> {
        self.send_request(
            "api/v5/public/time",
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_time_data_structure() {
        let time_json = json!({
            "ts": "1597026383085"
        });

        let time_data: TimeData = serde_json::from_value(time_json).unwrap();
        assert_eq!(time_data.ts, "1597026383085");
    }

    #[test]
    fn test_get_time_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetTimeResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().ts, "1597026383085");
    }

    #[test]
    fn test_time_data_serialization_roundtrip() {
        let original = TimeData {
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: TimeData = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.ts, deserialized.ts);
    }

    #[test]
    fn test_get_time_response_with_empty_data() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": []
        });

        let response: GetTimeResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 0);
    }

    #[test]
    fn test_get_time_response_error_case() {
        let response_json = json!({
            "code": "50001",
            "msg": "System error",
            "data": []
        });

        let response: GetTimeResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "50001");
        assert_eq!(response.msg, "System error");
        assert_eq!(response.data.len(), 0);
    }

    #[tokio::test]
    async fn test_get_time_method_compilation() {
        // This test ensures the get_time method compiles and is accessible
        // without needing to make an actual HTTP request
        use crate::okx::RateLimiter;
        let client = reqwest::Client::new();
        let rate_limiter = RateLimiter::new();
        let rest_client = super::RestClient::new("https://www.okx.com", client, rate_limiter);

        // Verify the method exists and is properly typed
        let _ = super::RestClient::get_time;
        let _ = &rest_client;

        // This proves the method signature is correct without calling it
        println!("get_time method is accessible and properly typed");
    }
}
