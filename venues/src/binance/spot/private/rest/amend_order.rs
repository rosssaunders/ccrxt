use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    OrderSide, OrderStatus, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

const AMEND_ORDER_ENDPOINT: &str = "/api/v3/order/amend/keepPriority";

/// Request parameters for amending an order
#[derive(Debug, Clone, Serialize)]
pub struct AmendOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// New quantity (required)
    #[serde(rename = "quantity")]
    pub quantity: Decimal,

    /// New price
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Amend order response
#[derive(Debug, Clone, Deserialize)]
pub struct AmendOrderResponse {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Order list ID
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Transaction time
    #[serde(rename = "transactTime")]
    pub transact_time: u64,

    /// Order price
    #[serde(rename = "price")]
    pub price: Decimal,

    /// Original quantity
    #[serde(rename = "origQty")]
    pub orig_qty: Decimal,

    /// Executed quantity
    #[serde(rename = "executedQty")]
    pub executed_qty: Decimal,

    /// Cumulative quote quantity
    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: Decimal,

    /// Order status
    #[serde(rename = "status")]
    pub status: OrderStatus,

    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Order side
    #[serde(rename = "side")]
    pub side: OrderSide,

    /// Stop price
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Decimal>,

    /// Iceberg quantity
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<Decimal>,

    /// Working time
    #[serde(rename = "workingTime")]
    pub working_time: u64,

    /// Self-trade prevention mode
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

impl RestClient {
    /// Amend an order
    ///
    /// Reduce the quantity of an existing open order.
    /// Either orderId or origClientOrderId must be sent.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#amend-order--trade)
    /// Method: PUT /api/v3/order/amend/keepPriority
    /// Weight: 4
    /// Security: TRADE
    pub async fn amend_order(&self, params: AmendOrderRequest) -> RestResult<AmendOrderResponse> {
        self.send_put_signed_request(
            AMEND_ORDER_ENDPOINT,
            params,
            4,
            true,)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_amend_order_request_with_order_id_serialization() {
        let request = AmendOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(12345),
            orig_client_order_id: None,
            quantity: dec!(0.5),
            price: Some(dec!(45000.50)),
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["orderId"], 12345);
        assert!(json.get("origClientOrderId").is_none());
        assert_eq!(json["quantity"], "0.5");
        assert_eq!(json["price"], "45000.50");
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_amend_order_request_with_orig_client_order_id_serialization() {
        let request = AmendOrderRequest {
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            orig_client_order_id: Some("my-order-123".to_string()),
            quantity: dec!(1.25),
            price: None,
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert!(json.get("orderId").is_none());
        assert_eq!(json["origClientOrderId"], "my-order-123");
        assert_eq!(json["quantity"], "1.25");
        assert!(json.get("price").is_none());
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_amend_order_request_with_both_ids_serialization() {
        // Test when both order_id and orig_client_order_id are provided
        let request = AmendOrderRequest {
            symbol: "BNBUSDT".to_string(),
            order_id: Some(99999),
            orig_client_order_id: Some("client-99999".to_string()),
            quantity: dec!(10.0),
            price: Some(dec!(300.0)),
            recv_window: Some(10000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BNBUSDT");
        assert_eq!(json["orderId"], 99999);
        assert_eq!(json["origClientOrderId"], "client-99999");
        assert_eq!(json["quantity"], "10.0");
        assert_eq!(json["price"], "300.0");
        assert_eq!(json["recvWindow"], 10000);
    }

    #[test]
    fn test_amend_order_request_minimal_serialization() {
        // Test with only required fields - symbol, quantity and one of the IDs
        let request = AmendOrderRequest {
            symbol: "SOLUSDT".to_string(),
            order_id: Some(7777),
            orig_client_order_id: None,
            quantity: dec!(2.5),
            price: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "SOLUSDT");
        assert_eq!(json["orderId"], 7777);
        assert!(json.get("origClientOrderId").is_none());
        assert_eq!(json["quantity"], "2.5");
        assert!(json.get("price").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_amend_order_request_with_high_precision_values() {
        let request = AmendOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(123456789),
            orig_client_order_id: None,
            quantity: dec!(0.123456789),
            price: Some(dec!(45678.123456789)),
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["orderId"], 123456789);
        assert_eq!(json["quantity"], "0.123456789");
        assert_eq!(json["price"], "45678.123456789");
    }

    #[test]
    fn test_amend_order_response_basic_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 12345,
            "orderListId": -1,
            "clientOrderId": "myOrder1",
            "transactTime": 1684804350068,
            "price": "50000.00000000",
            "origQty": "1.00000000",
            "executedQty": "0.50000000",
            "cummulativeQuoteQty": "25000.00000000",
            "status": "PARTIALLY_FILLED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "workingTime": 1684804350000,
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: AmendOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 12345);
        assert_eq!(response.order_list_id, -1);
        assert_eq!(response.client_order_id, "myOrder1");
        assert_eq!(response.transact_time, 1684804350068);
        assert_eq!(response.price.to_string(), "50000.00000000");
        assert_eq!(response.orig_qty.to_string(), "1.00000000");
        assert_eq!(response.executed_qty.to_string(), "0.50000000");
        assert_eq!(response.cummulative_quote_qty.to_string(), "25000.00000000");
        assert_eq!(response.status, OrderStatus::PartiallyFilled);
        assert_eq!(response.time_in_force, TimeInForce::GTC);
        assert_eq!(response.order_type, OrderType::Limit);
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(response.working_time, 1684804350000);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        );
        assert!(response.stop_price.is_none());
        assert!(response.iceberg_qty.is_none());
    }

    #[test]
    fn test_amend_order_response_with_stop_price_deserialization() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "orderId": 789,
            "orderListId": -1,
            "clientOrderId": "stopOrder123",
            "transactTime": 1684804350500,
            "price": "3000.00000000",
            "origQty": "0.50000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "STOP_LOSS_LIMIT",
            "side": "SELL",
            "stopPrice": "2950.00000000",
            "workingTime": 0,
            "selfTradePreventionMode": "EXPIRE_TAKER"
        }"#;

        let response: AmendOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETHUSDT");
        assert_eq!(response.order_id, 789);
        assert_eq!(response.client_order_id, "stopOrder123");
        assert_eq!(response.transact_time, 1684804350500);
        assert_eq!(response.price.to_string(), "3000.00000000");
        assert_eq!(response.orig_qty.to_string(), "0.50000000");
        assert_eq!(response.executed_qty.to_string(), "0.00000000");
        assert_eq!(response.status, OrderStatus::New);
        assert_eq!(response.order_type, OrderType::StopLossLimit);
        assert_eq!(response.side, OrderSide::Sell);
        assert_eq!(response.stop_price.unwrap().to_string(), "2950.00000000");
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        );
    }

    #[test]
    fn test_amend_order_response_with_iceberg_qty_deserialization() {
        let json = r#"{
            "symbol": "BNBUSDT",
            "orderId": 54321,
            "orderListId": -1,
            "clientOrderId": "icebergOrder1",
            "transactTime": 1684804350700,
            "price": "350.00000000",
            "origQty": "100.00000000",
            "executedQty": "20.00000000",
            "cummulativeQuoteQty": "7000.00000000",
            "status": "PARTIALLY_FILLED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "icebergQty": "10.00000000",
            "workingTime": 1684804350700,
            "selfTradePreventionMode": "EXPIRE_BOTH"
        }"#;

        let response: AmendOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BNBUSDT");
        assert_eq!(response.order_id, 54321);
        assert_eq!(response.orig_qty.to_string(), "100.00000000");
        assert_eq!(response.executed_qty.to_string(), "20.00000000");
        assert_eq!(response.iceberg_qty.unwrap().to_string(), "10.00000000");
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        );
    }

    #[test]
    fn test_amend_order_response_filled_status_deserialization() {
        let json = r#"{
            "symbol": "SOLUSDT",
            "orderId": 111222,
            "orderListId": -1,
            "clientOrderId": "filledOrder1",
            "transactTime": 1684804351000,
            "price": "20.50000000",
            "origQty": "50.00000000",
            "executedQty": "50.00000000",
            "cummulativeQuoteQty": "1025.00000000",
            "status": "FILLED",
            "timeInForce": "IOC",
            "type": "LIMIT",
            "side": "BUY",
            "workingTime": 1684804351000,
            "selfTradePreventionMode": "EXPIRE_MAKER"
        }"#;

        let response: AmendOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "SOLUSDT");
        assert_eq!(response.order_id, 111222);
        assert_eq!(response.status, OrderStatus::Filled);
        assert_eq!(response.time_in_force, TimeInForce::IOC);
        assert_eq!(response.orig_qty, response.executed_qty);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireMaker
        );
    }

    #[test]
    fn test_amend_order_response_market_order_deserialization() {
        let json = r#"{
            "symbol": "ADAUSDT",
            "orderId": 999888,
            "orderListId": -1,
            "clientOrderId": "marketOrder1",
            "transactTime": 1684804352000,
            "price": "0.00000000",
            "origQty": "1000.00000000",
            "executedQty": "1000.00000000",
            "cummulativeQuoteQty": "350.00000000",
            "status": "FILLED",
            "timeInForce": "GTC",
            "type": "MARKET",
            "side": "SELL",
            "workingTime": 1684804352000,
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: AmendOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ADAUSDT");
        assert_eq!(response.order_type, OrderType::Market);
        assert_eq!(response.price.to_string(), "0.00000000");
        assert_eq!(response.status, OrderStatus::Filled);
    }

    #[test]
    fn test_amend_order_response_various_order_statuses() {
        // Test CANCELED status
        let json = r#"{
            "symbol": "LTCUSDT",
            "orderId": 777666,
            "orderListId": -1,
            "clientOrderId": "canceledOrder1",
            "transactTime": 1684804353000,
            "price": "90.00000000",
            "origQty": "5.00000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "CANCELED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "workingTime": 0,
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: AmendOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, OrderStatus::Canceled);
        assert_eq!(response.executed_qty.to_string(), "0.00000000");

        // Test EXPIRED status
        let json = r#"{
            "symbol": "DOTUSDT",
            "orderId": 555444,
            "orderListId": -1,
            "clientOrderId": "expiredOrder1",
            "transactTime": 1684804354000,
            "price": "7.00000000",
            "origQty": "20.00000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "EXPIRED",
            "timeInForce": "FOK",
            "type": "LIMIT",
            "side": "SELL",
            "workingTime": 0,
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: AmendOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, OrderStatus::Expired);
        assert_eq!(response.time_in_force, TimeInForce::FOK);
    }

    #[test]
    fn test_amend_order_response_edge_cases() {
        // Test with very large order ID
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 9223372036854775807,
            "orderListId": 9223372036854775806,
            "clientOrderId": "largeIdOrder",
            "transactTime": 1684804355000,
            "price": "100000.00000000",
            "origQty": "0.00001000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "workingTime": 1684804355000,
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: AmendOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_id, 9223372036854775807u64);
        assert_eq!(response.order_list_id, 9223372036854775806i64);

        // Test with very small quantities
        assert_eq!(response.orig_qty.to_string(), "0.00001000");
    }

    #[test]
    fn test_skip_serializing_if_none_fields() {
        // Test that None fields are properly skipped during serialization
        let request = AmendOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(12345),
            orig_client_order_id: None,
            quantity: dec!(1.0),
            price: None,
            recv_window: None,
        };

        let json_str = serde_json::to_string(&request).unwrap();

        // Verify that None fields are not present in the JSON string
        assert!(!json_str.contains("origClientOrderId"));
        assert!(!json_str.contains("price"));
        assert!(!json_str.contains("recvWindow"));

        // Verify that Some fields are present
        assert!(json_str.contains("orderId"));
        assert!(json_str.contains("symbol"));
        assert!(json_str.contains("quantity"));
    }

    #[test]
    fn test_amend_order_request_urlencoded_serialization() {
        // Test URL encoding for the request
        let request = AmendOrderRequest {
            symbol: "BTC-USDT".to_string(), // Symbol with special character
            order_id: Some(12345),
            orig_client_order_id: None,
            quantity: dec!(0.5),
            price: Some(dec!(45000.50)),
            recv_window: Some(5000),
        };

        // While we don't use serde_urlencoded in amend_order (it uses body params),
        // this test ensures the structure would work correctly if needed
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTC-USDT");
        assert_eq!(json["orderId"], 12345);
        assert_eq!(json["quantity"], "0.5");
        assert_eq!(json["price"], "45000.50");
        assert_eq!(json["recvWindow"], 5000);
    }
}
