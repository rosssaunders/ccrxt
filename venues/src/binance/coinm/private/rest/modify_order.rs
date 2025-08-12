use serde::{Deserialize, Serialize};

use crate::binance::coinm::{
    OrderSide, OrderStatus, OrderType, PositionSide, PriceMatch, RestResult,
    SelfTradePreventionMode, TimeInForce, WorkingType, private::rest::client::RestClient,
};

const ORDER_ENDPOINT: &str = "/dapi/v1/order";

/// Request parameters for modifying an existing order (PUT /dapi/v1/order).
#[derive(Debug, Clone, Serialize)]
pub struct ModifyOrderRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP").
    pub symbol: String,

    /// Order side (BUY or SELL).
    pub side: OrderSide,

    /// Order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID. Either orderId or origClientOrderId must be sent.
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// New order quantity. Either quantity or price must be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,

    /// New order price. Either quantity or price must be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Price match mode. Only available for LIMIT/STOP/TAKE_PROFIT orders.
    /// Cannot be passed together with price.
    #[serde(rename = "priceMatch", skip_serializing_if = "Option::is_none")]
    pub price_match: Option<PriceMatch>,

    /// Receive window in milliseconds.
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,

    /// Timestamp in milliseconds.
    pub timestamp: u64,
}

/// Response for modifying an order (PUT /dapi/v1/order).
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyOrderResponse {
    /// Order ID.
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Trading symbol.
    pub symbol: String,

    /// Trading pair.
    pub pair: String,

    /// Order status.
    pub status: OrderStatus,

    /// Client order ID.
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Order price.
    pub price: String,

    /// Average price.
    #[serde(rename = "avgPrice")]
    pub avg_price: String,

    /// Original quantity.
    #[serde(rename = "origQty")]
    pub orig_qty: String,

    /// Executed quantity.
    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    /// Cumulative quantity.
    #[serde(rename = "cumQty")]
    pub cum_qty: String,

    /// Cumulative base quantity.
    #[serde(rename = "cumBase")]
    pub cum_base: String,

    /// Time in force.
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order type.
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Reduce only flag.
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,

    /// Close position flag.
    #[serde(rename = "closePosition")]
    pub close_position: bool,

    /// Order side.
    pub side: OrderSide,

    /// Position side.
    #[serde(rename = "positionSide")]
    pub position_side: PositionSide,

    /// Stop price.
    #[serde(rename = "stopPrice")]
    pub stop_price: String,

    /// Working type.
    #[serde(rename = "workingType")]
    pub working_type: WorkingType,

    /// Price protect flag.
    #[serde(rename = "priceProtect")]
    pub price_protect: bool,

    /// Original order type.
    #[serde(rename = "origType")]
    pub orig_type: OrderType,

    /// Price match mode.
    #[serde(rename = "priceMatch")]
    pub price_match: PriceMatch,

    /// Self-trade prevention mode.
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,

    /// Update time.
    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

impl RestClient {
    /// Modifies an existing order (TRADE) on Binance Coin-M Futures.
    ///
    /// [docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/trade/rest-api/Modify-Order)
    ///
    /// PUT /dapi/v1/order
    /// Weight: 1
    /// Requires API key and signature.
    ///
    /// Currently only LIMIT order modification is supported.
    /// Modified orders will be reordered in the match queue.
    /// Either orderId or origClientOrderId must be sent.
    /// Either quantity or price must be sent.
    ///
    /// # Arguments
    /// * `params` - The request parameters (see [`ModifyOrderRequest`])
    ///
    /// # Returns
    /// A [`ModifyOrderResponse`] object with updated order details.
    pub async fn modify_order(
        &self,
        params: ModifyOrderRequest,
    ) -> RestResult<ModifyOrderResponse> {
        let weight = 1;
        self.send_put_signed_request(ORDER_ENDPOINT, params, weight, true)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_order_request_serialization() {
        let request = ModifyOrderRequest {
            symbol: "BTCUSD_PERP".to_string(),
            side: OrderSide::Buy,
            order_id: Some(12345),
            orig_client_order_id: None,
            quantity: Some(10.5),
            price: Some(45000.0),
            price_match: None,
            recv_window: None,
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=BTCUSD_PERP"));
        assert!(serialized.contains("side=BUY"));
        assert!(serialized.contains("orderId=12345"));
        assert!(serialized.contains("quantity=10.5"));
        assert!(serialized.contains("price=45000.0"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("origClientOrderId"));
        assert!(!serialized.contains("priceMatch"));
        assert!(!serialized.contains("recvWindow"));
    }

    #[test]
    fn test_modify_order_request_with_client_order_id() {
        let request = ModifyOrderRequest {
            symbol: "ETHUSD_PERP".to_string(),
            side: OrderSide::Sell,
            order_id: None,
            orig_client_order_id: Some("my_order_123".to_string()),
            quantity: Some(5.0),
            price: None,
            price_match: Some(PriceMatch::Opponent),
            recv_window: Some(5000),
            timestamp: 1625097600000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("symbol=ETHUSD_PERP"));
        assert!(serialized.contains("side=SELL"));
        assert!(serialized.contains("origClientOrderId=my_order_123"));
        assert!(serialized.contains("quantity=5.0"));
        assert!(serialized.contains("priceMatch=OPPONENT"));
        assert!(serialized.contains("recvWindow=5000"));
        assert!(serialized.contains("timestamp=1625097600000"));
        assert!(!serialized.contains("orderId"));
        assert!(!serialized.contains("price="));
    }

    #[test]
    fn test_modify_order_response_deserialization() {
        let json = r#"{
            "orderId": 12345,
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "status": "NEW",
            "clientOrderId": "my_order_123",
            "price": "45000.0",
            "avgPrice": "0.0",
            "origQty": "10.5",
            "executedQty": "0.0",
            "cumQty": "0.0",
            "cumBase": "0.0",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "reduceOnly": false,
            "closePosition": false,
            "side": "BUY",
            "positionSide": "LONG",
            "stopPrice": "0.0",
            "workingType": "CONTRACT_PRICE",
            "priceProtect": false,
            "origType": "LIMIT",
            "priceMatch": "NONE",
            "selfTradePreventionMode": "NONE",
            "updateTime": 1625097600000
        }"#;

        let response: ModifyOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 12345);
        assert_eq!(response.symbol, "BTCUSD_PERP");
        assert_eq!(response.pair, "BTCUSD");
        assert_eq!(response.status, OrderStatus::New);
        assert_eq!(response.client_order_id, "my_order_123");
        assert_eq!(response.price, "45000.0");
        assert_eq!(response.avg_price, "0.0");
        assert_eq!(response.orig_qty, "10.5");
        assert_eq!(response.executed_qty, "0.0");
        assert_eq!(response.cum_qty, "0.0");
        assert_eq!(response.cum_base, "0.0");
        assert_eq!(response.time_in_force, TimeInForce::GTC);
        assert_eq!(response.order_type, OrderType::Limit);
        assert!(!response.reduce_only);
        assert!(!response.close_position);
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(response.position_side, PositionSide::Long);
        assert_eq!(response.stop_price, "0.0");
        assert_eq!(response.working_type, WorkingType::ContractPrice);
        assert!(!response.price_protect);
        assert_eq!(response.orig_type, OrderType::Limit);
        assert_eq!(response.price_match, PriceMatch::None);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        );
        assert_eq!(response.update_time, 1625097600000);
    }

    #[test]
    fn test_modify_order_response_with_execution() {
        let json = r#"{
            "orderId": 67890,
            "symbol": "ETHUSD_PERP",
            "pair": "ETHUSD",
            "status": "PARTIALLY_FILLED",
            "clientOrderId": "client_456",
            "price": "3000.0",
            "avgPrice": "2999.5",
            "origQty": "20.0",
            "executedQty": "10.0",
            "cumQty": "10.0",
            "cumBase": "0.00333",
            "timeInForce": "IOC",
            "type": "LIMIT",
            "reduceOnly": true,
            "closePosition": false,
            "side": "SELL",
            "positionSide": "SHORT",
            "stopPrice": "0.0",
            "workingType": "CONTRACT_PRICE",
            "priceProtect": true,
            "origType": "LIMIT",
            "priceMatch": "QUEUE",
            "selfTradePreventionMode": "EXPIRE_TAKER",
            "updateTime": 1625097700000
        }"#;

        let response: ModifyOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 67890);
        assert_eq!(response.symbol, "ETHUSD_PERP");
        assert_eq!(response.status, OrderStatus::PartiallyFilled);
        assert_eq!(response.executed_qty, "10.0");
        assert_eq!(response.avg_price, "2999.5");
        assert!(response.reduce_only);
        assert_eq!(response.side, OrderSide::Sell);
        assert_eq!(response.position_side, PositionSide::Short);
        assert!(response.price_protect);
        assert_eq!(response.price_match, PriceMatch::Queue);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        );
    }
}
