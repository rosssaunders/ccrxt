// Get Current Position Mode (USER_DATA) endpoint implementation for GET /dapi/v1/positionSide/dual
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Current-Position-Mode>

use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::shared;

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
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/account/rest-api/Get-Current-Position-Mode>
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
        shared::send_signed_request(
            self,
            "/dapi/v1/positionSide/dual",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
