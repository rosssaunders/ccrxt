// Get Order Modify History (USER_DATA) endpoint implementation for GET /dapi/v1/orderAmendment
// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Order-Modify-History>

use serde::{Deserialize, Serialize};

use crate::binance::coinm::RestResult;
use crate::binance::coinm::private::rest::client::RestClient;
use crate::binance::shared;

/// Request parameters for getting order modification history (GET /dapi/v1/orderAmendment).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrderModifyHistoryRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// Timestamp in ms to get modification history from (INCLUSIVE).
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// Timestamp in ms to get modification history until (INCLUSIVE).
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 50; max 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Represents a field change in order modification.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderAmendmentField {
    /// Value before the modification.
    pub before: String,

    /// Value after the modification.
    pub after: String,
}

/// Represents the changes made in an order modification.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderAmendment {
    /// Price change information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<OrderAmendmentField>,

    /// Original quantity change information.
    #[serde(rename = "origQty", skip_serializing_if = "Option::is_none")]
    pub orig_qty: Option<OrderAmendmentField>,

    /// Order modification count, representing the number of times the order has been modified.
    pub count: u32,
}

/// Individual order modification history entry.
#[derive(Debug, Clone, Deserialize)]
pub struct OrderModifyHistoryEntry {
    /// Order modification ID.
    #[serde(rename = "amendmentId")]
    pub amendment_id: u64,

    /// Trading symbol.
    pub symbol: String,

    /// Trading pair.
    pub pair: String,

    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Client order ID.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Order modification time.
    pub time: u64,

    /// Order modification details.
    pub amendment: OrderAmendment,
}

/// Response for getting order modification history (GET /dapi/v1/orderAmendment).
pub type GetOrderModifyHistoryResponse = Vec<OrderModifyHistoryEntry>;

impl RestClient {
    /// Gets order modification history (USER_DATA) for Binance Coin-M Futures.
    ///
    /// See: <https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Get-Order-Modify-History>
    /// GET /dapi/v1/orderAmendment
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Either orderId or origClientOrderId must be sent, and the orderId will prevail if both are sent.
    /// Order modify history longer than 3 months is not available.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`GetOrderModifyHistoryRequest`])
    ///
    /// # Returns
    /// A [`GetOrderModifyHistoryResponse`] - array of order modification history entries.
    pub async fn get_order_modify_history(
        &self,
        params: GetOrderModifyHistoryRequest,
    ) -> RestResult<GetOrderModifyHistoryResponse> {
        let weight = 1;
        shared::send_signed_request(
            self,
            "/dapi/v1/orderAmendment",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
