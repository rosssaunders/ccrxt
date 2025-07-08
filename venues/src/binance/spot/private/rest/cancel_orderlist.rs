use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{ContingencyType, OrderListOrderStatus, OrderListStatus, RestResult};

/// Request parameters for canceling an order list
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderListRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order list ID
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    pub order_list_id: Option<u64>,

    /// List client order ID
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,

    /// New client order ID
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Cancel order list response
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderListResponse {
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

    /// Orders in the canceled list
    #[serde(rename = "orders")]
    pub orders: Vec<CancelOrderListOrder>,

    /// Order reports
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<serde_json::Value>,
}

/// Order information in the canceled order list
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderListOrder {
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
    /// Cancel an entire order list
    ///
    /// Cancel an entire order list.
    /// Either orderListId or listClientOrderId must be provided.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#cancel-order-list--trade)
    /// Method: DELETE /api/v3/orderList
    /// Weight: 1
    /// Security: TRADE
    pub async fn cancel_order_list(
        &self,
        params: CancelOrderListRequest,
    ) -> RestResult<CancelOrderListResponse> {
        let body_params: Vec<(&str, String)> = vec![("symbol", params.symbol)]
            .into_iter()
            .chain(params.order_list_id.map(|v| ("orderListId", v.to_string())))
            .chain(
                params
                    .list_client_order_id
                    .map(|v| ("listClientOrderId", v)),
            )
            .chain(params.new_client_order_id.map(|v| ("newClientOrderId", v)))
            .chain(params.recv_window.map(|v| ("recvWindow", v.to_string())))
            .collect();

        let body: Vec<(&str, &str)> = body_params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.send_request(
            "/api/v3/orderList",
            reqwest::Method::DELETE,
            None,
            Some(&body),
            1,
            true,
        )
        .await
    }
}
