use serde::{Deserialize, Serialize};

use crate::binance::coinm::{
    MarginModificationType, PositionSide, RestResult, private::rest::client::RestClient,
};

const POSITION_MARGIN_ENDPOINT: &str = "/dapi/v1/positionMargin";

/// Request parameters for modifying isolated position margin (POST /dapi/v1/positionMargin).
#[derive(Debug, Clone, Serialize)]
pub struct ModifyIsolatedPositionMarginRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Position side. Default BOTH for One-way Mode; LONG or SHORT for Hedge Mode.
    /// It must be sent in Hedge Mode.
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,

    /// Margin amount.
    pub amount: String,

    /// Margin modification type: 1 = Add position margin, 2 = Reduce position margin.
    #[serde(rename = "type")]
    pub modification_type: MarginModificationType,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for modifying isolated position margin (POST /dapi/v1/positionMargin).
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyIsolatedPositionMarginResponse {
    /// Margin amount.
    pub amount: f64,

    /// Response code (200 for success).
    pub code: u32,

    /// Response message.
    pub msg: String,

    /// Modification type.
    #[serde(rename = "type")]
    pub modification_type: u32,
}

impl RestClient {
    /// Modifies isolated position margin (TRADE) on Binance Coin-M Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin)
    ///
    /// POST /dapi/v1/positionMargin
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Only for isolated symbol.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ModifyIsolatedPositionMarginRequest`])
    ///
    /// # Returns
    /// A [`ModifyIsolatedPositionMarginResponse`] with the operation result.
    pub async fn modify_isolated_position_margin(
        &self,
        params: ModifyIsolatedPositionMarginRequest,
    ) -> RestResult<ModifyIsolatedPositionMarginResponse> {
        let weight = 1;
        self.send_post_signed_request(POSITION_MARGIN_ENDPOINT, params, weight, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_isolated_position_margin_request_serialization_one_way() {
        let request = ModifyIsolatedPositionMarginRequest {
            symbol: "BTCUSD_PERP".to_string(),
            position_side: None,
            amount: "0.01".to_string(),
            modification_type: MarginModificationType::Add,
            recv_window: None,
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("amount=0.01"));
        assert!(serialized.contains("type=1"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("positionSide"));
    }

    #[test]
    fn test_modify_isolated_position_margin_request_serialization_hedge_mode() {
        let request = ModifyIsolatedPositionMarginRequest {
            symbol: "BTCUSD_PERP".to_string(),
            position_side: Some(PositionSide::Long),
            amount: "0.02".to_string(),
            modification_type: MarginModificationType::Reduce,
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("positionSide=LONG"));
        assert!(serialized.contains("amount=0.02"));
        assert!(serialized.contains("type=2"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_modify_isolated_position_margin_response_deserialization() {
        let json = r#"{
            "amount": 0.01,
            "code": 200,
            "msg": "Successfully modify position margin.",
            "type": 1
        }"#;
        let response: ModifyIsolatedPositionMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.amount, 0.01);
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "Successfully modify position margin.");
        assert_eq!(response.modification_type, 1);
    }

    #[test]
    fn test_modify_isolated_position_margin_response_deserialization_reduce() {
        let json = r#"{
            "amount": 0.005,
            "code": 200,
            "msg": "Successfully modify position margin.",
            "type": 2
        }"#;
        let response: ModifyIsolatedPositionMarginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.amount, 0.005);
        assert_eq!(response.code, 200);
        assert_eq!(response.modification_type, 2);
    }
}
