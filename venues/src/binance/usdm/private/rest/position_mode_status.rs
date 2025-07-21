use super::UsdmClient;
use crate::binance::usdm::RestResult;

use reqwest::Method;
use serde::{Deserialize, Serialize};

const GET_POSITION_MODE_STATUS_ENDPOINT: &str = "/fapi/v1/positionSide/dual";

/// Request parameters for the Get Position Mode Status endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionModeStatusRequest {
    /// Request timestamp in milliseconds.
    pub timestamp: u64,

    /// Optional receive window (milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response from the Get Position Mode Status endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionModeStatusResponse {
    /// Whether dual side position mode is enabled.
    pub dual_side_position: bool,
}

impl UsdmClient {
    /// Get Position Mode Status
    ///
    /// Retrieves the current position mode (Hedge Mode or One-way Mode) for the account.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Position-Mode
    ///
    /// Rate limit: 5 requests per second
    ///
    /// # Arguments
    /// * `params` - The request parameters for getting position mode status
    ///
    /// # Returns
    /// Returns `PositionModeStatusResponse` indicating whether dual side position mode is enabled.
    pub async fn get_position_mode_status(
        &self,
        params: GetPositionModeStatusRequest,
    ) -> RestResult<PositionModeStatusResponse> {
        self.send_signed_request(
            GET_POSITION_MODE_STATUS_ENDPOINT,
            Method::GET,
            params,
            5,
            true,
        )
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
    fn test_position_mode_status_response_deserialization() {
        let json = r#"{ "dualSidePosition": true }"#;
        let response: PositionModeStatusResponse = serde_json::from_str(json).unwrap();
        assert!(response.dual_side_position);
    }
}
