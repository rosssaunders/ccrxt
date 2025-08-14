use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{
    ContingencyType, OrderListOrderStatus, OrderListStatus, OrderSide, OrderStatus, OrderType,
    RestResult, SelfTradePreventionMode, TimeInForce,
};

const GET_ORDERLIST_ENDPOINT: &str = "/api/v3/orderList";

/// Request parameters for querying an order list
#[derive(Debug, Clone, Serialize)]
pub struct QueryOrderListRequest {
    /// Order list ID
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    pub order_list_id: Option<u64>,

    /// Original client order ID
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Query order list response
#[derive(Debug, Clone, Deserialize)]
pub struct QueryOrderListResponse {
    /// Order list ID
    #[serde(rename = "orderListId")]
    pub order_list_id: u64,

    /// Contingency type
    #[serde(rename = "contingencyType")]
    pub contingency_type: ContingencyType,

    /// List status type
    #[serde(rename = "listStatusType")]
    pub list_status_type: OrderListStatus,

    /// List order status
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: OrderListOrderStatus,

    /// List client order ID
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: String,

    /// Transaction time
    #[serde(rename = "transactionTime")]
    pub transaction_time: u64,

    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Orders in the list
    #[serde(rename = "orders")]
    pub orders: Vec<QueryOrderListOrder>,
}

/// Order information in the order list
#[derive(Debug, Clone, Deserialize)]
pub struct QueryOrderListOrder {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

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

    /// Original quote order quantity
    #[serde(rename = "origQuoteOrderQty")]
    pub orig_quote_order_qty: Decimal,

    /// Self-trade prevention mode
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}

impl RestClient {
    /// Retrieve a specific order list
    ///
    /// Retrieve a specific order list.
    /// Either orderListId or origClientOrderId must be provided.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-order-list--user_data)
    ///
    /// Method: GET /api/v3/orderList
    /// Weight: 4
    /// Security: USER_DATA
    pub async fn query_order_list(
        &self,
        params: QueryOrderListRequest,
    ) -> RestResult<QueryOrderListResponse> {
        self.send_get_signed_request(GET_ORDERLIST_ENDPOINT, params, 4, false)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_query_orderlist_request_with_order_list_id_serialization() {
        let request = QueryOrderListRequest {
            order_list_id: Some(12345),
            orig_client_order_id: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["orderListId"], 12345);
        assert!(json.get("origClientOrderId").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_query_orderlist_request_with_orig_client_order_id_serialization() {
        let request = QueryOrderListRequest {
            order_list_id: None,
            orig_client_order_id: Some("my-order-list-123".to_string()),
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.get("orderListId").is_none());
        assert_eq!(json["origClientOrderId"], "my-order-list-123");
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_query_orderlist_request_with_both_ids_serialization() {
        // Test when both order_list_id and orig_client_order_id are provided
        let request = QueryOrderListRequest {
            order_list_id: Some(99999),
            orig_client_order_id: Some("client-list-99999".to_string()),
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["orderListId"], 99999);
        assert_eq!(json["origClientOrderId"], "client-list-99999");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_query_orderlist_request_with_recv_window_serialization() {
        let request = QueryOrderListRequest {
            order_list_id: Some(7777),
            orig_client_order_id: None,
            recv_window: Some(10000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["orderListId"], 7777);
        assert!(json.get("origClientOrderId").is_none());
        assert_eq!(json["recvWindow"], 10000);
    }

    #[test]
    fn test_query_orderlist_request_minimal_serialization() {
        // Test with no IDs provided (should be handled by API validation)
        let request = QueryOrderListRequest {
            order_list_id: None,
            orig_client_order_id: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.get("orderListId").is_none());
        assert!(json.get("origClientOrderId").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_query_orderlist_response_oco_deserialization() {
        let json = r#"{
            "orderListId": 1234567,
            "contingencyType": "OCO",
            "listStatusType": "EXEC_STARTED",
            "listOrderStatus": "EXECUTING",
            "listClientOrderId": "JYVpp3F0f5CAG15DhtrqLp",
            "transactionTime": 1684804350000,
            "symbol": "BTCUSDT",
            "orders": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 12569099453,
                    "clientOrderId": "bX5wROblo6YeDwa9iTLeyY",
                    "price": "50000.00000000",
                    "origQty": "1.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "NEW",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "SELL",
                    "time": 1684804350000,
                    "updateTime": 1684804350000,
                    "isWorking": true,
                    "origQuoteOrderQty": "50000.00000000",
                    "selfTradePreventionMode": "NONE"
                },
                {
                    "symbol": "BTCUSDT",
                    "orderId": 12569099454,
                    "clientOrderId": "Tnu2IP0J5Y4mxw3IATBfmW",
                    "price": "45000.00000000",
                    "origQty": "1.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "NEW",
                    "timeInForce": "GTC",
                    "type": "STOP_LOSS_LIMIT",
                    "side": "SELL",
                    "stopPrice": "45500.00000000",
                    "time": 1684804350000,
                    "updateTime": 1684804350000,
                    "isWorking": false,
                    "origQuoteOrderQty": "45000.00000000",
                    "selfTradePreventionMode": "NONE"
                }
            ]
        }"#;

        let response: QueryOrderListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_list_id, 1234567);
        assert_eq!(response.contingency_type, ContingencyType::Oco);
        assert_eq!(response.list_status_type, OrderListStatus::ExecStarted);
        assert_eq!(response.list_order_status, OrderListOrderStatus::Executing);
        assert_eq!(response.list_client_order_id, "JYVpp3F0f5CAG15DhtrqLp");
        assert_eq!(response.transaction_time, 1684804350000);
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.orders.len(), 2);

        // First order
        let first_order = &response.orders[0];
        assert_eq!(first_order.symbol, "BTCUSDT");
        assert_eq!(first_order.order_id, 12569099453);
        assert_eq!(first_order.client_order_id, "bX5wROblo6YeDwa9iTLeyY");
        assert_eq!(first_order.price.to_string(), "50000.00000000");
        assert_eq!(first_order.orig_qty.to_string(), "1.00000000");
        assert_eq!(first_order.executed_qty.to_string(), "0.00000000");
        assert_eq!(first_order.cummulative_quote_qty.to_string(), "0.00000000");
        assert_eq!(first_order.status, OrderStatus::New);
        assert_eq!(first_order.time_in_force, TimeInForce::GTC);
        assert_eq!(first_order.order_type, OrderType::Limit);
        assert_eq!(first_order.side, OrderSide::Sell);
        assert_eq!(first_order.time, 1684804350000);
        assert_eq!(first_order.update_time, 1684804350000);
        assert!(first_order.is_working);
        assert_eq!(
            first_order.orig_quote_order_qty.to_string(),
            "50000.00000000"
        );
        assert_eq!(
            first_order.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        );
        assert!(first_order.stop_price.is_none());
        assert!(first_order.iceberg_qty.is_none());

        // Second order (stop loss)
        let second_order = &response.orders[1];
        assert_eq!(second_order.symbol, "BTCUSDT");
        assert_eq!(second_order.order_id, 12569099454);
        assert_eq!(second_order.client_order_id, "Tnu2IP0J5Y4mxw3IATBfmW");
        assert_eq!(second_order.price.to_string(), "45000.00000000");
        assert_eq!(second_order.orig_qty.to_string(), "1.00000000");
        assert_eq!(second_order.executed_qty.to_string(), "0.00000000");
        assert_eq!(second_order.cummulative_quote_qty.to_string(), "0.00000000");
        assert_eq!(second_order.status, OrderStatus::New);
        assert_eq!(second_order.time_in_force, TimeInForce::GTC);
        assert_eq!(second_order.order_type, OrderType::StopLossLimit);
        assert_eq!(second_order.side, OrderSide::Sell);
        assert_eq!(
            second_order.stop_price.unwrap().to_string(),
            "45500.00000000"
        );
        assert_eq!(second_order.time, 1684804350000);
        assert_eq!(second_order.update_time, 1684804350000);
        assert!(!second_order.is_working);
        assert_eq!(
            second_order.orig_quote_order_qty.to_string(),
            "45000.00000000"
        );
        assert_eq!(
            second_order.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        );
        assert!(second_order.iceberg_qty.is_none());
    }

    #[test]
    fn test_query_orderlist_response_oto_deserialization() {
        let json = r#"{
            "orderListId": 2234567,
            "contingencyType": "OTO",
            "listStatusType": "ALL_DONE",
            "listOrderStatus": "ALL_DONE",
            "listClientOrderId": "OtoOrderList123",
            "transactionTime": 1684804360000,
            "symbol": "ETHUSDT",
            "orders": [
                {
                    "symbol": "ETHUSDT",
                    "orderId": 22569099453,
                    "clientOrderId": "primary-order-123",
                    "price": "3000.00000000",
                    "origQty": "2.50000000",
                    "executedQty": "2.50000000",
                    "cummulativeQuoteQty": "7500.00000000",
                    "status": "FILLED",
                    "timeInForce": "IOC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1684804360000,
                    "updateTime": 1684804360100,
                    "isWorking": false,
                    "origQuoteOrderQty": "7500.00000000",
                    "selfTradePreventionMode": "EXPIRE_TAKER"
                },
                {
                    "symbol": "ETHUSDT",
                    "orderId": 22569099454,
                    "clientOrderId": "pending-order-123",
                    "price": "2950.00000000",
                    "origQty": "2.50000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "EXPIRED",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1684804360000,
                    "updateTime": 1684804360100,
                    "isWorking": false,
                    "origQuoteOrderQty": "7375.00000000",
                    "selfTradePreventionMode": "EXPIRE_BOTH"
                }
            ]
        }"#;

        let response: QueryOrderListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_list_id, 2234567);
        assert_eq!(response.contingency_type, ContingencyType::Oto);
        assert_eq!(response.list_status_type, OrderListStatus::AllDone);
        assert_eq!(response.list_order_status, OrderListOrderStatus::AllDone);
        assert_eq!(response.list_client_order_id, "OtoOrderList123");
        assert_eq!(response.transaction_time, 1684804360000);
        assert_eq!(response.symbol, "ETHUSDT");
        assert_eq!(response.orders.len(), 2);

        // Primary order (filled)
        let primary_order = &response.orders[0];
        assert_eq!(primary_order.status, OrderStatus::Filled);
        assert_eq!(primary_order.executed_qty.to_string(), "2.50000000");
        assert_eq!(primary_order.time_in_force, TimeInForce::IOC);
        assert_eq!(
            primary_order.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        );

        // Pending order (expired)
        let pending_order = &response.orders[1];
        assert_eq!(pending_order.status, OrderStatus::Expired);
        assert_eq!(pending_order.executed_qty.to_string(), "0.00000000");
        assert_eq!(
            pending_order.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        );
    }

    #[test]
    fn test_query_orderlist_response_otoco_deserialization() {
        let json = r#"{
            "orderListId": 3345678,
            "contingencyType": "OTOCO",
            "listStatusType": "REJECT",
            "listOrderStatus": "REJECT",
            "listClientOrderId": "OtocoOrderList456",
            "transactionTime": 1684804370000,
            "symbol": "BNBUSDT",
            "orders": [
                {
                    "symbol": "BNBUSDT",
                    "orderId": 33569099453,
                    "clientOrderId": "primary-otoco-order",
                    "price": "300.00000000",
                    "origQty": "10.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "REJECTED",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1684804370000,
                    "updateTime": 1684804370000,
                    "isWorking": false,
                    "origQuoteOrderQty": "3000.00000000",
                    "selfTradePreventionMode": "NONE"
                },
                {
                    "symbol": "BNBUSDT",
                    "orderId": 33569099454,
                    "clientOrderId": "oco-sell-order-1",
                    "price": "320.00000000",
                    "origQty": "10.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "REJECTED",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "SELL",
                    "time": 1684804370000,
                    "updateTime": 1684804370000,
                    "isWorking": false,
                    "origQuoteOrderQty": "3200.00000000",
                    "selfTradePreventionMode": "NONE"
                },
                {
                    "symbol": "BNBUSDT",
                    "orderId": 33569099455,
                    "clientOrderId": "oco-stop-order",
                    "price": "280.00000000",
                    "origQty": "10.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "REJECTED",
                    "timeInForce": "GTC",
                    "type": "STOP_LOSS_LIMIT",
                    "side": "SELL",
                    "stopPrice": "285.00000000",
                    "time": 1684804370000,
                    "updateTime": 1684804370000,
                    "isWorking": false,
                    "origQuoteOrderQty": "2800.00000000",
                    "selfTradePreventionMode": "NONE"
                }
            ]
        }"#;

        let response: QueryOrderListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_list_id, 3345678);
        assert_eq!(response.contingency_type, ContingencyType::Otoco);
        assert_eq!(response.list_status_type, OrderListStatus::Reject);
        assert_eq!(response.list_order_status, OrderListOrderStatus::Reject);
        assert_eq!(response.list_client_order_id, "OtocoOrderList456");
        assert_eq!(response.transaction_time, 1684804370000);
        assert_eq!(response.symbol, "BNBUSDT");
        assert_eq!(response.orders.len(), 3);

        // All orders should be rejected
        for order in &response.orders {
            assert_eq!(order.status, OrderStatus::Rejected);
            assert_eq!(order.executed_qty.to_string(), "0.00000000");
            assert!(!order.is_working);
        }

        // Check stop order has stop price
        let stop_order = &response.orders[2];
        assert_eq!(stop_order.order_type, OrderType::StopLossLimit);
        assert_eq!(stop_order.stop_price.unwrap().to_string(), "285.00000000");
    }

    #[test]
    fn test_query_orderlist_response_with_iceberg_deserialization() {
        let json = r#"{
            "orderListId": 4456789,
            "contingencyType": "OCO",
            "listStatusType": "RESPONSE",
            "listOrderStatus": "EXECUTING",
            "listClientOrderId": "IcebergOcoList789",
            "transactionTime": 1684804380000,
            "symbol": "SOLUSDT",
            "orders": [
                {
                    "symbol": "SOLUSDT",
                    "orderId": 44569099453,
                    "clientOrderId": "iceberg-limit-order",
                    "price": "100.00000000",
                    "origQty": "100.00000000",
                    "executedQty": "20.00000000",
                    "cummulativeQuoteQty": "2000.00000000",
                    "status": "PARTIALLY_FILLED",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "icebergQty": "10.00000000",
                    "time": 1684804380000,
                    "updateTime": 1684804381000,
                    "isWorking": true,
                    "origQuoteOrderQty": "10000.00000000",
                    "selfTradePreventionMode": "EXPIRE_MAKER"
                },
                {
                    "symbol": "SOLUSDT",
                    "orderId": 44569099454,
                    "clientOrderId": "stop-loss-order",
                    "price": "90.00000000",
                    "origQty": "100.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "NEW",
                    "timeInForce": "GTC",
                    "type": "STOP_LOSS_LIMIT",
                    "side": "SELL",
                    "stopPrice": "92.00000000",
                    "time": 1684804380000,
                    "updateTime": 1684804380000,
                    "isWorking": false,
                    "origQuoteOrderQty": "9000.00000000",
                    "selfTradePreventionMode": "EXPIRE_MAKER"
                }
            ]
        }"#;

        let response: QueryOrderListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.order_list_id, 4456789);
        assert_eq!(response.contingency_type, ContingencyType::Oco);
        assert_eq!(response.list_status_type, OrderListStatus::Response);
        assert_eq!(response.list_order_status, OrderListOrderStatus::Executing);
        assert_eq!(response.list_client_order_id, "IcebergOcoList789");
        assert_eq!(response.transaction_time, 1684804380000);
        assert_eq!(response.symbol, "SOLUSDT");
        assert_eq!(response.orders.len(), 2);

        // Check iceberg order
        let iceberg_order = &response.orders[0];
        assert_eq!(
            iceberg_order.iceberg_qty.unwrap().to_string(),
            "10.00000000"
        );
        assert_eq!(iceberg_order.status, OrderStatus::PartiallyFilled);
        assert_eq!(iceberg_order.executed_qty.to_string(), "20.00000000");
        assert!(iceberg_order.is_working);
        assert_eq!(
            iceberg_order.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireMaker
        );
    }

    #[test]
    fn test_query_orderlist_order_fields_deserialization() {
        // Test that all fields deserialize correctly
        let json = json!({
            "symbol": "BTCUSDT",
            "orderId": 123456789,
            "clientOrderId": "test-client-order-id",
            "price": "50000.12345678",
            "origQty": "1.23456789",
            "executedQty": "0.12345678",
            "cummulativeQuoteQty": "6172.83949506",
            "status": "CANCELED",
            "timeInForce": "FOK",
            "type": "MARKET",
            "side": "BUY",
            "stopPrice": "49000.00000000",
            "icebergQty": "0.10000000",
            "time": 1684804390000_u64,
            "updateTime": 1684804391000_u64,
            "isWorking": false,
            "origQuoteOrderQty": "50000.00000000",
            "selfTradePreventionMode": "EXPIRE_BOTH"
        });

        let order: QueryOrderListOrder = serde_json::from_value(json).unwrap();
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, 123456789);
        assert_eq!(order.client_order_id, "test-client-order-id");
        assert_eq!(order.price, dec!(50000.12345678));
        assert_eq!(order.orig_qty, dec!(1.23456789));
        assert_eq!(order.executed_qty, dec!(0.12345678));
        assert_eq!(order.cummulative_quote_qty, dec!(6172.83949506));
        assert_eq!(order.status, OrderStatus::Canceled);
        assert_eq!(order.time_in_force, TimeInForce::FOK);
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.stop_price, Some(dec!(49000.00000000)));
        assert_eq!(order.iceberg_qty, Some(dec!(0.10000000)));
        assert_eq!(order.time, 1684804390000);
        assert_eq!(order.update_time, 1684804391000);
        assert!(!order.is_working);
        assert_eq!(order.orig_quote_order_qty, dec!(50000.00000000));
        assert_eq!(
            order.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        );
    }

    #[test]
    fn test_query_orderlist_response_minimal_order_deserialization() {
        // Test order with only required fields (no optional stop_price or iceberg_qty)
        let json = r#"{
            "orderListId": 5567890,
            "contingencyType": "OCO",
            "listStatusType": "ALL_DONE",
            "listOrderStatus": "ALL_DONE",
            "listClientOrderId": "MinimalOcoList",
            "transactionTime": 1684804400000,
            "symbol": "BTCUSDT",
            "orders": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 55569099453,
                    "clientOrderId": "minimal-order",
                    "price": "0.00000000",
                    "origQty": "0.50000000",
                    "executedQty": "0.50000000",
                    "cummulativeQuoteQty": "25000.00000000",
                    "status": "FILLED",
                    "timeInForce": "GTC",
                    "type": "MARKET",
                    "side": "BUY",
                    "time": 1684804400000,
                    "updateTime": 1684804400500,
                    "isWorking": false,
                    "origQuoteOrderQty": "25000.00000000",
                    "selfTradePreventionMode": "NONE"
                }
            ]
        }"#;

        let response: QueryOrderListResponse = serde_json::from_str(json).unwrap();
        let order = &response.orders[0];
        assert!(order.stop_price.is_none());
        assert!(order.iceberg_qty.is_none());
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.price.to_string(), "0.00000000");
    }
}
