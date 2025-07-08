// Change Initial Leverage (TRADE) endpoint implementation for POST /dapi/v1/leverage
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Initial-Leverage>

use serde::{Deserialize, Serialize};

use crate::binance::{
    coinm::{RestResult, private::rest::client::RestClient},
    shared,
};

/// Request parameters for changing initial leverage (POST /dapi/v1/leverage).
#[derive(Debug, Clone, Serialize, Default)]
pub struct ChangeInitialLeverageRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Target initial leverage: int from 1 to 125.
    pub leverage: u32,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for changing initial leverage (POST /dapi/v1/leverage).
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeInitialLeverageResponse {
    /// Applied leverage.
    pub leverage: u32,

    /// Maximum quantity of base asset.
    #[serde(rename = "maxQty")]
    pub max_qty: String,

    /// Trading symbol.
    pub symbol: String,
}

impl RestClient {
    /// Changes user's initial leverage (TRADE) for a specific symbol on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Change-Initial-Leverage>
    /// POST /dapi/v1/leverage
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Changes user's initial leverage in the specific symbol market.
    /// For Hedge Mode, LONG and SHORT positions of one symbol use the same initial leverage and share a total notional value.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ChangeInitialLeverageRequest`])
    ///
    /// # Returns
    /// A [`ChangeInitialLeverageResponse`] with the new leverage configuration.
    pub async fn change_initial_leverage(
        &self,
        params: ChangeInitialLeverageRequest,
    ) -> RestResult<ChangeInitialLeverageResponse> {
        shared::send_signed_request(
            self,
            "/dapi/v1/leverage",
            reqwest::Method::POST,
            params,
            1,
            true,
        )
        .await
    }
}
