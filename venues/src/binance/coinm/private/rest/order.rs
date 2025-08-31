use serde::{Deserialize, Serialize};

use crate::binance::coinm::{
    OrderResponseType, OrderSide, OrderType, PositionSide, PriceMatch, RestResult,
    SelfTradePreventionMode, TimeInForce, WorkingType, private_client::RestClient,
};

const ORDER_ENDPOINT: &str = "/dapi/v1/order";

/// Request parameters for placing a new order (POST /dapi/v1/order).
#[derive(Debug, Clone, Serialize)]
pub struct NewOrderRequest {
    /// Trading symbol (e.g., "BTCUSD_200925").
    pub symbol: String,

    /// Order side (BUY or SELL).
    pub side: OrderSide,

    /// Position side (BOTH, LONG, SHORT). Optional.
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,

    /// Order type (LIMIT, MARKET, etc.).
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Time in force (GTC, IOC, FOK, GTX). Optional.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// Order quantity. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    /// Reduce only. Optional.
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<String>,

    /// Order price. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// New client order ID. Optional.
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Stop price. Optional.
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,

    /// Close position. Optional.
    #[serde(rename = "closePosition", skip_serializing_if = "Option::is_none")]
    pub close_position: Option<String>,

    /// Activation price (for trailing stop). Optional.
    #[serde(rename = "activationPrice", skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<String>,

    /// Callback rate (for trailing stop). Optional.
    #[serde(rename = "callbackRate", skip_serializing_if = "Option::is_none")]
    pub callback_rate: Option<String>,

    /// Working type (MARK_PRICE, CONTRACT_PRICE). Optional.
    #[serde(rename = "workingType", skip_serializing_if = "Option::is_none")]
    pub working_type: Option<WorkingType>,

    /// Price protect. Optional.
    #[serde(rename = "priceProtect", skip_serializing_if = "Option::is_none")]
    pub price_protect: Option<String>,

    /// New order response type (ACK, RESULT). Optional.
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,

    /// Price match mode. Optional.
    #[serde(rename = "priceMatch", skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,

    /// Self-trade prevention mode. Optional.
    #[serde(
        rename = "selfTradePreventionMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    /// Receive window. Optional.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp (milliseconds since epoch).
    pub timestamp: u64,
}

/// Response for a new order (POST /dapi/v1/order).
#[derive(Debug, Clone, Deserialize)]
pub struct NewOrderResponse {
    #[serde(rename = "clientOrderId")]
    pub client_order_id: Option<String>,

    #[serde(rename = "cumQty")]
    pub cum_qty: String,

    #[serde(rename = "cumBase")]
    pub cum_base: String,

    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    #[serde(rename = "orderId")]
    pub order_id: u64,

    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    #[serde(rename = "origQty")]
    pub orig_qty: String,

    pub price: String,

    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    pub side: OrderSide,

    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    pub status: String,

    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,

    #[serde(rename = "closePosition")]
    pub close_position: Option<bool>,

    pub symbol: String,

    pub pair: String,

    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<TimeInForce>,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    #[serde(rename = "origType")]
    pub orig_type: OrderType,

    #[serde(rename = "activatePrice")]
    pub activate_price: Option<String>,

    #[serde(rename = "priceRate")]
    pub price_rate: Option<String>,

    #[serde(rename = "updateTime")]
    pub update_time: u64,

    #[serde(rename = "workingType")]
    pub working_type: WorkingType,

    #[serde(rename = "priceProtect")]
    pub price_protect: bool,

    #[serde(rename = "priceMatch")]
    pub price_match: Option<PriceMatch>,

    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
}

impl RestClient {
    /// Places a new order (TRADE) on Binance Coin-M Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api)
    ///
    /// POST /dapi/v1/order
    /// Weight: 1 (order rate limit)
    /// Requires API key and signature.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`NewOrderRequest`])
    ///
    /// # Returns
    /// A [`NewOrderResponse`] object with order details.
    pub async fn post_order(&self, params: NewOrderRequest) -> RestResult<NewOrderResponse> {
        let weight = 1;
        // HIGH PERFORMANCE: Use POST-specific function, no HTTP verb branching
        self.send_post_signed_request(ORDER_ENDPOINT, params, weight, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_order_request_serialization() {
        let request = NewOrderRequest {
            symbol: "BTCUSD_PERP".to_string(),
            side: OrderSide::Buy,
            position_side: Some(PositionSide::Long),
            order_type: OrderType::Limit,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some("10.5".to_string()),
            reduce_only: None,
            price: Some("45000.0".to_string()),
            new_client_order_id: Some("my_order_123".to_string()),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
            new_order_resp_type: None,
            price_match: None,
            self_trade_prevention_mode: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("side=BUY"));
        assert!(serialized.contains("positionSide=LONG"));
        assert!(serialized.contains("type=LIMIT"));
        assert!(serialized.contains("timeInForce=GTC"));
        assert!(serialized.contains("quantity=10.5"));
        assert!(serialized.contains("price=45000.0"));
        assert!(serialized.contains("newClientOrderId=my_order_123"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_market_order_request() {
        let request = NewOrderRequest {
            symbol: "ETHUSD_PERP".to_string(),
            side: OrderSide::Sell,
            position_side: Some(PositionSide::Short),
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: Some("5.0".to_string()),
            reduce_only: Some("true".to_string()),
            price: None,
            new_client_order_id: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: Some("true".to_string()),
            new_order_resp_type: Some(OrderResponseType::Result),
            price_match: None,
            self_trade_prevention_mode: Some(SelfTradePreventionMode::ExpireTaker),
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("side=SELL"));
        assert!(serialized.contains("positionSide=SHORT"));
        assert!(serialized.contains("type=MARKET"));
        assert!(serialized.contains("quantity=5.0"));
        assert!(serialized.contains("reduceOnly=true"));
        assert!(serialized.contains("priceProtect=true"));
        assert!(serialized.contains("newOrderRespType=RESULT"));
        assert!(serialized.contains("selfTradePreventionMode=EXPIRE_TAKER"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
    }

    #[test]
    fn test_stop_order_request() {
        let request = NewOrderRequest {
            symbol: "BTCUSD_PERP".to_string(),
            side: OrderSide::Buy,
            position_side: Some(PositionSide::Long),
            order_type: OrderType::Stop,
            time_in_force: Some(TimeInForce::GTC),
            quantity: Some("20.0".to_string()),
            reduce_only: None,
            price: Some("44000.0".to_string()),
            new_client_order_id: None,
            stop_price: Some("43500.0".to_string()),
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: Some(WorkingType::MarkPrice),
            price_protect: None,
            new_order_resp_type: None,
            price_match: Some(PriceMatch::Queue),
            self_trade_prevention_mode: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("type=STOP"));
        assert!(serialized.contains("stopPrice=43500.0"));
        assert!(serialized.contains("workingType=MARK_PRICE"));
        assert!(serialized.contains("priceMatch=QUEUE"));
    }

    #[test]
    fn test_new_order_response_deserialization() {
        let json = r#"{
            "clientOrderId": "my_order_123",
            "cumQty": "0",
            "cumBase": "0",
            "executedQty": "0",
            "orderId": 123456789,
            "avgPrice": "0.0",
            "origQty": "10.5",
            "price": "45000.0",
            "reduceOnly": false,
            "side": "BUY",
            "positionSide": "LONG",
            "status": "NEW",
            "stopPrice": null,
            "closePosition": false,
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "origType": "LIMIT",
            "activatePrice": null,
            "priceRate": null,
            "updateTime": 1625097600000,
            "workingType": "CONTRACT_PRICE",
            "priceProtect": false,
            "priceMatch": "NONE",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: NewOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.client_order_id.as_ref().unwrap(), "my_order_123");
        assert_eq!(response.cum_qty, "0");
        assert_eq!(response.cum_base, "0");
        assert_eq!(response.executed_qty, "0");
        assert_eq!(response.order_id, 123456789);
        assert_eq!(response.avg_price, "0.0");
        assert_eq!(response.orig_qty, "10.5");
        assert_eq!(response.price, "45000.0");
        assert!(!response.reduce_only);
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(response.position_side, PositionSide::Long);
        assert_eq!(response.status, "NEW");
        assert!(response.stop_price.is_none());
        assert_eq!(response.close_position, Some(false));
        assert_eq!(response.symbol, "BTCUSD_PERP");
        assert_eq!(response.pair, "BTCUSD");
        assert_eq!(response.time_in_force, Some(TimeInForce::GTC));
        assert_eq!(response.order_type, OrderType::Limit);
        assert_eq!(response.orig_type, OrderType::Limit);
        assert!(response.activate_price.is_none());
        assert!(response.price_rate.is_none());
        assert_eq!(response.update_time, 1625097600000);
        assert_eq!(response.working_type, WorkingType::ContractPrice);
        assert!(!response.price_protect);
        assert_eq!(response.price_match, Some(PriceMatch::None));
        assert_eq!(
            response.self_trade_prevention_mode,
            Some(SelfTradePreventionMode::None)
        );
    }

    #[test]
    fn test_filled_order_response() {
        let json = r#"{
            "clientOrderId": "market_order_456",
            "cumQty": "5.0",
            "cumBase": "0.00111",
            "executedQty": "5.0",
            "orderId": 987654321,
            "avgPrice": "45012.5",
            "origQty": "5.0",
            "price": "0",
            "reduceOnly": true,
            "side": "SELL",
            "positionSide": "SHORT",
            "status": "FILLED",
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "type": "MARKET",
            "origType": "MARKET",
            "updateTime": 1625097700000,
            "workingType": "CONTRACT_PRICE",
            "priceProtect": true
        }"#;

        let response: NewOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 987654321);
        assert_eq!(response.status, "FILLED");
        assert_eq!(response.executed_qty, "5.0");
        assert_eq!(response.avg_price, "45012.5");
        assert!(response.reduce_only);
        assert_eq!(response.side, OrderSide::Sell);
        assert_eq!(response.order_type, OrderType::Market);
    }
}
