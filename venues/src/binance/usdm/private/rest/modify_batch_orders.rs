use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{
    RestResult,
    enums::{
        OrderSide, OrderStatus, OrderType, PositionSide, PriceMatch, SelfTradePreventionMode,
        TimeInForce, WorkingType,
    },
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
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,

    /// Trading pair (e.g., "BTCUSDT").
    pub pair: String,

    /// Order status. See [`OrderStatus`] for possible values.
    pub status: OrderStatus,

    /// Order ID.
    pub order_id: u64,

    /// Client order ID.
    pub client_order_id: String,

    /// Order price.
    pub price: String,

    /// Average price (filled).
    pub avg_price: String,

    /// Original order quantity.
    pub orig_qty: String,

    /// Executed quantity.
    pub executed_qty: String,

    /// Cumulative filled quantity.
    pub cum_qty: String,

    /// Cumulative base quantity.
    pub cum_base: String,

    /// Time in force. See [`TimeInForce`] for possible values.
    pub time_in_force: TimeInForce,

    /// Order type. See [`OrderType`] for possible values.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Reduce-only flag.
    pub reduce_only: bool,

    /// Close position flag.
    pub close_position: bool,

    /// Order side. See [`OrderSide`] for possible values.
    pub side: OrderSide,

    /// Position side. See [`PositionSide`] for possible values.
    pub position_side: PositionSide,

    /// Stop price (if applicable).
    pub stop_price: String,

    /// Working type. See [`WorkingType`] for possible values.
    pub working_type: WorkingType,

    /// Price protection enabled.
    pub price_protect: bool,

    /// Original order type. See [`OrderType`] for possible values.
    pub orig_type: OrderType,

    /// Price match mode. See [`PriceMatch`] for possible values.
    pub price_match: PriceMatch,

    /// Self-trade prevention mode. See [`SelfTradePreventionMode`] for possible values.
    pub self_trade_prevention_mode: SelfTradePreventionMode,

    /// Good-till-date (timestamp in milliseconds, for GTD orders).
    pub good_till_date: u64,

    /// Update time (timestamp in milliseconds).
    pub update_time: u64,

    /// Transaction time (timestamp in milliseconds).
    pub transact_time: u64,
}

impl UsdmClient {
    /// Modify multiple orders (PUT /fapi/v1/batchOrders)
    ///
    /// Modifies multiple orders in a single batch for USDM futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Modify-Multiple-Orders
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
        self.send_put_signed_request(MODIFY_BATCH_ORDERS_ENDPOINT, request, 5, true)
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
            "pair": "BTCUSDT",
            "orderId": 123456789,
            "clientOrderId": "test123",
            "transactTime": 1625184000000,
            "price": "50000.00",
            "avgPrice": "0.00",
            "origQty": "0.100",
            "executedQty": "0.000",
            "cumQty": "0.000",
            "cumBase": "0.000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "reduceOnly": false,
            "closePosition": false,
            "side": "BUY",
            "positionSide": "LONG",
            "stopPrice": "0.00",
            "workingType": "CONTRACT_PRICE",
            "priceProtect": false,
            "origType": "LIMIT",
            "priceMatch": "NONE",
            "selfTradePreventionMode": "NONE",
            "goodTillDate": 0,
            "updateTime": 1625184001000
        }"#;

        let response: ModifyBatchOrderSuccess = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.pair, "BTCUSDT");
        assert_eq!(response.order_id, 123456789);
        assert_eq!(response.client_order_id, "test123");
        assert_eq!(response.price, "50000.00");
        assert_eq!(response.status, OrderStatus::New);
    }
}
