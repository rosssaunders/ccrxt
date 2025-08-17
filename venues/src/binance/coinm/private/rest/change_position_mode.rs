use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, private::rest::client::RestClient};

const POSITION_SIDE_DUAL_ENDPOINT: &str = "/dapi/v1/positionSide/dual";

/// Request parameters for changing position mode (POST /dapi/v1/positionSide/dual).
#[derive(Debug, Clone, Serialize, Default)]
pub struct ChangePositionModeRequest {
    /// Position mode. "true": Hedge Mode; "false": One-way Mode.
    #[serde(rename = "dualSidePosition")]
    pub dual_side_position: String,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for changing position mode (POST /dapi/v1/positionSide/dual).
#[derive(Debug, Clone, Deserialize)]
pub struct ChangePositionModeResponse {
    /// Response code (200 for success).
    pub code: u32,

    /// Response message.
    pub msg: String,
}

impl RestClient {
    /// Changes user's position mode (TRADE) on Binance Coin-M Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Position-Mode)
    ///
    /// POST /dapi/v1/positionSide/dual
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Changes user's position mode (Hedge Mode or One-way Mode) on EVERY symbol.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ChangePositionModeRequest`])
    ///
    /// # Returns
    /// A [`ChangePositionModeResponse`] with the operation result.
    pub async fn change_position_mode(
        &self,
        params: ChangePositionModeRequest,
    ) -> RestResult<ChangePositionModeResponse> {
        self.send_post_signed_request(POSITION_SIDE_DUAL_ENDPOINT, params, 1, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_position_mode_request_serialization_hedge_mode() {
        let request = ChangePositionModeRequest {
            dual_side_position: "true".to_string(),
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("dualSidePosition=true"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_change_position_mode_request_serialization_one_way_mode() {
        let request = ChangePositionModeRequest {
            dual_side_position: "false".to_string(),
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("dualSidePosition=false"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_change_position_mode_response_deserialization() {
        let json = r#"{
            "code": 200,
            "msg": "success"
        }"#;

        let response: ChangePositionModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "success");
    }

    #[test]
    fn test_change_position_mode_response_deserialization_with_different_message() {
        let json = r#"{
            "code": 200,
            "msg": "Position mode changed successfully"
        }"#;

        let response: ChangePositionModeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "Position mode changed successfully");
    }
}
