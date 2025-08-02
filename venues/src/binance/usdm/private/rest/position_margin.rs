use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::{MarginAction, PositionSide};

const MODIFY_POSITION_MARGIN_ENDPOINT: &str = "/fapi/v1/positionMargin";

/// Request parameters for the Modify Isolated Position Margin endpoint.
///
/// All credential fields are securely stored and expected as SecretString.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyPositionMarginRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: Cow<'static, str>,

    /// Position side (BOTH for one-way mode, LONG/SHORT for hedge mode).
    pub position_side: PositionSide,

    /// Margin amount to add or reduce.
    pub amount: Cow<'static, str>,

    /// Margin action (Add or Reduce).
    #[serde(rename = "type")]
    pub action: MarginAction,

    /// Request timestamp in milliseconds.
    pub timestamp: u64,

    /// Optional receive window (milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Response from the Modify Isolated Position Margin endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyPositionMarginResponse {
    /// Margin amount that was modified.
    pub amount: Cow<'static, str>,

    /// Response code (200 indicates success).
    pub code: i32,

    /// Response message from the API.
    pub msg: Cow<'static, str>,

    /// Margin action (Add or Reduce).
    #[serde(rename = "type")]
    pub action: MarginAction,
}

impl UsdmClient {
    /// Modify Isolated Position Margin
    ///
    /// Adds or reduces margin for an isolated position. This operation is only available for symbols in isolated margin mode and requires an existing position.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin
    ///
    /// Weight: 1
    ///
    /// # Arguments
    /// * `params` - The request parameters for modifying position margin
    ///
    /// # Returns
    /// Returns `ModifyPositionMarginResponse` containing the modified amount and action.
    ///
    /// # Errors
    /// Returns error if:
    /// - The symbol is invalid or not in isolated margin mode
    /// - The position side is invalid
    /// - The amount is invalid or insufficient balance for reduction
    /// - No position exists for the symbol/side
    /// - Authentication fails
    /// - Rate limits are exceeded
    pub async fn modify_position_margin(
        &self,
        params: ModifyPositionMarginRequest,
    ) -> RestResult<ModifyPositionMarginResponse> {
        self.send_post_signed_request(
            MODIFY_POSITION_MARGIN_ENDPOINT,
            params,
            1, // Weight per docs
            true,)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_position_margin_request_serialization() {
        let request = ModifyPositionMarginRequest {
            symbol: Cow::Borrowed("BTCUSDT"),
            position_side: PositionSide::Long,
            amount: Cow::Borrowed("100.0"),
            action: MarginAction::Add,
            timestamp: 1625097600000,
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("positionSide=LONG"));
        assert!(serialized.contains("amount=100.0"));
        assert!(serialized.contains("type=1"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(serialized.contains("recvWindow=5000"));
    }

    #[test]
    fn test_modify_position_margin_response_deserialization() {
        let json = r#"{"amount":"100.0","code":200,"msg":"Successfully modify position margin.","type":"1"}"#;
        let response: ModifyPositionMarginResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.amount, "100.0");
        assert_eq!(response.code, 200);
        assert_eq!(response.msg, "Successfully modify position margin.");
        assert_eq!(response.action, MarginAction::Add);
    }
}
