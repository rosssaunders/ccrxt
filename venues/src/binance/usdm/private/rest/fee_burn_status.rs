use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const FEE_BURN_STATUS_ENDPOINT: &str = "/fapi/v1/feeBurn";

/// Request parameters for getting BNB burn status.
///
/// Used to query the current BNB burn status for futures trading fees.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeBurnStatusRequest {}

/// Response from the get fee burn status endpoint.
///
/// Contains the current BNB burn status for futures trading fees and margin interest.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeBurnStatusResponse {
    /// Whether BNB burn is currently enabled for futures trading fees.
    pub fee_burn: bool,
}

impl UsdmClient {
    /// Get BNB Burn Status (USER_DATA)
    ///
    /// Get current BNB burn status for futures trading and margin interest.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-BNB-Burn-Status
    ///
    /// Rate limit: 5
    ///
    /// # Arguments
    /// * `request` - The get fee burn status request parameters
    ///
    /// # Returns
    /// Response containing the current BNB burn status
    pub async fn get_fee_burn_status(
        &self,
        request: GetFeeBurnStatusRequest,
    ) -> RestResult<FeeBurnStatusResponse> {
        self.send_signed_request(FEE_BURN_STATUS_ENDPOINT, Method::GET, request, 5, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fee_burn_status_request_serialization() {
        let request = GetFeeBurnStatusRequest {};

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_fee_burn_status_response_deserialization_enabled() {
        let json = r#"
        {
            "feeBurn": true
        }
        "#;

        let response: FeeBurnStatusResponse = serde_json::from_str(json).unwrap();
        assert!(response.fee_burn);
    }

    #[test]
    fn test_fee_burn_status_response_deserialization_disabled() {
        let json = r#"
        {
            "feeBurn": false
        }
        "#;

        let response: FeeBurnStatusResponse = serde_json::from_str(json).unwrap();
        assert!(!response.fee_burn);
    }
}
