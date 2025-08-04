use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

const GET_TIME_ENDPOINT: &str = "api/v5/public/time";

/// Time data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeData {
    /// System time, Unix timestamp format in milliseconds
    pub ts: String,
}

impl RestClient {
    /// Get server time
    ///
    /// Retrieve API server time.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-time
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Returns
    /// Response containing the current system time as Unix timestamp in milliseconds
    pub async fn get_time(&self) -> RestResult<Vec<TimeData>> {
        self.send_get_request(
            GET_TIME_ENDPOINT,
            None::<&()>,
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::OkxApiResponse;
    use serde_json::json;

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

        let response: OkxApiResponse<TimeData> = serde_json::from_value(response_json).unwrap();
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

        let response: OkxApiResponse<TimeData> = serde_json::from_value(response_json).unwrap();
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

        let response: OkxApiResponse<TimeData> = serde_json::from_value(response_json).unwrap();
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
