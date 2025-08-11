use serde::{Deserialize, Serialize};

use super::super::RestClient;

/// Endpoint for getting BGB deduct information
const BGB_DEDUCT_INFO_ENDPOINT: &str = "/api/v2/spot/account/bgb-deduct-info";

/// BGB deduct status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BgbDeductStatus {
    /// BGB deduction is enabled
    On,
    /// BGB deduction is disabled
    Off,
}

/// Request parameters for getting BGB deduct information.
/// This endpoint requires no parameters.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetBgbDeductInfoRequest {
    // No parameters required
}

/// Response from the get BGB deduct info endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBgbDeductInfoResponse {
    /// Response code indicating success or failure.
    pub code: String,

    /// Response message providing additional details.
    pub msg: String,

    /// Timestamp when the request was processed (milliseconds since epoch).
    pub request_time: u64,

    /// BGB deduct information data.
    pub data: BgbDeductInfo,
}

/// BGB deduct information containing the current deduct status.
#[derive(Debug, Clone, Deserialize)]
pub struct BgbDeductInfo {
    /// Current BGB deduct status (on/off).
    pub deduct: BgbDeductStatus,
}

impl RestClient {
    /// Get BGB Deduct Info
    ///
    /// Get the current BGB deduct status for fee optimization.
    /// [docs]: https://www.bitget.com/api-doc/spot/account/Get-BGB-Deduct-Info
    ///
    /// Rate limit: 5 req/sec/UID
    ///
    /// Returns a `RestResult<GetBgbDeductInfoResponse>` containing the BGB deduct info or an error.
    pub async fn get_bgb_deduct_info(
        &self,
        _request: GetBgbDeductInfoRequest,
    ) -> crate::bitget::spot::RestResult<GetBgbDeductInfoResponse> {
        self.send_get_signed_request_no_params(BGB_DEDUCT_INFO_ENDPOINT, 5, false, None)
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
        assert_eq!(response.data.deduct, BgbDeductStatus::On);
    }

    #[test]
    fn test_bgb_deduct_status_serialization() {
        assert_eq!(
            serde_json::to_string(&BgbDeductStatus::On).unwrap(),
            "\"on\""
        );
        assert_eq!(
            serde_json::to_string(&BgbDeductStatus::Off).unwrap(),
            "\"off\""
        );
    }

    #[test]
    fn test_bgb_deduct_status_deserialization() {
        assert_eq!(
            serde_json::from_str::<BgbDeductStatus>("\"on\"").unwrap(),
            BgbDeductStatus::On
        );
        assert_eq!(
            serde_json::from_str::<BgbDeductStatus>("\"off\"").unwrap(),
            BgbDeductStatus::Off
        );
    }

    #[tokio::test]
    async fn test_get_bgb_deduct_info_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = GetBgbDeductInfoRequest::default();

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.get_bgb_deduct_info(request).await.unwrap();
        // log::info!("Response: {:?}", response);
    }
}
