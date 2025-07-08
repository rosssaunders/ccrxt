// Change Position Mode (TRADE) endpoint implementation for POST /dapi/v1/positionSide/dual
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Position-Mode>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

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
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Position-Mode>
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
        shared::send_signed_request(
            self,
            "/dapi/v1/positionSide/dual",
            reqwest::Method::POST,
            params,
            1,
            true,
        )
        .await
    }
}
