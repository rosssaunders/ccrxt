use serde::{Deserialize, Serialize};

use super::super::RestClient;

/// Endpoint for getting BGB deduct information
const BGB_DEDUCT_INFO_ENDPOINT: &str = "/api/v2/spot/account/bgb-deduct-info";

/// Get BGB Deduct Info
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

impl RestClient {
    /// Get BGB Deduct Info
    ///
    /// Get the current BGB deduct status for fee optimization.
    /// [API Documentation](https://www.bitget.com/api-doc/spot/account/GetBgbDeductInfo)
    ///
    /// Rate limit: 5 req/sec/UID
    ///
    /// Returns a `RestResult<GetBgbDeductInfoResponse>` containing the BGB deduct info or an error.
    pub async fn get_bgb_deduct_info(
        &self,
        _request: GetBgbDeductInfoRequest,
    ) -> crate::bitget::RestResult<GetBgbDeductInfoResponse> {
        self.send_signed_request(
            BGB_DEDUCT_INFO_ENDPOINT,
            reqwest::Method::GET,
            None,
            None,
            5,
            false,
            None,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bgb_deduct_info_request_serialization() {
        let request = GetBgbDeductInfoRequest::default();

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
        let _request = GetBgbDeductInfoRequest::default();

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_bgb_deduct_info(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
