// Modify Isolated Position Margin (TRADE) endpoint implementation for POST /dapi/v1/positionMargin
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin>

use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::coinm::{MarginModificationType, PositionSide};
use crate::binance::shared;

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
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Isolated-Position-Margin>
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
        shared::send_signed_request(
            self,
            "/dapi/v1/positionMargin",
            reqwest::Method::POST,
            params,
            weight,
            true,
        )
        .await
    }
}
