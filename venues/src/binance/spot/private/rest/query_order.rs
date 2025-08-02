use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    OrderSide, OrderStatus, OrderType, RestResult, SelfTradePreventionMode, TimeInForce,
};

const GET_ORDER_ENDPOINT: &str = "/api/v3/order";

/// Request parameters for querying an order
#[derive(Debug, Clone, Serialize)]
pub struct QueryOrderRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,

    /// Original client order ID
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Query order response
#[derive(Debug, Clone, Deserialize)]
pub struct QueryOrderResponse {
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
    /// Check an order's status
    ///
    /// Check an order's status.
    /// Either orderId or origClientOrderId must be sent.
    ///
    /// [docs]: (https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-order--user_data)
    /// Method: GET /api/v3/order
    /// Weight: 4
    /// Security: USER_DATA
    pub async fn query_order(&self, params: QueryOrderRequest) -> RestResult<QueryOrderResponse> {
        self.send_get_signed_request(
            GET_ORDER_ENDPOINT,
            params,
            4,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_query_order_request_with_order_id_serialization() {
        let request = QueryOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: Some(12345),
            orig_client_order_id: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["orderId"], 12345);
        assert!(json.get("origClientOrderId").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_query_order_request_with_orig_client_order_id_serialization() {
        let request = QueryOrderRequest {
            symbol: "ETHUSDT".to_string(),
            order_id: None,
            orig_client_order_id: Some("my-order-123".to_string()),
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert!(json.get("orderId").is_none());
        assert_eq!(json["origClientOrderId"], "my-order-123");
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_query_order_request_with_both_ids_serialization() {
        // Test when both order_id and orig_client_order_id are provided
        let request = QueryOrderRequest {
            symbol: "BNBUSDT".to_string(),
            order_id: Some(99999),
            orig_client_order_id: Some("client-99999".to_string()),
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BNBUSDT");
        assert_eq!(json["orderId"], 99999);
        assert_eq!(json["origClientOrderId"], "client-99999");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_query_order_request_with_recv_window_serialization() {
        let request = QueryOrderRequest {
            symbol: "SOLUSDT".to_string(),
            order_id: Some(7777),
            orig_client_order_id: None,
            recv_window: Some(10000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "SOLUSDT");
        assert_eq!(json["orderId"], 7777);
        assert!(json.get("origClientOrderId").is_none());
        assert_eq!(json["recvWindow"], 10000);
    }

    #[test]
    fn test_query_order_request_minimal_serialization() {
        // Test with only required fields - symbol and one of the IDs
        let request = QueryOrderRequest {
            symbol: "BTCUSDT".to_string(),
            order_id: None,
            orig_client_order_id: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert!(json.get("orderId").is_none());
        assert!(json.get("origClientOrderId").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_query_order_response_basic_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 12345,
            "orderListId": -1,
            "clientOrderId": "myOrder1",
            "price": "50000.00000000",
            "origQty": "1.00000000",
            "executedQty": "0.50000000",
            "cummulativeQuoteQty": "25000.00000000",
            "status": "PARTIALLY_FILLED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "time": 1684804350000,
            "updateTime": 1684804350068,
            "isWorking": true,
            "workingTime": 1684804350000,
            "origQuoteOrderQty": "50000.00000000",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.order_id, 12345);
        assert_eq!(response.order_list_id, -1);
        assert_eq!(response.client_order_id, "myOrder1");
        assert_eq!(response.price.to_string(), "50000.00000000");
        assert_eq!(response.orig_qty.to_string(), "1.00000000");
        assert_eq!(response.executed_qty.to_string(), "0.50000000");
        assert_eq!(response.cummulative_quote_qty.to_string(), "25000.00000000");
        assert_eq!(response.status, OrderStatus::PartiallyFilled);
        assert_eq!(response.time_in_force, TimeInForce::GTC);
        assert_eq!(response.order_type, OrderType::Limit);
        assert_eq!(response.side, OrderSide::Buy);
        assert_eq!(response.time, 1684804350000);
        assert_eq!(response.update_time, 1684804350068);
        assert_eq!(response.is_working, true);
        assert_eq!(response.working_time, 1684804350000);
        assert_eq!(response.orig_quote_order_qty.to_string(), "50000.00000000");
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        );
        assert!(response.stop_price.is_none());
        assert!(response.iceberg_qty.is_none());
    }

    #[test]
    fn test_query_order_response_with_stop_price_deserialization() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "orderId": 789,
            "orderListId": -1,
            "clientOrderId": "stopOrder123",
            "price": "3000.00000000",
            "origQty": "0.50000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "STOP_LOSS_LIMIT",
            "side": "SELL",
            "stopPrice": "2950.00000000",
            "time": 1684804350000,
            "updateTime": 1684804350000,
            "isWorking": false,
            "workingTime": 0,
            "origQuoteOrderQty": "1500.00000000",
            "selfTradePreventionMode": "EXPIRE_TAKER"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "ETHUSDT");
        assert_eq!(response.order_type, OrderType::StopLossLimit);
        assert_eq!(response.stop_price.unwrap().to_string(), "2950.00000000");
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        );
        assert_eq!(response.is_working, false);
        assert_eq!(response.working_time, 0);
    }

    #[test]
    fn test_query_order_response_with_iceberg_qty_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 999,
            "orderListId": -1,
            "clientOrderId": "icebergOrder1",
            "price": "50000.00000000",
            "origQty": "10.00000000",
            "executedQty": "2.00000000",
            "cummulativeQuoteQty": "100000.00000000",
            "status": "FILLED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "icebergQty": "1.00000000",
            "time": 1684804350000,
            "updateTime": 1684804360000,
            "isWorking": false,
            "workingTime": 1684804350000,
            "origQuoteOrderQty": "500000.00000000",
            "selfTradePreventionMode": "EXPIRE_BOTH"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.iceberg_qty.unwrap().to_string(), "1.00000000");
        assert_eq!(response.executed_qty.to_string(), "2.00000000");
        assert_eq!(response.status, OrderStatus::Filled);
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        );
    }

    #[test]
    fn test_query_order_response_market_order_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 222,
            "orderListId": -1,
            "clientOrderId": "marketOrder1",
            "price": "0.00000000",
            "origQty": "0.10000000",
            "executedQty": "0.10000000",
            "cummulativeQuoteQty": "5000.00000000",
            "status": "FILLED",
            "timeInForce": "IOC",
            "type": "MARKET",
            "side": "BUY",
            "time": 1684804350000,
            "updateTime": 1684804350001,
            "isWorking": false,
            "workingTime": 1684804350000,
            "origQuoteOrderQty": "5000.00000000",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_type, OrderType::Market);
        assert_eq!(response.time_in_force, TimeInForce::IOC);
        assert_eq!(response.price.to_string(), "0.00000000");
        assert_eq!(response.status, OrderStatus::Filled);
    }

    #[test]
    fn test_query_order_response_canceled_order_deserialization() {
        let json = r#"{
            "symbol": "BNBUSDT",
            "orderId": 111,
            "orderListId": -1,
            "clientOrderId": "canceledOrder1",
            "price": "300.00000000",
            "origQty": "5.00000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "CANCELED",
            "timeInForce": "FOK",
            "type": "LIMIT",
            "side": "SELL",
            "time": 1684804350000,
            "updateTime": 1684804352000,
            "isWorking": false,
            "workingTime": 1684804350000,
            "origQuoteOrderQty": "1500.00000000",
            "selfTradePreventionMode": "EXPIRE_MAKER"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, OrderStatus::Canceled);
        assert_eq!(response.time_in_force, TimeInForce::FOK);
        assert_eq!(response.executed_qty.to_string(), "0.00000000");
        assert_eq!(
            response.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireMaker
        );
    }

    #[test]
    fn test_query_order_response_all_order_statuses_deserialization() {
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

        for (status_str, expected_status) in statuses {
            let json = format!(
                r#"{{
                    "symbol": "BTCUSDT",
                    "orderId": 1,
                    "orderListId": -1,
                    "clientOrderId": "order1",
                    "price": "1.00000000",
                    "origQty": "1.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "{}",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1684804350000,
                    "updateTime": 1684804350000,
                    "isWorking": true,
                    "workingTime": 1684804350000,
                    "origQuoteOrderQty": "1.00000000",
                    "selfTradePreventionMode": "NONE"
                }}"#,
                status_str
            );

            let response: QueryOrderResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.status, expected_status);
        }
    }

    #[test]
    fn test_query_order_response_all_time_in_force_deserialization() {
        let time_in_forces = vec![
            ("GTC", TimeInForce::GTC),
            ("IOC", TimeInForce::IOC),
            ("FOK", TimeInForce::FOK),
        ];

        for (tif_str, expected_tif) in time_in_forces {
            let json = format!(
                r#"{{
                    "symbol": "BTCUSDT",
                    "orderId": 1,
                    "orderListId": -1,
                    "clientOrderId": "order1",
                    "price": "1.00000000",
                    "origQty": "1.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "NEW",
                    "timeInForce": "{}",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1684804350000,
                    "updateTime": 1684804350000,
                    "isWorking": true,
                    "workingTime": 1684804350000,
                    "origQuoteOrderQty": "1.00000000",
                    "selfTradePreventionMode": "NONE"
                }}"#,
                tif_str
            );

            let response: QueryOrderResponse = serde_json::from_str(&json).unwrap();
            assert_eq!(response.time_in_force, expected_tif);
        }
    }

    #[test]
    fn test_query_order_response_precision_deserialization() {
        // Test high precision decimals
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 333,
            "orderListId": -1,
            "clientOrderId": "precisionOrder1",
            "price": "50123.45678901",
            "origQty": "0.12345678",
            "executedQty": "0.08765432",
            "cummulativeQuoteQty": "4392.87654321",
            "status": "PARTIALLY_FILLED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "time": 1684804350000,
            "updateTime": 1684804350068,
            "isWorking": true,
            "workingTime": 1684804350000,
            "origQuoteOrderQty": "6189.12345678",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.price.to_string(), "50123.45678901");
        assert_eq!(response.orig_qty.to_string(), "0.12345678");
        assert_eq!(response.executed_qty.to_string(), "0.08765432");
        assert_eq!(response.cummulative_quote_qty.to_string(), "4392.87654321");
        assert_eq!(response.orig_quote_order_qty.to_string(), "6189.12345678");
    }

    #[test]
    fn test_query_order_response_with_order_list_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 555,
            "orderListId": 12345,
            "clientOrderId": "ocoOrder1",
            "price": "55000.00000000",
            "origQty": "0.50000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "NEW",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "SELL",
            "time": 1684804350000,
            "updateTime": 1684804350000,
            "isWorking": true,
            "workingTime": 1684804350000,
            "origQuoteOrderQty": "27500.00000000",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_list_id, 12345);
        assert!(response.order_list_id > 0); // Part of an OCO order
    }

    #[test]
    fn test_query_order_response_expired_order_deserialization() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "orderId": 666,
            "orderListId": -1,
            "clientOrderId": "expiredOrder1",
            "price": "2800.00000000",
            "origQty": "1.00000000",
            "executedQty": "0.00000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "EXPIRED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "time": 1684804350000,
            "updateTime": 1684804400000,
            "isWorking": false,
            "workingTime": 1684804350000,
            "origQuoteOrderQty": "2800.00000000",
            "selfTradePreventionMode": "NONE"
        }"#;

        let response: QueryOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, OrderStatus::Expired);
        assert_eq!(response.is_working, false);
        assert_eq!(response.executed_qty, dec!(0));
    }
}
