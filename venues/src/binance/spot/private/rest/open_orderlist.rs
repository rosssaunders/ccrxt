use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::binance::spot::{
    ContingencyType, OrderListOrderStatus, OrderListStatus, OrderSide, OrderStatus, OrderType,
    RestResult, SelfTradePreventionMode, TimeInForce, private_client::RestClient,
};

const GET_OPEN_ORDERLIST_ENDPOINT: &str = "/api/v3/openOrderList";

/// Request parameters for getting open order lists
#[derive(Debug, Clone, Serialize, Default)]
pub struct OpenOrderListRequest {
    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Open order list information
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrderList {
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
    pub orders: Vec<OpenOrderListOrder>,
}

/// Order information in the open order list
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrderListOrder {
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
    /// Get all open order lists
    ///
    /// Get all open order lists.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-open-order-lists--user_data)
    ///
    /// Method: GET /api/v3/openOrderList
    /// Weight: 6
    /// Security: USER_DATA
    pub async fn get_open_order_lists(
        &self,
        params: Option<OpenOrderListRequest>,
    ) -> RestResult<Vec<OpenOrderList>> {
        self.send_get_signed_request(
            GET_OPEN_ORDERLIST_ENDPOINT,
            params.unwrap_or_default(),
            6,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_open_order_list_request_serialization_default() {
        let request = OpenOrderListRequest::default();

        // Default should serialize to empty string (no fields)
        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_open_order_list_request_serialization_with_recv_window() {
        let request = OpenOrderListRequest {
            recv_window: Some(5000),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "recvWindow=5000");
    }

    #[test]
    fn test_open_order_list_deserialization_oco() {
        let json = r#"{
            "orderListId": 123456,
            "contingencyType": "OCO",
            "listStatusType": "ALL_DONE",
            "listOrderStatus": "ALL_DONE",
            "listClientOrderId": "myOCOList1",
            "transactionTime": 1625184000000,
            "symbol": "BTCUSDT",
            "orders": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 100001,
                    "clientOrderId": "order1",
                    "price": "50000.00",
                    "origQty": "0.10000000",
                    "executedQty": "0.10000000",
                    "cummulativeQuoteQty": "5000.00000000",
                    "status": "FILLED",
                    "timeInForce": "GTC",
                    "type": "LIMIT_MAKER",
                    "side": "BUY",
                    "time": 1625183900000,
                    "updateTime": 1625184000000,
                    "isWorking": false,
                    "origQuoteOrderQty": "5000.00000000",
                    "selfTradePreventionMode": "EXPIRE_TAKER"
                },
                {
                    "symbol": "BTCUSDT",
                    "orderId": 100002,
                    "clientOrderId": "order2",
                    "price": "48000.00",
                    "origQty": "0.10000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "EXPIRED",
                    "timeInForce": "GTC",
                    "type": "STOP_LOSS_LIMIT",
                    "side": "SELL",
                    "stopPrice": "48500.00",
                    "time": 1625183900000,
                    "updateTime": 1625184000000,
                    "isWorking": false,
                    "origQuoteOrderQty": "4800.00000000",
                    "selfTradePreventionMode": "EXPIRE_BOTH"
                }
            ]
        }"#;

        let order_list: OpenOrderList = serde_json::from_str(json).unwrap();
        assert_eq!(order_list.order_list_id, 123456);
        assert!(matches!(order_list.contingency_type, ContingencyType::Oco));
        assert!(matches!(
            order_list.list_status_type,
            OrderListStatus::AllDone
        ));
        assert!(matches!(
            order_list.list_order_status,
            OrderListOrderStatus::AllDone
        ));
        assert_eq!(order_list.list_client_order_id, "myOCOList1");
        assert_eq!(order_list.transaction_time, 1625184000000);
        assert_eq!(order_list.symbol, "BTCUSDT");
        assert_eq!(order_list.orders.len(), 2);

        // Verify first order
        let order1 = &order_list.orders[0];
        assert_eq!(order1.symbol, "BTCUSDT");
        assert_eq!(order1.order_id, 100001);
        assert_eq!(order1.client_order_id, "order1");
        assert_eq!(order1.price, dec!(50000.00));
        assert_eq!(order1.orig_qty, dec!(0.10000000));
        assert_eq!(order1.executed_qty, dec!(0.10000000));
        assert_eq!(order1.cummulative_quote_qty, dec!(5000.00000000));
        assert!(matches!(order1.status, OrderStatus::Filled));
        assert!(matches!(order1.time_in_force, TimeInForce::GTC));
        assert!(matches!(order1.order_type, OrderType::LimitMaker));
        assert!(matches!(order1.side, OrderSide::Buy));
        assert_eq!(order1.stop_price, None);
        assert_eq!(order1.iceberg_qty, None);
        assert_eq!(order1.time, 1625183900000);
        assert_eq!(order1.update_time, 1625184000000);
        assert!(!order1.is_working);
        assert_eq!(order1.orig_quote_order_qty, dec!(5000.00000000));
        assert!(matches!(
            order1.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireTaker
        ));

        // Verify second order
        let order2 = &order_list.orders[1];
        assert_eq!(order2.symbol, "BTCUSDT");
        assert_eq!(order2.order_id, 100002);
        assert_eq!(order2.client_order_id, "order2");
        assert_eq!(order2.price, dec!(48000.00));
        assert_eq!(order2.orig_qty, dec!(0.10000000));
        assert_eq!(order2.executed_qty, dec!(0.00000000));
        assert_eq!(order2.cummulative_quote_qty, dec!(0.00000000));
        assert!(matches!(order2.status, OrderStatus::Expired));
        assert!(matches!(order2.time_in_force, TimeInForce::GTC));
        assert!(matches!(order2.order_type, OrderType::StopLossLimit));
        assert!(matches!(order2.side, OrderSide::Sell));
        assert_eq!(order2.stop_price, Some(dec!(48500.00)));
        assert_eq!(order2.iceberg_qty, None);
        assert_eq!(order2.time, 1625183900000);
        assert_eq!(order2.update_time, 1625184000000);
        assert!(!order2.is_working);
        assert_eq!(order2.orig_quote_order_qty, dec!(4800.00000000));
        assert!(matches!(
            order2.self_trade_prevention_mode,
            SelfTradePreventionMode::ExpireBoth
        ));
    }

    #[test]
    fn test_open_order_list_deserialization_oto() {
        let json = r#"{
            "orderListId": 234567,
            "contingencyType": "OTO",
            "listStatusType": "EXEC_STARTED",
            "listOrderStatus": "EXECUTING",
            "listClientOrderId": "myOTOList1",
            "transactionTime": 1625270400000,
            "symbol": "ETHUSDT",
            "orders": [
                {
                    "symbol": "ETHUSDT",
                    "orderId": 200001,
                    "clientOrderId": "primaryOrder",
                    "price": "3000.00",
                    "origQty": "1.00000000",
                    "executedQty": "0.50000000",
                    "cummulativeQuoteQty": "1500.00000000",
                    "status": "PARTIALLY_FILLED",
                    "timeInForce": "IOC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1625270300000,
                    "updateTime": 1625270400000,
                    "isWorking": true,
                    "origQuoteOrderQty": "3000.00000000",
                    "selfTradePreventionMode": "EXPIRE_MAKER"
                },
                {
                    "symbol": "ETHUSDT",
                    "orderId": 200002,
                    "clientOrderId": "secondaryOrder",
                    "price": "3100.00",
                    "origQty": "1.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "NEW",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "icebergQty": "0.10000000",
                    "time": 1625270300000,
                    "updateTime": 1625270400000,
                    "isWorking": false,
                    "origQuoteOrderQty": "3100.00000000",
                    "selfTradePreventionMode": "NONE"
                }
            ]
        }"#;

        let order_list: OpenOrderList = serde_json::from_str(json).unwrap();
        assert_eq!(order_list.order_list_id, 234567);
        assert!(matches!(order_list.contingency_type, ContingencyType::Oto));
        assert!(matches!(
            order_list.list_status_type,
            OrderListStatus::ExecStarted
        ));
        assert!(matches!(
            order_list.list_order_status,
            OrderListOrderStatus::Executing
        ));
        assert_eq!(order_list.list_client_order_id, "myOTOList1");
        assert_eq!(order_list.transaction_time, 1625270400000);
        assert_eq!(order_list.symbol, "ETHUSDT");
        assert_eq!(order_list.orders.len(), 2);

        // Verify first order with is_working = true
        let order1 = &order_list.orders[0];
        assert!(matches!(order1.status, OrderStatus::PartiallyFilled));
        assert!(matches!(order1.time_in_force, TimeInForce::IOC));
        assert!(order1.is_working);
        assert_eq!(order1.executed_qty, dec!(0.50000000));

        // Verify second order with iceberg_qty
        let order2 = &order_list.orders[1];
        assert!(matches!(order2.status, OrderStatus::New));
        assert_eq!(order2.iceberg_qty, Some(dec!(0.10000000)));
        assert!(!order2.is_working);
        assert!(matches!(
            order2.self_trade_prevention_mode,
            SelfTradePreventionMode::None
        ));
    }

    #[test]
    fn test_open_order_list_deserialization_otoco() {
        let json = r#"{
            "orderListId": 345678,
            "contingencyType": "OTOCO",
            "listStatusType": "RESPONSE",
            "listOrderStatus": "ALL_DONE",
            "listClientOrderId": "myOTOCOList1",
            "transactionTime": 1625356800000,
            "symbol": "BNBUSDT",
            "orders": [
                {
                    "symbol": "BNBUSDT",
                    "orderId": 300001,
                    "clientOrderId": "primaryOTOCO",
                    "price": "400.00",
                    "origQty": "10.00000000",
                    "executedQty": "10.00000000",
                    "cummulativeQuoteQty": "4000.00000000",
                    "status": "FILLED",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "BUY",
                    "time": 1625356700000,
                    "updateTime": 1625356800000,
                    "isWorking": false,
                    "origQuoteOrderQty": "4000.00000000",
                    "selfTradePreventionMode": "EXPIRE_TAKER"
                },
                {
                    "symbol": "BNBUSDT",
                    "orderId": 300002,
                    "clientOrderId": "ocoLeg1",
                    "price": "450.00",
                    "origQty": "10.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "NEW",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "side": "SELL",
                    "time": 1625356700000,
                    "updateTime": 1625356800000,
                    "isWorking": true,
                    "origQuoteOrderQty": "4500.00000000",
                    "selfTradePreventionMode": "EXPIRE_BOTH"
                },
                {
                    "symbol": "BNBUSDT",
                    "orderId": 300003,
                    "clientOrderId": "ocoLeg2",
                    "price": "380.00",
                    "origQty": "10.00000000",
                    "executedQty": "0.00000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "NEW",
                    "timeInForce": "GTC",
                    "type": "STOP_LOSS_LIMIT",
                    "side": "SELL",
                    "stopPrice": "385.00",
                    "time": 1625356700000,
                    "updateTime": 1625356800000,
                    "isWorking": false,
                    "origQuoteOrderQty": "3800.00000000",
                    "selfTradePreventionMode": "EXPIRE_BOTH"
                }
            ]
        }"#;

        let order_list: OpenOrderList = serde_json::from_str(json).unwrap();
        assert_eq!(order_list.order_list_id, 345678);
        assert!(matches!(
            order_list.contingency_type,
            ContingencyType::Otoco
        ));
        assert!(matches!(
            order_list.list_status_type,
            OrderListStatus::Response
        ));
        assert!(matches!(
            order_list.list_order_status,
            OrderListOrderStatus::AllDone
        ));
        assert_eq!(order_list.list_client_order_id, "myOTOCOList1");
        assert_eq!(order_list.transaction_time, 1625356800000);
        assert_eq!(order_list.symbol, "BNBUSDT");
        assert_eq!(order_list.orders.len(), 3);

        // Verify all three orders exist
        assert_eq!(order_list.orders[0].client_order_id, "primaryOTOCO");
        assert_eq!(order_list.orders[1].client_order_id, "ocoLeg1");
        assert_eq!(order_list.orders[2].client_order_id, "ocoLeg2");

        // Verify stop order has stop price
        assert_eq!(order_list.orders[2].stop_price, Some(dec!(385.00)));
    }

    #[test]
    fn test_open_order_list_array_deserialization_empty() {
        let json = "[]";
        let order_lists: Vec<OpenOrderList> = serde_json::from_str(json).unwrap();
        assert_eq!(order_lists.len(), 0);
    }

    #[test]
    fn test_open_order_list_array_deserialization_multiple() {
        let json = r#"[
            {
                "orderListId": 111111,
                "contingencyType": "OCO",
                "listStatusType": "ALL_DONE",
                "listOrderStatus": "ALL_DONE",
                "listClientOrderId": "list1",
                "transactionTime": 1625184000000,
                "symbol": "BTCUSDT",
                "orders": [
                    {
                        "symbol": "BTCUSDT",
                        "orderId": 1001,
                        "clientOrderId": "order1a",
                        "price": "50000.00",
                        "origQty": "0.10000000",
                        "executedQty": "0.10000000",
                        "cummulativeQuoteQty": "5000.00000000",
                        "status": "FILLED",
                        "timeInForce": "GTC",
                        "type": "LIMIT",
                        "side": "BUY",
                        "time": 1625183900000,
                        "updateTime": 1625184000000,
                        "isWorking": false,
                        "origQuoteOrderQty": "5000.00000000",
                        "selfTradePreventionMode": "EXPIRE_TAKER"
                    },
                    {
                        "symbol": "BTCUSDT",
                        "orderId": 1002,
                        "clientOrderId": "order1b",
                        "price": "48000.00",
                        "origQty": "0.10000000",
                        "executedQty": "0.00000000",
                        "cummulativeQuoteQty": "0.00000000",
                        "status": "EXPIRED",
                        "timeInForce": "GTC",
                        "type": "STOP_LOSS",
                        "side": "SELL",
                        "stopPrice": "48500.00",
                        "time": 1625183900000,
                        "updateTime": 1625184000000,
                        "isWorking": false,
                        "origQuoteOrderQty": "4800.00000000",
                        "selfTradePreventionMode": "EXPIRE_BOTH"
                    }
                ]
            },
            {
                "orderListId": 222222,
                "contingencyType": "OTO",
                "listStatusType": "RESPONSE",
                "listOrderStatus": "EXECUTING",
                "listClientOrderId": "list2",
                "transactionTime": 1625270400000,
                "symbol": "ETHUSDT",
                "orders": [
                    {
                        "symbol": "ETHUSDT",
                        "orderId": 2001,
                        "clientOrderId": "order2a",
                        "price": "3000.00",
                        "origQty": "1.00000000",
                        "executedQty": "0.00000000",
                        "cummulativeQuoteQty": "0.00000000",
                        "status": "NEW",
                        "timeInForce": "GTC",
                        "type": "LIMIT",
                        "side": "BUY",
                        "time": 1625270300000,
                        "updateTime": 1625270400000,
                        "isWorking": true,
                        "origQuoteOrderQty": "3000.00000000",
                        "selfTradePreventionMode": "EXPIRE_MAKER"
                    },
                    {
                        "symbol": "ETHUSDT",
                        "orderId": 2002,
                        "clientOrderId": "order2b",
                        "price": "3100.00",
                        "origQty": "1.00000000",
                        "executedQty": "0.00000000",
                        "cummulativeQuoteQty": "0.00000000",
                        "status": "NEW",
                        "timeInForce": "GTC",
                        "type": "LIMIT",
                        "side": "BUY",
                        "time": 1625270300000,
                        "updateTime": 1625270400000,
                        "isWorking": false,
                        "origQuoteOrderQty": "3100.00000000",
                        "selfTradePreventionMode": "NONE"
                    }
                ]
            }
        ]"#;

        let order_lists: Vec<OpenOrderList> = serde_json::from_str(json).unwrap();
        assert_eq!(order_lists.len(), 2);

        // Verify first list
        assert_eq!(order_lists[0].order_list_id, 111111);
        assert!(matches!(
            order_lists[0].contingency_type,
            ContingencyType::Oco
        ));
        assert_eq!(order_lists[0].orders.len(), 2);

        // Verify second list
        assert_eq!(order_lists[1].order_list_id, 222222);
        assert!(matches!(
            order_lists[1].contingency_type,
            ContingencyType::Oto
        ));
        assert_eq!(order_lists[1].orders.len(), 2);
    }

    #[test]
    fn test_open_order_list_order_deserialization_market_order() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 400001,
            "clientOrderId": "marketOrder1",
            "price": "0.00",
            "origQty": "0.10000000",
            "executedQty": "0.10000000",
            "cummulativeQuoteQty": "5023.45000000",
            "status": "FILLED",
            "timeInForce": "IOC",
            "type": "MARKET",
            "side": "BUY",
            "time": 1625443200000,
            "updateTime": 1625443200100,
            "isWorking": false,
            "origQuoteOrderQty": "5000.00000000",
            "selfTradePreventionMode": "EXPIRE_TAKER"
        }"#;

        let order: OpenOrderListOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, 400001);
        assert_eq!(order.client_order_id, "marketOrder1");
        assert_eq!(order.price, dec!(0.00));
        assert!(matches!(order.order_type, OrderType::Market));
        assert!(matches!(order.time_in_force, TimeInForce::IOC));
        assert_eq!(order.executed_qty, dec!(0.10000000));
        assert_eq!(order.cummulative_quote_qty, dec!(5023.45000000));
        assert!(matches!(order.status, OrderStatus::Filled));
    }

    #[test]
    fn test_open_order_list_order_all_status_types() {
        // Test different order statuses
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
                "orderId": 500001,
                "clientOrderId": "testOrder",
                "price": "50000.00",
                "origQty": "0.10000000",
                "executedQty": "0.00000000",
                "cummulativeQuoteQty": "0.00000000",
                "status": "{}",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "BUY",
                "time": 1625443200000,
                "updateTime": 1625443200100,
                "isWorking": true,
                "origQuoteOrderQty": "5000.00000000",
                "selfTradePreventionMode": "NONE"
            }}"#,
                status_str
            );

            let order: OpenOrderListOrder = serde_json::from_str(&json).unwrap();
            assert!(
                matches!(order.status, ref s if std::mem::discriminant(s) == std::mem::discriminant(&expected_status))
            );
        }
    }
}
