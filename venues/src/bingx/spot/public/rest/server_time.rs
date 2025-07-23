use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bingx::spot::{EndpointType, RestResult};

const SERVER_TIME_ENDPOINT: &str = "/openApi/spot/v1/server/time";

/// Request for the server time endpoint
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetServerTimeRequest {}

/// Response from the server time endpoint
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServerTimeResponse {
    /// Server time in milliseconds since Unix epoch
    pub server_time: i64,
}

impl RestClient {
    /// Get the server time
    ///
    /// This endpoint returns the current server time in milliseconds.
    /// No authentication required.
    ///
    /// # Returns
    /// The server time response containing the current timestamp
    ///
    /// # Rate Limit
    /// - IP: 100 requests per 10 seconds (Group 1)
    ///
    /// # API Documentation
    /// - Endpoint: GET /openApi/spot/v1/server/time
    /// - No parameters required
    /// - [docs]: https://bingx-api.github.io/docs/#/en-us/spot/base-info.html#Server%20time
    pub async fn get_server_time(&self) -> RestResult<GetServerTimeResponse> {
        self.send_request::<GetServerTimeResponse, GetServerTimeRequest>(
            SERVER_TIME_ENDPOINT,
            None,
            EndpointType::PublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::*;
    use crate::bingx::spot::RateLimiter;

    #[tokio::test]
    async fn test_get_server_time_request_structure() {
        let client = RestClient::new("http://127.0.0.1:0", Client::new(), RateLimiter::new());

        // Test that the method exists and can be called
        // Note: This is a structure test, not an actual API call
        // We expect an error since we're not making real requests to the API
        let result = client.get_server_time().await;
        assert!(result.is_err());
    }

    #[test]
    fn test_server_time_request_serialization() {
        let request = GetServerTimeRequest::default();
        let json = serde_json::to_string(&request).unwrap();
        // Since GetServerTimeRequest is an empty struct, it serializes to an empty object
        assert_eq!(json, "{}");
    }
}
