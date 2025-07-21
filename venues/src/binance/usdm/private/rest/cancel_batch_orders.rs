use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;

const CANCEL_BATCH_ORDERS_ENDPOINT: &str = "/fapi/v1/batchOrders";

/// Request to cancel multiple orders in a batch.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchOrdersRequest {
    /// List of order IDs to cancel (either this or origClientOrderIdList must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id_list: Option<Vec<u64>>,

    /// List of original client order IDs to cancel (either this or orderIdList must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id_list: Option<Vec<Cow<'static, str>>>,

    /// Symbol
    pub symbol: Cow<'static, str>,
}

/// Response for cancelled order in batch (can be success or error).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CancelBatchOrderResponse {
    /// Successful cancellation
    Success(CancelBatchOrderSuccess),
    /// Error during cancellation
    Error { code: i64, msg: String },
}

/// Successful order cancellation response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchOrderSuccess {
    /// Trading symbol.
    pub symbol: String,

    /// Order ID.
    pub order_id: u64,

    /// Client order ID.
    pub client_order_id: String,

    /// Order price.
    pub price: String,

    /// Original order quantity.
    pub orig_qty: String,

    /// Executed quantity.
    pub executed_qty: String,

    /// Cumulative quote quantity.
    pub cum_quote: String,

    /// Order status.
    pub status: String,

    /// Time in force.
    pub time_in_force: String,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: String,

    /// Order side.
    pub side: String,

    /// Position side.
    pub position_side: String,

    /// Update time (timestamp in milliseconds).
    pub update_time: u64,
}

impl UsdmClient {
    /// Cancel multiple orders (DELETE /fapi/v1/batchOrders)
    ///
    /// Cancels multiple orders in a single batch for USDM futures.
    ///
    /// [docs]: https://binance-docs.github.io/apidocs/futures/en/#cancel-multiple-orders-trade
    ///
    /// Rate limit: 5 weight
    ///
    /// # Arguments
    /// * `request` - The batch order cancellation request parameters
    ///
    /// # Returns
    /// Vector of cancellation responses, one for each order in the batch
    pub async fn cancel_batch_orders(
        &self,
        request: CancelBatchOrdersRequest,
    ) -> RestResult<Vec<CancelBatchOrderResponse>> {
        self.send_signed_request(
            CANCEL_BATCH_ORDERS_ENDPOINT,
            Method::DELETE,
            request,
            5,
            true,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_batch_orders_request_serialization() {
        let request = CancelBatchOrdersRequest {
            order_id_list: Some(vec![123456789, 987654321]),
            orig_client_order_id_list: None,
            symbol: "BTCUSDT".into(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""orderIdList":[123456789,987654321]"#));
        assert!(json.contains(r#""symbol":"BTCUSDT""#));
        assert!(!json.contains("origClientOrderIdList"));
    }

    #[test]
    fn test_cancel_batch_order_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 123456789,
            "clientOrderId": "test123",
            "price": "50000.00",
            "origQty": "0.100",
            "executedQty": "0.000",
            "cumQuote": "0.00000000",
            "status": "CANCELED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "positionSide": "LONG",
            "updateTime": 1625184001000
        }"#;

        let response: CancelBatchOrderSuccess = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 123456789);
        assert_eq!(response.client_order_id, "test123");
        assert_eq!(response.status, "CANCELED");
    }
}
