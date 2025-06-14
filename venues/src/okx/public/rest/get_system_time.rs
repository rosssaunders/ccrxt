use serde::{Deserialize, Serialize};
use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Response for getting system time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSystemTimeResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// System time data
    pub data: Vec<SystemTime>,
}

/// System time information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTime {
    /// System time in Unix timestamp format (milliseconds)
    pub ts: String,
}

impl RestClient {
    /// Get API server time
    ///
    /// Retrieve API server time.
    ///
    /// See: https://www.okx.com/docs-v5/en/#rest-api-public-data-get-system-time
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Returns
    /// Response containing the current system time
    pub async fn get_system_time(&self) -> RestResult<GetSystemTimeResponse> {
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
    use super::*;
    use serde_json::json;

    #[test]
    fn test_system_time_structure() {
        let time_json = json!({
            "ts": "1597026383085"
        });

        let system_time: SystemTime = serde_json::from_value(time_json).unwrap();
        assert_eq!(system_time.ts, "1597026383085");
    }

    #[test]
    fn test_get_system_time_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ts": "1597026383085"
                }
            ]
        });

        let response: GetSystemTimeResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().ts, "1597026383085");
    }
}