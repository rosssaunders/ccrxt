use serde::Serialize;

use super::{batch_cancel_orders::CancelResponse, client::RestClient};
use crate::binance::options::RestResult;

const CANCEL_ALL_BY_UNDERLYING_ENDPOINT: &str = "/eapi/v1/allOpenOrdersByUnderlying";

/// Request parameters for canceling all orders by underlying
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllByUnderlyingRequest {
    /// Underlying asset (e.g., "BTCUSDT")
    #[serde(rename = "underlying")]
    pub underlying: String,

    /// Request timeout window in milliseconds (max 60000)
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp in milliseconds
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

impl RestClient {
    /// Cancel all active orders on specified underlying
    ///
    /// Cancels all open orders for the specified underlying asset.
    ///
    /// [docs]: (https://developers.binance.com/docs/derivatives/option/trade/Cancel-all-Active-Orders-on-Specified-Underlying)
    /// Method: DELETE /eapi/v1/allOpenOrdersByUnderlying
    /// Weight: 1
    /// Requires: API key and signature
    pub async fn cancel_all_orders_by_underlying(
        &self,
        params: CancelAllByUnderlyingRequest,
    ) -> RestResult<Vec<CancelResponse>> {
        self.send_delete_signed_request(
            CANCEL_ALL_BY_UNDERLYING_ENDPOINT,
            params,
            1,
            true, // is_order = true for order endpoints
        )
        .await
    }
}
