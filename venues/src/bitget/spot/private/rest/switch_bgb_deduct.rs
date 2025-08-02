use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bitget::spot::{RestResult, enums::*};

const SWITCH_BGB_DEDUCT_ENDPOINT: &str = "/api/v2/spot/account/switch-deduct";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchBgbDeductRequest {
    /// BGB deduct status: "on" / "off"
    pub deduct: BgbDeductStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchBgbDeductResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: u64,
    /// Success status
    pub data: bool,
}

impl RestClient {
    /// Switch BGB Deduct
    ///
    /// Switch the BGB deduct status for fee optimization.
    pub async fn switch_bgb_deduct(
        &self,
        request: SwitchBgbDeductRequest,
    ) -> RestResult<SwitchBgbDeductResponse> {
        self.send_signed_post_request(SWITCH_BGB_DEDUCT_ENDPOINT, &request, 1, false, None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_bgb_deduct_request_serialization() {
        let request = SwitchBgbDeductRequest {
            deduct: BgbDeductStatus::On,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        println!("Serialized request: {}", serialized);

        assert!(serialized.contains("\"deduct\""));
    }

    #[test]
    fn test_switch_bgb_deduct_response_deserialization() {
        let json = r#"
        {
            "code": "00000",
            "msg": "success",
            "requestTime": 1683875302853,
            "data": true
        }"#;

        let response: SwitchBgbDeductResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, "00000");
        assert!(response.data);
    }

    #[tokio::test]
    async fn test_switch_bgb_deduct_endpoint() {
        // This test requires API credentials and should be run manually
        let _request = SwitchBgbDeductRequest {
            deduct: BgbDeductStatus::On,
        };

        // Uncomment the following lines to test with real API credentials:
        // let client = BitgetRestClient::new("api_key", "secret", "passphrase", false);
        // let response = client.switch_bgb_deduct(request).await.unwrap();
        // println!("Response: {:?}", response);
    }
}
