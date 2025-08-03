use serde::Serialize;

use super::batch_cancel_orders::CancelResponse;
use super::client::RestClient;
use crate::binance::options::RestResult;

const CANCEL_ALL_BY_SYMBOL_ENDPOINT: &str = "/eapi/v1/allOpenOrders";

/// Request parameters for canceling all orders by symbol
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllBySymbolRequest {
    /// Option trading pair
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

impl RestClient {
    /// Cancel all active orders on a symbol
    ///
    /// Cancels all open orders for the specified symbol.
    ///
    /// [docs]: (https://developers.binance.com/docs/derivatives/option/trade/Cancel-all-Active-Orders-on-a-Symbol)
    /// Method: DELETE /eapi/v1/allOpenOrders
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn cancel_all_orders_by_symbol(
        &self,
        params: CancelAllBySymbolRequest,
    ) -> RestResult<Vec<CancelResponse>> {
        self.send_delete_signed_request(
            CANCEL_ALL_BY_SYMBOL_ENDPOINT,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }
}