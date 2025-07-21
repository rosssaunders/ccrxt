use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::{
    RestResult,
    enums::{OrderSide, PriceMatch},
};

const MODIFY_BATCH_ORDERS_ENDPOINT: &str = "/fapi/v1/batchOrders";

/// Request to modify multiple orders in a batch.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyBatchOrdersRequest {
    /// List of order modifications (max 5)
    pub batch_orders: Vec<ModifyBatchOrderItem>,
}

/// Individual order modification in a batch.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyBatchOrderItem {
    /// Order ID to modify (either this or origClientOrderId must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID (either this or orderId must be provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<Cow<'static, str>>,

    /// Symbol
    pub symbol: Cow<'static, str>,

    /// Order side
    pub side: OrderSide,

    /// Order quantity
    pub quantity: Cow<'static, str>,

    /// Order price
    pub price: Cow<'static, str>,

    /// Price match mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,
}

/// Response for modified order in batch (can be success or error).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ModifyBatchOrderResponse {
    /// Successful modification
    Success(ModifyBatchOrderSuccess),
    /// Error during modification
    Error { code: i64, msg: String },
}

/// Successful order modification response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyBatchOrderSuccess {
    /// Trading symbol.
    pub symbol: String,

    /// Order ID.
    pub order_id: u64,

    /// Client order ID.
    pub client_order_id: String,

    /// Transaction time (timestamp in milliseconds).
    pub transact_time: u64,

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
    /// Modify multiple orders (PUT /fapi/v1/batchOrders)
    ///
    /// Modifies multiple orders in a single batch for USDM futures.
    ///
    /// [docs]: https://binance-docs.github.io/apidocs/futures/en/#modify-multiple-orders-trade
    ///
    /// Rate limit: 5 weight
    ///
    /// # Arguments
    /// * `request` - The batch order modification request parameters
    ///
    /// # Returns
    /// Vector of modification responses, one for each order in the batch
    pub async fn modify_batch_orders(
        &self,
        request: ModifyBatchOrdersRequest,
    ) -> RestResult<Vec<ModifyBatchOrderResponse>> {
        self.send_signed_request(MODIFY_BATCH_ORDERS_ENDPOINT, Method::PUT, request, 5, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_batch_order_item_serialization() {
        let item = ModifyBatchOrderItem {
            order_id: Some(123456789),
            orig_client_order_id: None,
            symbol: "BTCUSDT".into(),
            side: OrderSide::Buy,
            quantity: "0.1".into(),
            price: "50000".into(),
            price_match: None,
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains(r#""orderId":123456789"#));
        assert!(json.contains(r#""symbol":"BTCUSDT""#));
        assert!(json.contains(r#""side":"BUY""#));
        assert!(json.contains(r#""quantity":"0.1""#));
        assert!(json.contains(r#""price":"50000""#));
        assert!(!json.contains("origClientOrderId"));
    }

    #[test]
    fn test_modify_batch_orders_request_serialization() {
        let request = ModifyBatchOrdersRequest {
            batch_orders: vec![ModifyBatchOrderItem {
                order_id: Some(123456789),
                orig_client_order_id: None,
                symbol: "BTCUSDT".into(),
                side: OrderSide::Buy,
                quantity: "0.1".into(),
                price: "50000".into(),
                price_match: None,
            }],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""batchOrders":[{"#));
        assert!(json.contains(r#""orderId":123456789"#));
    }

    #[test]
    fn test_modify_batch_order_response_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 123456789,
            "clientOrderId": "test123",
            "transactTime": 1625184000000,
            "price": "50000.00",
            "origQty": "0.100",
            "executedQty": "0.000",
            "cumQuote": "0.00000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "positionSide": "LONG",
            "updateTime": 1625184001000
        }"#;

        let response: ModifyBatchOrderSuccess = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 123456789);
        assert_eq!(response.client_order_id, "test123");
        assert_eq!(response.price, "50000.00");
        assert_eq!(response.status, "NEW");
    }
}
