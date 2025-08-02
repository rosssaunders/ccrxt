use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::binance::usdm::RestResult;

// Request path constant
const PING_ENDPOINT: &str = "/fapi/v1/ping";

/// Request parameters for the ping endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct PingRequest {}

/// Response from the Binance USDM ping endpoint.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
pub struct PingResponse {}

impl RestClient {
    /// Test connectivity
    ///
    /// Test connectivity to the Rest API.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/#test-connectivity
    ///
    /// Rate limit: 1 weight
    ///
    /// # Arguments
    /// * `request` - The ping request parameters (empty struct)
    ///
    /// # Returns
    /// Empty PingResponse struct on success
    pub async fn ping(&self, request: PingRequest) -> RestResult<PingResponse> {
        self.send_public_request(PING_ENDPOINT, reqwest::Method::GET, Some(request), 1)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_ping_request_serialization() {
        let request = PingRequest::default();
        let result = serde_json::to_string(&request);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "{}");
    }

    #[test]
    fn test_ping_response_deserialization() {
        let json = r#"{}"#;
        let result: Result<PingResponse, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ping_response_with_extra_fields() {
        let json = r#"{
            "unexpected": "field",
            "another": 123
        }"#;
        let result: Result<PingResponse, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ping_response_default() {
        let response = PingResponse::default();
        assert_eq!(response, PingResponse {});
    }
}
