use serde::{Deserialize, Serialize};

use crate::binance::usdm::PrivateRestClient as RestClient;
use crate::binance::usdm::RestResult;

const GET_POSITION_MODE_STATUS_ENDPOINT: &str = "/fapi/v1/positionSide/dual";

/// Request parameters for the Get Current Position Mode endpoint.
///
/// All fields are required by the Binance USDM API.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionModeStatusRequest {
    /// Request timestamp in milliseconds since epoch.
    /// Must be the current server time.
    pub timestamp: u64,

    /// Optional receive window (milliseconds).
    /// If provided, must be greater than 0 and less than 60000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response from the Get Current Position Mode endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionModeStatusResponse {
    /// Whether dual side position mode is enabled.
    /// `true` means Hedge Mode; `false` means One-way Mode.
    pub dual_side_position: bool,
}

impl RestClient {
    /// Get Current Position Mode
    ///
    /// Retrieves the current position mode (Hedge Mode or One-way Mode) for the account.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/usds-margined-futures/account/rest-api/Get-Current-Position-Mode)
    ///
    /// Request weight: 30
    /// Rate limit: 5 requests per second
    ///
    /// # Arguments
    /// * `params` - The request parameters for getting current position mode
    ///
    /// # Returns
    /// Returns `PositionModeStatusResponse` indicating whether dual side position mode is enabled (`true` for Hedge Mode, `false` for One-way Mode).
    pub async fn get_position_mode_status(
        &self,
        params: GetPositionModeStatusRequest,
    ) -> RestResult<PositionModeStatusResponse> {
        self.send_get_signed_request(GET_POSITION_MODE_STATUS_ENDPOINT, params, 30, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_mode_status_request_serialization() {
        let request = GetPositionModeStatusRequest {
            timestamp: 1625097600000,
            recv_window: Some(5000),
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_position_mode_status_response_deserialization_true() {
        let json = r#"{ "dualSidePosition": true }"#;
        let response: PositionModeStatusResponse = serde_json::from_str(json).unwrap();
        assert!(response.dual_side_position);
    }

    #[test]
    fn test_position_mode_status_response_deserialization_false() {
        let json = r#"{ "dualSidePosition": false }"#;
        let response: PositionModeStatusResponse = serde_json::from_str(json).unwrap();
        assert!(!response.dual_side_position);
    }
}
