//! Change position mode (dual position side) on Binance USDM REST API.

use super::UsdmClient;
use crate::binance::usdm::RestResponse;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::PositionMode;
use reqwest::Method;
use serde::Serialize;

/// Endpoint path for changing position mode.
const CHANGE_POSITION_MODE_ENDPOINT: &str = "/fapi/v1/positionSide/dual";

/// Request parameters for the Change Position Mode endpoint.
///
/// All fields are required by the Binance USDM API. See official docs for valid values.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePositionModeRequest {
    /// Position mode to set for the account.
    ///
    /// - `PositionMode::True`: Hedge Mode (dual position side)
    /// - `PositionMode::False`: One-way Mode (single position side)
    ///
    /// This field is a fixed set of values and uses the `PositionMode` enum.
    pub dual_side_position: PositionMode,

    /// Request timestamp in milliseconds since epoch.
    ///
    /// Must be a valid timestamp. Used for request authentication.
    pub timestamp: u64,
}

impl UsdmClient {
    /// Change Position Mode (Dual Position Side)
    ///
    /// Changes the position mode for the account between hedge mode (dual position side)
    /// and one-way mode (single position side). This operation is not allowed when there are open positions.
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
        self.send_signed_request::<(), _, _>(
            CHANGE_POSITION_MODE_ENDPOINT,
            Method::POST,
            request,
            1,
            false,
        )
        .await
        .map(|resp| RestResponse {
            data: (),
            headers: resp.headers,
            rate_limit_info: resp.rate_limit_info,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_position_mode_request_serialization_true() {
        let request = ChangePositionModeRequest {
            dual_side_position: PositionMode::True,
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("dualSidePosition=true"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_change_position_mode_request_serialization_false() {
        let request = ChangePositionModeRequest {
            dual_side_position: PositionMode::False,
            timestamp: 1625097600000,
        };
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("dualSidePosition=false"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }
}
