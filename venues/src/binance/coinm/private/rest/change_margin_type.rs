// Change Margin Type (TRADE) endpoint implementation for POST /dapi/v1/marginType
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Margin-Type>

use serde::{Deserialize, Serialize};

use crate::binance::coinm::MarginType;
use crate::binance::coinm::RestResult;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::shared;

/// Request parameters for changing margin type (POST /dapi/v1/marginType).
#[derive(Debug, Clone, Serialize)]
pub struct ChangeMarginTypeRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Margin type: ISOLATED or CROSSED.
    #[serde(rename = "marginType")]
    pub margin_type: MarginType,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for changing margin type (POST /dapi/v1/marginType).
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeMarginTypeResponse {
    /// Response code (200 for success).
    pub code: u32,

    /// Response message.
    pub msg: String,
}

impl RestClient {
    /// Changes user's margin type (TRADE) for a specific symbol on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Margin-Type>
    /// POST /dapi/v1/marginType
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Changes user's margin type in the specific symbol market.
    /// For Hedge Mode, LONG and SHORT positions of one symbol use the same margin type.
    /// With ISOLATED margin type, margins of the LONG and SHORT positions are isolated from each other.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ChangeMarginTypeRequest`])
    ///
    /// # Returns
    /// A [`ChangeMarginTypeResponse`] with the operation result.
    pub async fn change_margin_type(
        &self,
        params: ChangeMarginTypeRequest,
    ) -> RestResult<ChangeMarginTypeResponse> {
        shared::send_signed_request(
            self,
            "/dapi/v1/marginType",
            reqwest::Method::POST,
            params,
            1,
            true,
        )
        .await
    }
}
