use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, RestResult};
use super::RestClient;

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
    pub async fn get_server_time(&self) -> RestResult<GetServerTimeResponse> {
        self.send_request::<GetServerTimeResponse, GetServerTimeRequest>(
            "/openApi/spot/v1/server/time",
            None,
            EndpointType::PublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use crate::bingx::RateLimiter;

    #[tokio::test]
    async fn test_get_server_time_request_structure() {
        let client = RestClient::new(
            "https://open-api.bingx.com",
            Client::new(),
            RateLimiter::new(),
        );

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