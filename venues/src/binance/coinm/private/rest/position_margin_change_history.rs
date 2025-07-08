// Get Position Margin Change History (TRADE) endpoint implementation for GET /dapi/v1/positionMargin/history
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{MarginModificationType, PositionSide, RestResult, private::rest::client::RestClient},
    shared,
};

/// Request parameters for getting position margin change history (GET /dapi/v1/positionMargin/history).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetPositionMarginChangeHistoryRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Margin modification type: 1 = Add position margin, 2 = Reduce position margin. Optional.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub modification_type: Option<MarginModificationType>,

    /// Start time in milliseconds. Optional.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time in milliseconds. Optional.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Number of records to return. Default: 50. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Individual position margin change history entry.
#[derive(Debug, Clone, Deserialize)]
pub struct PositionMarginChangeHistoryEntry {
    /// Margin amount.
    pub amount: String,

    /// Asset name.
    pub asset: String,

    /// Trading symbol.
    pub symbol: String,

    /// Time of the margin change.
    pub time: u64,

    /// Modification type: 1 = Add position margin, 2 = Reduce position margin.
    #[serde(rename = "type")]
    pub modification_type: u32,

    /// Position side.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,
}

/// Response for getting position margin change history (GET /dapi/v1/positionMargin/history).
pub type GetPositionMarginChangeHistoryResponse = Vec<PositionMarginChangeHistoryEntry>;

impl RestClient {
    /// Gets position margin change history (TRADE) on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Position-Margin-Change-History>
    /// GET /dapi/v1/positionMargin/history
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetPositionMarginChangeHistoryRequest`])
    ///
    /// # Returns
    /// A [`GetPositionMarginChangeHistoryResponse`] - array of position margin change history entries.
    pub async fn get_position_margin_change_history(
        &self,
        params: GetPositionMarginChangeHistoryRequest,
    ) -> RestResult<GetPositionMarginChangeHistoryResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            "/dapi/v1/positionMargin/history",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
