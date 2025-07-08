use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::RestResult;

/// Request parameters for querying order amendments
#[derive(Debug, Clone, Serialize)]
pub struct OrderAmendmentsRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// From execution ID
    #[serde(rename = "fromExecutionId", skip_serializing_if = "Option::is_none")]
    pub from_execution_id: Option<u64>,

    /// Default 500; max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Order amendment information
#[derive(Debug, Clone, Deserialize)]
pub struct OrderAmendment {
    /// Amendment ID
    #[serde(rename = "amendmentId")]
    pub amendment_id: u64,

    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Amendment time
    #[serde(rename = "time")]
    pub time: u64,

    /// Original quantity
    #[serde(rename = "origQty")]
    pub orig_qty: Decimal,

    /// Current quantity
    #[serde(rename = "qty")]
    pub qty: Decimal,

    /// Original price
    #[serde(rename = "origPrice")]
    pub orig_price: Decimal,

    /// Current price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Amendment status
    #[serde(rename = "status")]
    pub status: String,

    /// Execution ID
    #[serde(rename = "executionId")]
    pub execution_id: u64,
}

impl RestClient {
    /// Query all amendments of a single order
    ///
    /// Query all amendments of a single order.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-order-amendments--user_data)
    /// Method: GET /api/v3/order/amendments
    /// Weight: 4
    /// Security: USER_DATA
    pub async fn get_order_amendments(
        &self,
        params: OrderAmendmentsRequest,
    ) -> RestResult<Vec<OrderAmendment>> {
        let query_string = serde_urlencoded::to_string(&params)
            .map_err(|e| crate::binance::spot::Errors::Error(format!("URL encoding error: {e}")))?;

        self.send_request(
            "/api/v3/order/amendments",
            reqwest::Method::GET,
            Some(&query_string),
            None,
            4,
            false,
        )
        .await
    }
}
