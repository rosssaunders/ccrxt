use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const TOGGLE_FEE_BURN_ENDPOINT: &str = "/fapi/v1/feeBurn";

/// Request parameters for toggling BNB burn status.
///
/// Enables or disables BNB burn for futures trading fees and margin interest.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFeeBurnRequest {
    /// Whether to enable BNB burn for fees. True to enable, false to disable.
    pub fee_burn: bool,
}

/// Response from the toggle fee burn endpoint.
///
/// Contains the result code and message indicating the success or failure
/// of the fee burn status change operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleFeeBurnResponse {
    /// Response code (200 indicates success).
    pub code: i32,

    /// Response message describing the result.
    pub msg: String,
}

impl UsdmClient {
    /// Toggle BNB Burn On Futures Trading And Margin Interest (USER_DATA)
    ///
    /// Toggle BNB burn on futures trading and margin interest.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Toggle-BNB-Burn-On-Futures-Trade)
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `request` - The toggle fee burn request parameters
    ///
    /// # Returns
    /// Response containing the operation result code and message
    pub async fn toggle_fee_burn(
        &self,
        request: ToggleFeeBurnRequest,
    ) -> RestResult<ToggleFeeBurnResponse> {
        self.send_post_signed_request(TOGGLE_FEE_BURN_ENDPOINT, request, 1, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_fee_burn_request_serialization() {
        let request = ToggleFeeBurnRequest { fee_burn: true };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "feeBurn=true");
    }

    #[test]
    fn test_toggle_fee_burn_request_serialization_false() {
        let request = ToggleFeeBurnRequest { fee_burn: false };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "feeBurn=false");
    }

    #[test]
    fn test_toggle_fee_burn_response_deserialization() {
        let json = r#"
        {
            "code": 200,
            "msg": "success"
        }
        "#;

        let response: ToggleFeeBurnResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
    }

    #[test]
    fn test_toggle_fee_burn_response_error() {
        let json = r#"
        {
            "code": -1022,
            "msg": "Signature for this request is not valid"
        }
        "#;

        let response: ToggleFeeBurnResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, -1022);
        assert_eq!(response.msg, "Signature for this request is not valid");
    }
}
