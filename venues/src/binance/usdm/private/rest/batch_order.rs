use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::UsdmClient;
use crate::binance::usdm::{
    OrderStatus, PriceMatch, RestResult, SelfTradePreventionMode,
    enums::{NewOrderRespType, OrderSide, OrderType, PositionSide, TimeInForce, WorkingType},
};

const BATCH_ORDERS_ENDPOINT: &str = "/fapi/v1/batchOrders";

/// Represents a single order in a batch order request for USDM futures.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderItem {
    /// Trading symbol (e.g., "BTCUSDT").
    pub symbol: String,

    /// Order side. See [`OrderSide`].
    pub side: OrderSide,

    /// Position side. See [`PositionSide`]. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,

    /// Order type. See [`OrderType`].
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force. See [`TimeInForce`]. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Order quantity. Required for most order types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    /// Order price. Optional for market orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Client order ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Stop price. Used with STOP/STOP_MARKET or TAKE_PROFIT/TAKE_PROFIT_MARKET orders. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Activation price. Used with TRAILING_STOP_MARKET orders. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<String>,

    /// Callback rate. Used with TRAILING_STOP_MARKET orders. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<String>,

    /// Close position flag. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_position: Option<bool>,

    /// Reduce only flag. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Working type. See [`WorkingType`]. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,

    /// Price protection flag. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_protect: Option<bool>,

    /// New order response type. See [`NewOrderRespType`]. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<NewOrderRespType>,

    /// Price match mode. See [`PriceMatch`]. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,

    /// Self trade prevention mode. See [`SelfTradePreventionMode`]. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    /// Good till date (timestamp in ms). Optional, required for GTD orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub good_till_date: Option<u64>,
}

/// Request parameters for placing multiple orders in a single batch for USDM futures.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderRequest {
    /// List of orders to place (max 5).
    pub batch_orders: Vec<BatchOrderItem>,

    /// Optional: The number of milliseconds the request is valid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Response for a single order in the batch.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResponse {
    /// Trading symbol.
    pub symbol: String,

    /// Order ID.
    pub order_id: u64,

    /// Client order ID.
    pub client_order_id: String,

    /// Transaction time (timestamp in milliseconds).
    #[serde(default)]
    pub transact_time: Option<u64>,

    /// Order price.
    pub price: String,

    /// Original order quantity.
    pub orig_qty: String,

    /// Executed quantity.
    pub executed_qty: String,

    /// Cumulative quote quantity.
    pub cum_quote: String,

    /// Order status. See [`OrderStatus`].
    pub status: OrderStatus,

    /// Time in force. See [`TimeInForce`].
    pub time_in_force: TimeInForce,

    /// Order type. See [`OrderType`].
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side. See [`OrderSide`].
    pub side: OrderSide,

    /// Position side. See [`PositionSide`].
    pub position_side: PositionSide,

    /// Working type. See [`WorkingType`].
    pub working_type: WorkingType,

    /// Reduce only flag.
    #[serde(default)]
    pub reduce_only: Option<bool>,

    /// Stop price. Only present for STOP/TAKE_PROFIT orders.
    #[serde(default)]
    pub stop_price: Option<String>,

    /// Activation price. Only present for TRAILING_STOP_MARKET orders.
    #[serde(default)]
    pub activate_price: Option<String>,

    /// Callback rate. Only present for TRAILING_STOP_MARKET orders.
    #[serde(default)]
    pub price_rate: Option<String>,

    /// Order average price. Only present for some order types.
    #[serde(default)]
    pub avg_price: Option<String>,

    /// Order update time (timestamp in ms).
    #[serde(default)]
    pub update_time: Option<u64>,

    /// Price protection flag.
    #[serde(default)]
    pub price_protect: Option<bool>,

    /// Price match mode. See [`PriceMatch`].
    #[serde(default)]
    pub price_match: Option<PriceMatch>,

    /// Self trade prevention mode. See [`SelfTradePreventionMode`].
    #[serde(default)]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    /// Good till date (timestamp in ms). Only present for GTD orders.
    #[serde(default)]
    pub good_till_date: Option<u64>,

    /// Order original type. See [`OrderType`].
    #[serde(default)]
    pub orig_type: Option<OrderType>,
}

impl UsdmClient {
    /// Place Multiple Orders
    ///
    /// Places multiple orders in a single batch for USDM futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api/Place-Multiple-Orders
    ///
    /// Rate limit: 5 weight
    ///
    /// # Arguments
    /// * `request` - The batch order request parameters
    ///
    /// # Returns
    /// Vector of order responses, one for each order in the batch
    pub async fn place_batch_orders(
        &self,
        request: BatchOrderRequest,
    ) -> RestResult<Vec<BatchOrderResponse>> {
        self.send_post_signed_request(
            BATCH_ORDERS_ENDPOINT,
            request, 5, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::usdm::enums::*;
    use serde_json;

    #[test]
    fn test_batch_order_item_serialization() {
        let item = BatchOrderItem {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            position_side: Some(PositionSide::Long),
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some("0.1".to_string()),
            price: Some("50000".to_string()),
            new_client_order_id: Some("test123".to_string()),
            stop_price: Some("49000".to_string()),
            activation_price: Some("48000".to_string()),
            callback_rate: Some("0.3".to_string()),
            close_position: Some(false),
            reduce_only: Some(false),
            working_type: Some(WorkingType::ContractPrice),
            price_protect: Some(true),
            new_order_resp_type: Some(NewOrderRespType::Result),
            price_match: Some(PriceMatch::Opponent),
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireTaker),
            good_till_date: Some(1693207680000),
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains(r#""symbol":"BTCUSDT""#));
        assert!(json.contains(r#""side":"BUY""#));
        assert!(json.contains(r#""type":"LIMIT""#));
        assert!(json.contains(r#""quantity":"0.1""#));
        assert!(json.contains(r#""price":"50000""#));
        assert!(json.contains(r#""activationPrice":"48000""#));
        assert!(json.contains(r#""callbackRate":"0.3""#));
        assert!(json.contains(r#""priceProtect":true"#));
    }

    #[test]
    fn test_batch_order_request_serialization() {
        let request = BatchOrderRequest {
            batch_orders: vec![BatchOrderItem {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                position_side: Some(PositionSide::Long),
                order_type: OrderType::Limit,
                time_in_force: Some(TimeInForce::GTC),
                quantity: Some("0.1".to_string()),
                price: Some("50000".to_string()),
                new_client_order_id: Some("order1".to_string()),
                stop_price: Some("49000".to_string()),
                activation_price: None,
                callback_rate: None,
                close_position: Some(false),
                reduce_only: Some(false),
                working_type: Some(WorkingType::ContractPrice),
                price_protect: Some(false),
                new_order_resp_type: None,
                price_match: None,
                self_trade_prevention_mode: None,
                good_till_date: None,
            }],
            recv_window: Some(5000),
            timestamp: 1625184000000,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains(r#""batchOrders":[{"#));
        assert!(json.contains(r#""symbol":"BTCUSDT""#));
        assert!(json.contains(r#""recvWindow":5000"#));
        assert!(json.contains(r#""timestamp":1625184000000"#));
    }

    #[test]
    fn test_batch_order_response_deserialization() {
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
            "workingType": "CONTRACT_PRICE",
            "reduceOnly": false,
            "stopPrice": "49000",
            "activatePrice": "48000",
            "priceRate": "0.3",
            "avgPrice": "0.00000",
            "updateTime": 1625184001000,
            "priceProtect": true,
            "priceMatch": "OPPONENT",
            "selfTradePreventionMode": "EXPIRE_TAKER",
            "goodTillDate": 1693207680000,
            "origType": "LIMIT"
        }"#;

        let response: BatchOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 123456789);
        assert_eq!(response.client_order_id, "test123");
        assert_eq!(response.price, "50000.00");
        assert_eq!(response.status, OrderStatus::New);
        assert_eq!(response.time_in_force, TimeInForce::GTC);
        assert_eq!(response.order_type, OrderType::Limit);
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(response.position_side, PositionSide::Long);
        assert_eq!(response.working_type, WorkingType::ContractPrice);
        assert_eq!(response.reduce_only, Some(false));
        assert_eq!(response.stop_price, Some("49000".to_string()));
        assert_eq!(response.activate_price, Some("48000".to_string()));
        assert_eq!(response.price_rate, Some("0.3".to_string()));
        assert_eq!(response.avg_price, Some("0.00000".to_string()));
        assert_eq!(response.update_time, Some(1625184001000));
        assert_eq!(response.price_protect, Some(true));
        assert_eq!(response.price_match, Some(PriceMatch::Opponent));
        assert_eq!(
            response.self_trade_prevention_mode,
            Some(SelfTradePreventionMode::ExpireTaker)
        );
        assert_eq!(response.good_till_date, Some(1693207680000));
        assert_eq!(response.orig_type, Some(OrderType::Limit));
    }
}
