// No top-of-file comments per project instructions.

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::PositionMode;
use reqwest::Method;
use serde::Serialize;

/// Endpoint path for changing position mode.
const CHANGE_POSITION_MODE_ENDPOINT: &str = "/fapi/v1/positionSide/dual";

/// Request parameters for the Change Position Mode endpoint.
///
/// Changes the user's position mode (Hedge Mode or One-way Mode) for all symbols.
/// All fields are required by the Binance USDM API.
///
/// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Position-Mode
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePositionModeRequest {
    /// Position mode to set for the account.
    ///
    /// - `PositionMode::True`: Hedge Mode (dual position side)
    /// - `PositionMode::False`: One-way Mode (single position side)
    ///
    /// This field is a fixed set of values and uses the `PositionMode` enum.
    /// Securely stored and expected as `PositionMode`.
    pub dual_side_position: PositionMode,

    /// Optional window for the request validity period in milliseconds.
    ///
    /// If provided, must be a positive integer. Defaults to Binance API default if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds since epoch.
    ///
    /// Must be a valid timestamp. Used for request authentication.
    pub timestamp: u64,
}

impl UsdmClient {
    /// Change Position Mode (TRADE)
    ///
    /// Changes the user's position mode (Hedge Mode or One-way Mode) for all symbols.
    /// This operation is not allowed when there are open positions.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Change-Position-Mode
    ///
    /// Rate limit: 1
    ///
    /// # Arguments
    /// * `request` - The request parameters for changing position mode
    ///
    /// # Returns
    /// Empty response on success. If there is an error, it will be returned.
    pub async fn change_position_mode(&self, request: ChangePositionModeRequest) -> RestResult<()> {
        self.send_post_signed_request(
            CHANGE_POSITION_MODE_ENDPOINT,
            request,
            1,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_position_mode_request_serialization_true() {
        let request = ChangePositionModeRequest {
            dual_side_position: PositionMode::True,
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("dualSidePosition=true"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_change_position_mode_request_serialization_false_no_recv_window() {
        let request = ChangePositionModeRequest {
            dual_side_position: PositionMode::False,
            recv_window: None,
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("dualSidePosition=false"));
        assert!(!serialized.contains("recvWindow"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }
}
