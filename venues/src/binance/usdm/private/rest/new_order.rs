use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::UsdmClient;
use crate::binance::usdm::RestResult;
use crate::binance::usdm::enums::*;

const NEW_ORDER_ENDPOINT: &str = "/fapi/v1/order";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest {
    /// Trading symbol (e.g., "BTCUSDT").
    /// Required. Must be a valid symbol listed on Binance USDM Futures.
    pub symbol: Cow<'static, str>,

    /// Order side. Allowed values: BUY, SELL.
    /// Required.
    pub side: OrderSide,

    /// Position side. Allowed values: BOTH, LONG, SHORT.
    /// Optional. Default BOTH for One-way Mode; LONG or SHORT for Hedge Mode. Must be sent in Hedge Mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,

    /// Order type. Allowed values: LIMIT, MARKET, STOP, STOP_MARKET, TAKE_PROFIT, TAKE_PROFIT_MARKET, TRAILING_STOP_MARKET.
    /// Required.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force. Allowed values: GTC, IOC, FOK, GTD.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Order quantity.
    /// Optional. Cannot be sent with closePosition=true (Close-All).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,

    /// Reduce only. Allowed values: true, false. Default false.
    /// Optional. Cannot be sent in Hedge Mode; cannot be sent with closePosition=true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Order price.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Unique client order ID.
    /// Optional. A unique id among open orders. Automatically generated if not sent. Must match ^[\.A-Z\:/a-z0-9_-]{1,36}$
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<Cow<'static, str>>,

    /// Stop price for conditional orders.
    /// Optional. Used with STOP/STOP_MARKET or TAKE_PROFIT/TAKE_PROFIT_MARKET orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,

    /// Close position (Close-All).
    /// Optional. true, false; Close-All, used with STOP_MARKET or TAKE_PROFIT_MARKET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_position: Option<bool>,

    /// Activation price for trailing stop.
    /// Optional. Used with TRAILING_STOP_MARKET orders, default as the latest price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<f64>,

    /// Callback rate for trailing stop.
    /// Optional. Used with TRAILING_STOP_MARKET orders, min 0.1, max 10 where 1 for 1%.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<f64>,

    /// Working type. Allowed values: MARK_PRICE, CONTRACT_PRICE.
    /// Optional. stopPrice triggered by: "MARK_PRICE", "CONTRACT_PRICE". Default "CONTRACT_PRICE".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,

    /// Price protect. Allowed values: true, false. Default false.
    /// Optional. Used with STOP/STOP_MARKET or TAKE_PROFIT/TAKE_PROFIT_MARKET orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_protect: Option<bool>,

    /// New order response type. Allowed values: ACK, RESULT. Default ACK.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<NewOrderRespType>,

    /// Price match mode.
    /// Optional. Only available for LIMIT/STOP/TAKE_PROFIT order; can be set to OPPONENT/OPPONENT_5/OPPONENT_10/OPPONENT_20/QUEUE/QUEUE_5/QUEUE_10/QUEUE_20; Can't be passed together with price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,

    /// Self trade prevention mode.
    /// Optional. Allowed values: EXPIRE_TAKER, EXPIRE_MAKER, EXPIRE_BOTH, NONE. Only effective when timeInForce set to IOC, GTC, or GTD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    /// Good till date for GTD orders.
    /// Optional. Order cancel time for timeInForce GTD, mandatory when timeInForce set to GTD; timestamp only retains second-level precision, ms part will be ignored; must be greater than current time plus 600 seconds and smaller than 253402300799000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub good_till_date: Option<u64>,

    /// Receive window.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Request timestamp (milliseconds since epoch).
    /// Required.
    pub timestamp: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResponse {
    /// Unique client order ID.
    pub client_order_id: String,

    /// Cumulative filled quantity.
    pub cum_qty: Option<String>,

    /// Cumulative quote asset transacted.
    pub cum_quote: String,

    /// Executed quantity.
    pub executed_qty: String,

    /// Order ID.
    pub order_id: u64,

    /// Average price.
    pub avg_price: Option<String>,

    /// Original order quantity.
    pub orig_qty: String,

    /// Order price.
    pub price: String,

    /// Reduce only flag.
    pub reduce_only: Option<bool>,

    /// Order side (BUY or SELL).
    pub side: String,

    /// Position side (BOTH, LONG, SHORT).
    pub position_side: Option<String>,

    /// Order status.
    pub status: String,

    /// Stop price for conditional orders.
    pub stop_price: Option<String>,

    /// Close position (Close-All).
    pub close_position: Option<bool>,

    /// Trading symbol.
    pub symbol: String,

    /// Time in force (GTC, IOC, FOK, GTD).
    pub time_in_force: Option<String>,

    /// Order type (LIMIT, MARKET, etc).
    #[serde(rename = "type")]
    pub order_type: String,

    /// Original order type.
    pub orig_type: Option<String>,

    /// Activation price for trailing stop.
    pub activate_price: Option<String>,

    /// Callback rate for trailing stop.
    pub price_rate: Option<String>,

    /// Last update time (milliseconds since epoch).
    pub update_time: Option<u64>,

    /// Working type (MARK_PRICE, CONTRACT_PRICE).
    pub working_type: Option<String>,

    /// Price protect flag.
    pub price_protect: Option<bool>,

    /// Price match mode.
    pub price_match: Option<String>,

    /// Self trade prevention mode.
    pub self_trade_prevention_mode: Option<String>,

    /// Good till date for GTD orders.
    pub good_till_date: Option<u64>,

    /// Working time (milliseconds since epoch).
    pub working_time: Option<u64>,
}

impl UsdmClient {
    /// New Order (TRADE)
    ///
    /// Send in a new order.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/usds-margined-futures/trade/rest-api#new-ordertrade
    ///
    /// Rate limit: 1 on 10s order rate limit (X-MBX-ORDER-COUNT-10S); 1 on 1min order rate limit (X-MBX-ORDER-COUNT-1M); 0 on IP rate limit (x-mbx-used-weight-1m)
    ///
    /// # Arguments
    /// * `request` - The new order request parameters
    ///
    /// # Returns
    /// Returns a `NewOrderResponse` containing order details.
    pub async fn new_order(&self, request: NewOrderRequest) -> RestResult<NewOrderResponse> {
        self.send_post_signed_request(
            NEW_ORDER_ENDPOINT,
            request,
            1,    // weight
            true, // is_order
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::usdm::enums::{
        NewOrderRespType, OrderSide, OrderType, PositionSide, TimeInForce, WorkingType,
    };
    use serde_json;
    use std::borrow::Cow;

    #[test]
    fn test_new_order_request_serialization() {
        let req = NewOrderRequest {
            symbol: Cow::Borrowed("BTCUSDT"),
            side: OrderSide::Buy,
            position_side: Some(PositionSide::Long),
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some(0.1),
            reduce_only: Some(false),
            price: Some(30000.0),
            new_client_order_id: Some(Cow::Borrowed("my_id")),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: Some(WorkingType::MarkPrice),
            price_protect: Some(true),
            new_order_resp_type: Some(NewOrderRespType::Result),
            price_match: None,
            self_trade_prevention_mode: None,
            good_till_date: None,
            recv_window: Some(5000),
            timestamp: 1620000000000,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("BTCUSDT"));
        assert!(json.contains("LIMIT"));
        assert!(json.contains("GTC"));
        assert!(json.contains("my_id"));
        assert!(json.contains("30000"));
        assert!(json.contains("1620000000000"));
    }

    #[test]
    fn test_new_order_response_deserialization() {
        let data = r#"{
            "orderId": 123456,
            "symbol": "BTCUSDT",
            "status": "NEW",
            "clientOrderId": "my_id",
            "price": "30000.0",
            "avgPrice": "0.0",
            "origQty": "0.1",
            "executedQty": "0.0",
            "cumQuote": "0.0",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "positionSide": "LONG",
            "reduceOnly": false,
            "closePosition": false,
            "stopPrice": "0.0",
            "workingType": "MARK_PRICE",
            "priceProtect": true,
            "origType": "LIMIT",
            "activatePrice": "0.0",
            "priceRate": "0.0",
            "updateTime": 1620000000001,
            "workingTime": 1620000000002,
            "selfTradePreventionMode": "NONE"
        }"#;
        let resp: NewOrderResponse = serde_json::from_str(data).unwrap();
        assert_eq!(resp.order_id, 123456);
        assert_eq!(resp.symbol, "BTCUSDT");
        assert_eq!(resp.status, "NEW");
        assert_eq!(resp.client_order_id, "my_id");
        assert_eq!(resp.price, "30000.0");
        assert_eq!(resp.orig_qty, "0.1");
        assert_eq!(resp.time_in_force.as_deref(), Some("GTC"));
        assert_eq!(resp.order_type, "LIMIT");
        assert_eq!(resp.side, "BUY");
        assert_eq!(resp.position_side.as_deref(), Some("LONG"));
        assert_eq!(resp.reduce_only, Some(false));
        assert_eq!(resp.close_position, Some(false));
        assert_eq!(resp.price_protect, Some(true));
        assert_eq!(resp.update_time, Some(1620000000001));
        assert_eq!(resp.working_time, Some(1620000000002));
        assert_eq!(resp.self_trade_prevention_mode.as_deref(), Some("NONE"));
    }
}
