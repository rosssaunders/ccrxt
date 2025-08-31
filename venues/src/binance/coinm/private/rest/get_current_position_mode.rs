use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, private_client::RestClient};

const POSITION_SIDE_DUAL_ENDPOINT: &str = "/dapi/v1/positionSide/dual";

/// Request parameters for getting current position mode (GET /dapi/v1/positionSide/dual).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetCurrentPositionModeRequest {
    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for getting current position mode (GET /dapi/v1/positionSide/dual).
#[derive(Debug, Clone, Deserialize)]
pub struct GetCurrentPositionModeResponse {
    /// Position mode. "true": Hedge Mode; "false": One-way Mode.
    #[serde(rename = "dualSidePosition")]
    pub dual_side_position: bool,
}

impl RestClient {
    /// Gets user's current position mode (USER_DATA) on Binance Coin-M Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Current-Position-Mode)
    ///
    /// GET /dapi/v1/positionSide/dual
    /// Weight: 30
    /// Requires API key and signature.
    ///
    /// Gets user's position mode (Hedge Mode or One-way Mode) on EVERY symbol.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetCurrentPositionModeRequest`])
    ///
    /// # Returns
    /// A [`GetCurrentPositionModeResponse`] with the current position mode.
    pub async fn get_current_position_mode(
        &self,
        params: GetCurrentPositionModeRequest,
    ) -> RestResult<GetCurrentPositionModeResponse> {
        let weight = 30;
        self.send_get_signed_request(POSITION_SIDE_DUAL_ENDPOINT, params, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_position_mode_request_serialization() {
        let request = GetCurrentPositionModeRequest {
            recv_window: None,
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "timestamp=1625097600000");
    }

    #[test]
    fn test_get_current_position_mode_request_serialization_with_recv_window() {
        let request = GetCurrentPositionModeRequest {
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_get_current_position_mode_response_deserialization_hedge_mode() {
        let json = r#"{
            "dualSidePosition": true
        }"#;
        let response: GetCurrentPositionModeResponse = serde_json::from_str(json).unwrap();
        assert!(response.dual_side_position);
    }

    #[test]
    fn test_get_current_position_mode_response_deserialization_one_way_mode() {
        let json = r#"{
            "dualSidePosition": false
        }"#;
        let response: GetCurrentPositionModeResponse = serde_json::from_str(json).unwrap();
        assert!(!response.dual_side_position);
    }
}
