use serde::{Deserialize, Serialize};

use crate::binance::spot::{ContingencyType, OrderListOrderStatus, OrderListStatus, RestResult};

use super::client::RestClient;

/// Request parameters for getting all order lists
#[derive(Debug, Clone, Serialize, Default)]
pub struct AllOrderListRequest {
    /// From ID
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

    /// Start time timestamp in ms
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time timestamp in ms
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 500; max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// All order list information
#[derive(Debug, Clone, Deserialize)]
pub struct AllOrderList {
    /// Order list ID
    #[serde(rename = "orderListId")]
    pub order_list_id: u64,

    /// Contingency type
    #[serde(rename = "contingencyType")]
    pub contingency_type: ContingencyType,

    /// List status type
    #[serde(rename = "listStatusType")]
    pub list_status_type: OrderListStatus,

    /// List order status
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: OrderListOrderStatus,

    /// List client order ID
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: String,

    /// Transaction time
    #[serde(rename = "transactionTime")]
    pub transaction_time: u64,

    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Orders in the list
    #[serde(rename = "orders")]
    pub orders: Vec<AllOrderListOrder>,
}

/// Order information in the all order list
#[derive(Debug, Clone, Deserialize)]
pub struct AllOrderListOrder {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
}

impl RestClient {
    /// Retrieve all order lists
    ///
    /// Retrieve all order lists.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-all-order-lists--user_data)
    /// Method: GET /api/v3/allOrderList
    /// Weight: 20
    /// Security: USER_DATA
    pub async fn get_all_order_lists(
        &self,
        params: Option<AllOrderListRequest>,
    ) -> RestResult<Vec<AllOrderList>> {
        let query_string = if let Some(p) = params {
            if p.from_id.is_some()
                || p.start_time.is_some()
                || p.end_time.is_some()
                || p.limit.is_some()
                || p.recv_window.is_some()
            {
                Some(serde_urlencoded::to_string(&p).map_err(|e| {
                    crate::binance::spot::Errors::Error(format!("URL encoding error: {e}"))
                })?)
            } else {
                None
            }
        } else {
            None
        };

        self.send_request(
            "/api/v3/allOrderList",
            reqwest::Method::GET,
            query_string.as_deref(),
            None,
            20,
            false,
        )
        .await
    }
}
