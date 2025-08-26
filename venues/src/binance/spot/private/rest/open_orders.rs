use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::PrivateRestClient as RestClient;
use crate::binance::spot::{
    OrderSide, OrderStatus, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

const GET_OPEN_ORDERS_ENDPOINT: &str = "/api/v3/openOrders";

/// Request parameters for getting open orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct OpenOrdersRequest {
    /// Trading pair symbol (optional - if not provided, returns all open orders)
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Open order information
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrder {
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

    /// Order creation time
    #[serde(rename = "time")]
    pub time: u64,

    /// Last update time
    #[serde(rename = "updateTime")]
    pub update_time: u64,

    /// Is working (true if the order is active)
    #[serde(rename = "isWorking")]
    pub is_working: bool,

    /// Working time
    #[serde(rename = "workingTime")]
    pub working_time: u64,

    /// Original quote order quantity
    #[serde(rename = "origQuoteOrderQty")]
    pub orig_quote_order_qty: Decimal,

    /// Self-trade prevention mode
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

impl RestClient {
    /// Get all open orders on a symbol or all symbols
    ///
    /// Get all open orders on a symbol. Careful when accessing this with no symbol.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#current-open-orders--user_data)
    ///
    /// Method: GET /api/v3/openOrders
    /// Weight: 6 (for one symbol), 80 (for all symbols)
    /// Security: USER_DATA
    pub async fn get_open_orders(
        &self,
        params: Option<OpenOrdersRequest>,
    ) -> RestResult<Vec<OpenOrder>> {
        let request = params.unwrap_or_default();
        let weight = if request.symbol.is_some() { 6 } else { 80 };

        self.send_get_signed_request(GET_OPEN_ORDERS_ENDPOINT, request, weight, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_open_orders_request_serialization_minimal() {
        let request = OpenOrdersRequest {
            symbol: None,
            recv_window: None,
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(query_string, "");
    }

    #[test]
    fn test_open_orders_request_serialization_with_symbol() {
        let request = OpenOrdersRequest {
            symbol: Some("BTCUSDT".to_string()),
            recv_window: None,
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(query_string, "symbol=BTCUSDT");
    }

    #[test]
    fn test_open_orders_request_serialization_with_recv_window() {
        let request = OpenOrdersRequest {
            symbol: None,
            recv_window: Some(5000),
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(query_string, "recvWindow=5000");
    }

    #[test]
    fn test_open_orders_request_serialization_full() {
        let request = OpenOrdersRequest {
            symbol: Some("ETHUSDT".to_string()),
            recv_window: Some(10000),
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert!(query_string.contains("symbol=ETHUSDT"));
        assert!(query_string.contains("recvWindow=10000"));
    }

    #[test]
    fn test_open_order_deserialization_minimal() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 28457,
            "orderListId": -1,
            "clientOrderId": "ABC123",
            "price": "31000.00000000",
            "origQty": "0.10000000",
            "executedQty": "0.05000000",
            "cummulativeQuoteQty": "1550.00000000",
            "status": "PARTIALLY_FILLED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "time": 1625184000000,
            "updateTime": 1625184100000,
            "isWorking": true,
            "workingTime": 1625184000000,
            "origQuoteOrderQty": "3100.00000000",
            "selfTradePreventionMode": "NONE"
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, 28457);
        assert_eq!(order.order_list_id, -1);
        assert_eq!(order.client_order_id, "ABC123");
        assert_eq!(order.price, dec!(31000.00000000));
        assert_eq!(order.orig_qty, dec!(0.10000000));
        assert_eq!(order.executed_qty, dec!(0.05000000));
        assert_eq!(order.cummulative_quote_qty, dec!(1550.00000000));
        assert!(matches!(order.status, OrderStatus::PartiallyFilled));
        assert!(matches!(order.time_in_force, TimeInForce::GTC));
        assert!(matches!(order.order_type, OrderType::Limit));
        assert!(matches!(order.side, OrderSide::Buy));
        assert_eq!(order.time, 1625184000000);
        assert_eq!(order.update_time, 1625184100000);
        assert!(order.is_working);
        assert_eq!(order.working_time, 1625184000000);
        assert_eq!(order.orig_quote_order_qty, dec!(3100.00000000));
        assert!(matches!(
            order.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        ));
        assert!(order.stop_price.is_none());
        assert!(order.iceberg_qty.is_none());
    }

    #[test]
    fn test_open_order_deserialization_with_stop_price() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "orderId": 12345,
            "orderListId": -1,
            "clientOrderId": "DEF456",
            "price": "2000.00000000",
            "origQty": "1.00000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "STOP_LOSS_LIMIT",
            "side": "SELL",
            "stopPrice": "1950.00000000",
            "time": 1625184000000,
            "updateTime": 1625184000000,
            "isWorking": false,
            "workingTime": 0,
            "origQuoteOrderQty": "2000.00000000",
            "selfTradePreventionMode": "EXPIRE_TAKER"
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "ETHUSDT");
        assert_eq!(order.order_id, 12345);
        assert!(matches!(order.order_type, OrderType::StopLossLimit));
        assert!(matches!(order.side, OrderSide::Sell));
        assert_eq!(order.stop_price, Some(dec!(1950.00000000)));
        assert!(!order.is_working);
        assert_eq!(order.working_time, 0);
        assert!(matches!(
            order.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        ));
    }

    #[test]
    fn test_open_order_deserialization_with_iceberg() {
        let json = r#"{
            "symbol": "BNBUSDT",
            "orderId": 54321,
            "orderListId": -1,
            "clientOrderId": "GHI789",
            "price": "300.00000000",
            "origQty": "10.00000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "icebergQty": "1.00000000",
            "time": 1625270400000,
            "updateTime": 1625270400000,
            "isWorking": true,
            "workingTime": 1625270400000,
            "origQuoteOrderQty": "3000.00000000",
            "selfTradePreventionMode": "EXPIRE_MAKER"
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BNBUSDT");
        assert_eq!(order.iceberg_qty, Some(dec!(1.00000000)));
        assert!(matches!(
            order.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireMaker
        ));
    }

    #[test]
    fn test_open_order_array_deserialization_empty() {
        let json = "[]";
        let orders: Vec<OpenOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 0);
    }

    #[test]
    fn test_open_order_array_deserialization_multiple() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "orderId": 1,
                "orderListId": -1,
                "clientOrderId": "order1",
                "price": "30000.00000000",
                "origQty": "0.10000000",
                "executedQty": "0.00000000",
                "cummulativeQuoteQty": "0.00000000",
                "status": "NEW",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "BUY",
                "time": 1625184000000,
                "updateTime": 1625184000000,
                "isWorking": true,
                "workingTime": 1625184000000,
                "origQuoteOrderQty": "3000.00000000",
                "selfTradePreventionMode": "NONE"
            },
            {
                "symbol": "ETHUSDT",
                "orderId": 2,
                "orderListId": -1,
                "clientOrderId": "order2",
                "price": "2000.00000000",
                "origQty": "1.00000000",
                "executedQty": "0.50000000",
                "cummulativeQuoteQty": "1000.00000000",
                "status": "PARTIALLY_FILLED",
                "timeInForce": "IOC",
                "type": "LIMIT",
                "side": "SELL",
                "time": 1625270400000,
                "updateTime": 1625270500000,
                "isWorking": true,
                "workingTime": 1625270400000,
                "origQuoteOrderQty": "2000.00000000",
                "selfTradePreventionMode": "EXPIRE_BOTH"
            }
        ]"#;

        let orders: Vec<OpenOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 2);

        assert_eq!(orders[0].symbol, "BTCUSDT");
        assert!(matches!(orders[0].status, OrderStatus::New));
        assert!(matches!(orders[0].time_in_force, TimeInForce::GTC));
        assert_eq!(orders[0].executed_qty, dec!(0.00000000));

        assert_eq!(orders[1].symbol, "ETHUSDT");
        assert!(matches!(orders[1].status, OrderStatus::PartiallyFilled));
        assert!(matches!(orders[1].time_in_force, TimeInForce::IOC));
        assert_eq!(orders[1].executed_qty, dec!(0.50000000));
        assert!(matches!(
            orders[1].self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        ));
    }

    #[test]
    fn test_open_order_deserialization_different_order_types() {
        // Test MARKET order (though unlikely to be open)
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 100,
            "orderListId": -1,
            "clientOrderId": "market_order",
            "price": "0.00000000",
            "origQty": "0.10000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "NEW",
            "timeInForce": "IOC",
            "type": "MARKET",
            "side": "BUY",
            "time": 1625184000000,
            "updateTime": 1625184000000,
            "isWorking": true,
            "workingTime": 1625184000000,
            "origQuoteOrderQty": "0.00000000",
            "selfTradePreventionMode": "NONE"
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert!(matches!(order.order_type, OrderType::Market));
        assert_eq!(order.price, dec!(0));

        // Test STOP_LOSS order
        let json = r#"{
            "symbol": "ETHUSDT",
            "orderId": 101,
            "orderListId": -1,
            "clientOrderId": "stop_loss_order",
            "price": "0.00000000",
            "origQty": "1.00000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "STOP_LOSS",
            "side": "SELL",
            "stopPrice": "1900.00000000",
            "time": 1625184000000,
            "updateTime": 1625184000000,
            "isWorking": false,
            "workingTime": 0,
            "origQuoteOrderQty": "0.00000000",
            "selfTradePreventionMode": "NONE"
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert!(matches!(order.order_type, OrderType::StopLoss));
        assert_eq!(order.stop_price, Some(dec!(1900.00000000)));
    }

    #[test]
    fn test_open_order_deserialization_all_fields_populated() {
        let json = r#"{
            "symbol": "LTCUSDT",
            "orderId": 999999,
            "orderListId": 12345,
            "clientOrderId": "full_order_test",
            "price": "100.50000000",
            "origQty": "25.00000000",
            "executedQty": "10.00000000",
            "cummulativeQuoteQty": "1005.00000000",
            "status": "PARTIALLY_FILLED",
            "timeInForce": "FOK",
            "type": "LIMIT_MAKER",
            "side": "SELL",
            "stopPrice": "99.00000000",
            "icebergQty": "5.00000000",
            "time": 1625356800000,
            "updateTime": 1625360400000,
            "isWorking": true,
            "workingTime": 1625356800000,
            "origQuoteOrderQty": "2512.50000000",
            "selfTradePreventionMode": "EXPIRE_BOTH"
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "LTCUSDT");
        assert_eq!(order.order_id, 999999);
        assert_eq!(order.order_list_id, 12345);
        assert_eq!(order.client_order_id, "full_order_test");
        assert_eq!(order.price, dec!(100.50000000));
        assert_eq!(order.orig_qty, dec!(25.00000000));
        assert_eq!(order.executed_qty, dec!(10.00000000));
        assert_eq!(order.cummulative_quote_qty, dec!(1005.00000000));
        assert!(matches!(order.status, OrderStatus::PartiallyFilled));
        assert!(matches!(order.time_in_force, TimeInForce::FOK));
        assert!(matches!(order.order_type, OrderType::LimitMaker));
        assert!(matches!(order.side, OrderSide::Sell));
        assert_eq!(order.stop_price, Some(dec!(99.00000000)));
        assert_eq!(order.iceberg_qty, Some(dec!(5.00000000)));
        assert_eq!(order.time, 1625356800000);
        assert_eq!(order.update_time, 1625360400000);
        assert!(order.is_working);
        assert_eq!(order.working_time, 1625356800000);
        assert_eq!(order.orig_quote_order_qty, dec!(2512.50000000));
        assert!(matches!(
            order.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        ));
    }

    #[test]
    fn test_open_order_deserialization_different_statuses() {
        let statuses = vec![
            ("NEW", OrderStatus::New),
            ("PARTIALLY_FILLED", OrderStatus::PartiallyFilled),
            ("FILLED", OrderStatus::Filled),
            ("CANCELED", OrderStatus::Canceled),
            ("PENDING_CANCEL", OrderStatus::PendingCancel),
            ("REJECTED", OrderStatus::Rejected),
            ("EXPIRED", OrderStatus::Expired),
            ("EXPIRED_IN_MATCH", OrderStatus::ExpiredInMatch),
        ];

        for (status_str, _expected_status) in statuses {
            let json = format!(
                r#"{{
                    "symbol": "BTCUSDT",
                    "orderId": 1,
                    "orderListId": -1,
                    "clientOrderId": "test",
                    "price": "1.0",
                    "origQty": "1.0",
                    "executedQty": "0.0",
                    "cummulativeQuoteQty": "0.0",
                    "status": "{}",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1625184000000,
                    "updateTime": 1625184000000,
                    "isWorking": true,
                    "workingTime": 1625184000000,
                    "origQuoteOrderQty": "1.0",
                    "selfTradePreventionMode": "NONE"
                }}"#,
                status_str
            );

            let order: OpenOrder = serde_json::from_str(&json).unwrap();
            match (status_str, &order.status) {
                ("NEW", OrderStatus::New) => (),
                ("PARTIALLY_FILLED", OrderStatus::PartiallyFilled) => (),
                ("FILLED", OrderStatus::Filled) => (),
                ("CANCELED", OrderStatus::Canceled) => (),
                ("PENDING_CANCEL", OrderStatus::PendingCancel) => (),
                ("REJECTED", OrderStatus::Rejected) => (),
                ("EXPIRED", OrderStatus::Expired) => (),
                ("EXPIRED_IN_MATCH", OrderStatus::ExpiredInMatch) => (),
                _ => panic!(
                    "Unexpected status mapping: {} -> {:?}",
                    status_str, order.status
                ),
            }
        }
    }

    #[test]
    fn test_open_order_deserialization_decimal_precision() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 1,
            "orderListId": -1,
            "clientOrderId": "precision_test",
            "price": "12345.12345678",
            "origQty": "0.00000001",
            "executedQty": "999999.99999999",
            "cummulativeQuoteQty": "0.10000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "stopPrice": "12344.87654321",
            "icebergQty": "0.00001000",
            "time": 1625184000000,
            "updateTime": 1625184000000,
            "isWorking": true,
            "workingTime": 1625184000000,
            "origQuoteOrderQty": "123.45678901",
            "selfTradePreventionMode": "NONE"
        }"#;

        let order: OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.price, dec!(12345.12345678));
        assert_eq!(order.orig_qty, dec!(0.00000001));
        assert_eq!(order.executed_qty, dec!(999999.99999999));
        assert_eq!(order.cummulative_quote_qty, dec!(0.10000000));
        assert_eq!(order.stop_price, Some(dec!(12344.87654321)));
        assert_eq!(order.iceberg_qty, Some(dec!(0.00001000)));
        assert_eq!(order.orig_quote_order_qty, dec!(123.45678901));
    }

    #[test]
    fn test_open_orders_request_default() {
        let request = OpenOrdersRequest::default();
        assert!(request.symbol.is_none());
        assert!(request.recv_window.is_none());
    }
}
