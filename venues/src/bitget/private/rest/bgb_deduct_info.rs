use crate::bitget::{BitgetRestClient, enums::*, error::BitgetError};
use reqwest::Method;
use rest::BitgetRequest;
use serde::{Deserialize, Serialize};

/// Get BGB Deduct Info
///
/// Get the current BGB deduct status for fee optimization.
///
/// Rate limit: 5 req/sec/UID

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBgbDeductInfoRequest {
    // No parameters required
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBgbDeductInfoResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    pub data: BgbDeductInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BgbDeductInfo {
    /// BGB deduct status: "on" / "off"
    pub deduct: String,
}

impl GetBgbDeductInfoRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl BitgetRequest for GetBgbDeductInfoRequest {
    type Response = GetBgbDeductInfoResponse;

    fn path(&self) -> String {
        "/api/v2/spot/account/deduct-info".to_string()
    }

    fn method(&self) -> String {
        "GET".to_string()
    }

    fn need_signature(&self) -> bool {
        true
    }
}

impl BitgetRestClient {
    /// Get BGB Deduct Info
    ///
    /// Get the current BGB deduct status for fee optimization.
    pub async fn get_bgb_deduct_info(
        &self,
        request: GetBgbDeductInfoRequest,
    ) -> Result<GetBgbDeductInfoResponse, BitgetError> {
        self.send_request(&request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bgb_deduct_info_request_serialization() {
        let request = GetBgbDeductInfoRequest::new();

        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized request: {}", serialized);

        assert_eq!(serialized, "{}");
    }

    #[test]
    fn test_get_bgb_deduct_info_response_deserialization() {
        let json = r#"
        {
            "code": "00000",
            "msg": "success",
            "requestTime": 1695808949356,
            "data": {
                "deduct": "on"
            }
        }"#;

        let response: GetBgbDeductInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert_eq!(response.data.deduct, "on");
    }

    #[tokio::test]
    async fn test_get_bgb_deduct_info_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = GetBgbDeductInfoRequest::new();

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_bgb_deduct_info(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
