use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    OrderSide, OrderStatus, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

const GET_ALL_ORDERS_ENDPOINT: &str = "/api/v3/allOrders";

/// Request parameters for getting all orders
#[derive(Debug, Clone, Serialize)]
pub struct AllOrdersRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Start time timestamp in ms
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,

    /// End time timestamp in ms
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,

    /// Default 500; max 1000
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// All order information (same as OpenOrder structure)
#[derive(Debug, Clone, Deserialize)]
pub struct AllOrder {
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
    /// Get all account orders (active, canceled, or filled)
    ///
    /// Get all account orders; active, canceled, or filled.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#all-orders--user_data)
    ///
    /// Method: GET /api/v3/allOrders
    /// Weight: 20
    /// Security: USER_DATA
    pub async fn get_all_orders(&self, params: AllOrdersRequest) -> RestResult<Vec<AllOrder>> {
        self.send_get_signed_request(GET_ALL_ORDERS_ENDPOINT, params, 20, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_all_orders_request_serialization_minimal() {
        let request = AllOrdersRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(query_string, "symbol=BTCUSDT");
    }

    #[test]
    fn test_all_orders_request_serialization_with_order_id() {
        let request = AllOrdersRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(12345),
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert!(query_string.contains("symbol=BTCUSDT"));
        assert!(query_string.contains("orderId=12345"));
    }

    #[test]
    fn test_all_orders_request_serialization_with_time_range() {
        let request = AllOrdersRequest {
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            limit: None,
            recv_window: None,
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert!(query_string.contains("symbol=ETHUSDT"));
        assert!(query_string.contains("startTime=1625184000000"));
        assert!(query_string.contains("endTime=1625270400000"));
    }

    #[test]
    fn test_all_orders_request_serialization_full() {
        let request = AllOrdersRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(99999),
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            limit: Some(500),
            recv_window: Some(5000),
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert!(query_string.contains("symbol=BTCUSDT"));
        assert!(query_string.contains("orderId=99999"));
        assert!(query_string.contains("startTime=1625184000000"));
        assert!(query_string.contains("endTime=1625270400000"));
        assert!(query_string.contains("limit=500"));
        assert!(query_string.contains("recvWindow=5000"));
    }

    #[test]
    fn test_all_order_deserialization_minimal() {
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

        let order: AllOrder = serde_json::from_str(json).unwrap();
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
    fn test_all_order_deserialization_with_stop_price() {
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

        let order: AllOrder = serde_json::from_str(json).unwrap();
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
    fn test_all_order_deserialization_with_iceberg() {
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

        let order: AllOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BNBUSDT");
        assert_eq!(order.iceberg_qty, Some(dec!(1.00000000)));
        assert!(matches!(
            order.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireMaker
        ));
    }

    #[test]
    fn test_all_order_array_deserialization_empty() {
        let json = "[]";
        let orders: Vec<AllOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 0);
    }

    #[test]
    fn test_all_order_array_deserialization_multiple() {
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "orderId": 1,
                "orderListId": -1,
                "clientOrderId": "order1",
                "price": "30000.00000000",
                "origQty": "0.10000000",
                "executedQty": "0.10000000",
                "cummulativeQuoteQty": "3000.00000000",
                "status": "FILLED",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "BUY",
                "time": 1625184000000,
                "updateTime": 1625184100000,
                "isWorking": false,
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
                "executedQty": "0.00000000",
                "cummulativeQuoteQty": "0.00000000",
                "status": "CANCELED",
                "timeInForce": "IOC",
                "type": "LIMIT",
                "side": "SELL",
                "time": 1625270400000,
                "updateTime": 1625270500000,
                "isWorking": false,
                "workingTime": 0,
                "origQuoteOrderQty": "2000.00000000",
                "selfTradePreventionMode": "EXPIRE_BOTH"
            }
        ]"#;

        let orders: Vec<AllOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(orders.len(), 2);

        assert_eq!(orders[0].symbol, "BTCUSDT");
        assert!(matches!(orders[0].status, OrderStatus::Filled));
        assert!(matches!(orders[0].time_in_force, TimeInForce::GTC));

        assert_eq!(orders[1].symbol, "ETHUSDT");
        assert!(matches!(orders[1].status, OrderStatus::Canceled));
        assert!(matches!(orders[1].time_in_force, TimeInForce::IOC));
        assert!(matches!(
            orders[1].self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        ));
    }

    #[test]
    fn test_all_order_deserialization_different_order_types() {
        // Test MARKET order
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 100,
            "orderListId": -1,
            "clientOrderId": "market_order",
            "price": "0.00000000",
            "origQty": "0.10000000",
            "executedQty": "0.10000000",
            "cummulativeQuoteQty": "3150.50000000",
            "status": "FILLED",
            "timeInForce": "IOC",
            "type": "MARKET",
            "side": "BUY",
            "time": 1625184000000,
            "updateTime": 1625184000100,
            "isWorking": false,
            "workingTime": 0,
            "origQuoteOrderQty": "0.00000000",
            "selfTradePreventionMode": "NONE"
        }"#;

        let order: AllOrder = serde_json::from_str(json).unwrap();
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

        let order: AllOrder = serde_json::from_str(json).unwrap();
        assert!(matches!(order.order_type, OrderType::StopLoss));
        assert_eq!(order.stop_price, Some(dec!(1900.00000000)));
    }

    #[test]
    fn test_all_order_deserialization_different_statuses() {
        let statuses = vec![
            ("NEW", OrderStatus::New),
            ("PARTIALLY_FILLED", OrderStatus::PartiallyFilled),
            ("FILLED", OrderStatus::Filled),
            ("CANCELED", OrderStatus::Canceled),
            ("REJECTED", OrderStatus::Rejected),
            ("EXPIRED", OrderStatus::Expired),
            ("EXPIRED_IN_MATCH", OrderStatus::ExpiredInMatch),
        ];

        for (status_str, expected_status) in statuses {
            let json = format!(
                r#"{{
                    "symbol": "BTCUSDT",
                    "orderId": 1000,
                    "orderListId": -1,
                    "clientOrderId": "test_status",
                    "price": "30000.00000000",
                    "origQty": "0.10000000",
                    "executedQty": "0.05000000",
                    "cummulativeQuoteQty": "1500.00000000",
                    "status": "{}",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1625184000000,
                    "updateTime": 1625184100000,
                    "isWorking": true,
                    "workingTime": 1625184000000,
                    "origQuoteOrderQty": "3000.00000000",
                    "selfTradePreventionMode": "NONE"
                }}"#,
                status_str
            );

            let order: AllOrder = serde_json::from_str(&json).unwrap();
            match expected_status {
                OrderStatus::New => assert!(matches!(order.status, OrderStatus::New)),
                OrderStatus::PartiallyFilled => {
                    assert!(matches!(order.status, OrderStatus::PartiallyFilled))
                }
                OrderStatus::Filled => assert!(matches!(order.status, OrderStatus::Filled)),
                OrderStatus::Canceled => assert!(matches!(order.status, OrderStatus::Canceled)),
                OrderStatus::Rejected => assert!(matches!(order.status, OrderStatus::Rejected)),
                OrderStatus::Expired => assert!(matches!(order.status, OrderStatus::Expired)),
                OrderStatus::ExpiredInMatch => {
                    assert!(matches!(order.status, OrderStatus::ExpiredInMatch))
                }
                _ => panic!("Unexpected status"),
            }
        }
    }

    #[test]
    fn test_all_order_deserialization_different_time_in_force() {
        let tifs = vec![
            ("GTC", TimeInForce::GTC),
            ("IOC", TimeInForce::IOC),
            ("FOK", TimeInForce::FOK),
        ];

        for (tif_str, expected_tif) in tifs {
            let json = format!(
                r#"{{
                    "symbol": "BTCUSDT",
                    "orderId": 2000,
                    "orderListId": -1,
                    "clientOrderId": "test_tif",
                    "price": "30000.00000000",
                    "origQty": "0.10000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "NEW",
                    "timeInForce": "{}",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1625184000000,
                    "updateTime": 1625184000000,
                    "isWorking": true,
                    "workingTime": 1625184000000,
                    "origQuoteOrderQty": "3000.00000000",
                    "selfTradePreventionMode": "NONE"
                }}"#,
                tif_str
            );

            let order: AllOrder = serde_json::from_str(&json).unwrap();
            match expected_tif {
                TimeInForce::GTC => assert!(matches!(order.time_in_force, TimeInForce::GTC)),
                TimeInForce::IOC => assert!(matches!(order.time_in_force, TimeInForce::IOC)),
                TimeInForce::FOK => assert!(matches!(order.time_in_force, TimeInForce::FOK)),
            }
        }
    }
}
