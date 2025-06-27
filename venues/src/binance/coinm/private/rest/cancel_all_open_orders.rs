// Cancel All Open Orders (TRADE) endpoint implementation for DELETE /dapi/v1/allOpenOrders
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-All-Open-Orders>

use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::shared;

/// Request parameters for canceling all open orders (DELETE /dapi/v1/allOpenOrders).
#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelAllOpenOrdersRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for canceling all open orders (DELETE /dapi/v1/allOpenOrders).
#[derive(Debug, Clone, Deserialize)]
pub struct CancelAllOpenOrdersResponse {
    /// Response code (200 for success).
    pub code: u32,

    /// Response message.
    pub msg: String,
}

impl RestClient {
    /// Cancels all open orders (TRADE) for a symbol on Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Cancel-All-Open-Orders>
    /// DELETE /dapi/v1/allOpenOrders
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`CancelAllOpenOrdersRequest`])
    ///
    /// # Returns
    /// A [`CancelAllOpenOrdersResponse`] with the operation result.
    pub async fn cancel_all_open_orders(
        &self,
        params: CancelAllOpenOrdersRequest,
    ) -> RestResult<CancelAllOpenOrdersResponse> {
        shared::send_signed_request(
            self,
            "/dapi/v1/allOpenOrders",
            reqwest::Method::DELETE,
            params,
            1,
            true,
        )
        .await
    }
}
