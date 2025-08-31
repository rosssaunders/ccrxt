use serde::{Deserialize, Serialize};

use crate::binance::usdm::{RestResult, private_client::UsdmClient};

/// Endpoint path for getting BNB burn status.
const FEE_BURN_STATUS_ENDPOINT: &str = "/fapi/v1/feeBurn";

/// Request parameters for the Get BNB Burn Status endpoint.
///
/// Used to query the current BNB burn status for futures trading fees and margin interest.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeBurnStatusRequest {
    /// The value cannot be greater than 60000
    ///
    /// Optional. If not sent, default is 5000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds since epoch.
    pub timestamp: u64,
}

/// Response from the Get BNB Burn Status endpoint.
///
/// Contains the current BNB burn status for futures trading fees and margin interest.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeBurnStatusResponse {
    /// Whether BNB burn is currently enabled for futures trading fees.
    ///
    /// `true`: Fee Discount On; `false`: Fee Discount Off
    pub fee_burn: bool,
}

impl UsdmClient {
    /// Get BNB Burn Status (USER_DATA)
    ///
    /// Get user's BNB Fee Discount (Fee Discount On or Fee Discount Off).
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-BNB-Burn-Status)
    ///
    /// Rate limit: 30
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
        self.send_get_signed_request(FEE_BURN_STATUS_ENDPOINT, request, 30, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fee_burn_status_request_serialization_empty_recv_window() {
        let request = GetFeeBurnStatusRequest {
            recv_window: None,
            timestamp: 1234567890,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        // Only timestamp should be present
        assert_eq!(serialized, "timestamp=1234567890");
    }

    #[test]
    fn test_get_fee_burn_status_request_serialization_with_recv_window() {
        let request = GetFeeBurnStatusRequest {
            recv_window: Some(60000),
            timestamp: 1234567890,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=60000"));
        assert!(serialized.contains("timestamp=1234567890"));
    }

    #[test]
    fn test_fee_burn_status_response_deserialization_enabled() {
        let json = r#"{"feeBurn": true}"#;
        let response: FeeBurnStatusResponse = serde_json::from_str(json).unwrap();
        assert!(response.fee_burn);
    }

    #[test]
    fn test_fee_burn_status_response_deserialization_disabled() {
        let json = r#"{"feeBurn": false}"#;
        let response: FeeBurnStatusResponse = serde_json::from_str(json).unwrap();
        assert!(!response.fee_burn);
    }
}
