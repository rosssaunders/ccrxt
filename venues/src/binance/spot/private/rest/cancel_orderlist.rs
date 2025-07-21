use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::binance::spot::{ContingencyType, OrderListOrderStatus, OrderListStatus, RestResult};

const CANCEL_ORDERLIST_ENDPOINT: &str = "/api/v3/orderList";

/// Request parameters for canceling an order list
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderListRequest {
    /// Trading pair symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order list ID
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    pub order_list_id: Option<u64>,

    /// List client order ID
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,

    /// New client order ID
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    /// Receive window
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u64>,
}

/// Cancel order list response
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderListResponse {
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

    /// Orders in the canceled list
    #[serde(rename = "orders")]
    pub orders: Vec<CancelOrderListOrder>,

    /// Order reports
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<serde_json::Value>,
}

/// Order information in the canceled order list
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderListOrder {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: u64,

    /// Client order ID
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
}

impl RestClient {
    /// Cancel an entire order list
    ///
    /// Cancel an entire order list.
    /// Either orderListId or listClientOrderId must be provided.
    ///
    /// See: [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#cancel-order-list--trade)
    /// Method: DELETE /api/v3/orderList
    /// Weight: 1
    /// Security: TRADE
    pub async fn cancel_order_list(
        &self,
        params: CancelOrderListRequest,
    ) -> RestResult<CancelOrderListResponse> {
        self.send_signed_request(
            CANCEL_ORDERLIST_ENDPOINT,
            reqwest::Method::DELETE,
            params,
            1,
            true,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_order_list_request_with_order_list_id() {
        // Test request serialization with order_list_id
        let request = CancelOrderListRequest {
            symbol: "BTCUSDT".to_string(),
            order_list_id: Some(12345),
            list_client_order_id: None,
            new_client_order_id: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["orderListId"], 12345);
        assert!(json.get("listClientOrderId").is_none());
        assert!(json.get("newClientOrderId").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_cancel_order_list_request_with_list_client_order_id() {
        // Test request serialization with list_client_order_id
        let request = CancelOrderListRequest {
            symbol: "ETHUSDT".to_string(),
            order_list_id: None,
            list_client_order_id: Some("my-order-list-123".to_string()),
            new_client_order_id: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ETHUSDT");
        assert!(json.get("orderListId").is_none());
        assert_eq!(json["listClientOrderId"], "my-order-list-123");
        assert!(json.get("newClientOrderId").is_none());
        assert!(json.get("recvWindow").is_none());
    }

    #[test]
    fn test_cancel_order_list_request_with_both_ids() {
        // Test request with both order_list_id and list_client_order_id (API should handle priority)
        let request = CancelOrderListRequest {
            symbol: "BNBUSDT".to_string(),
            order_list_id: Some(67890),
            list_client_order_id: Some("another-list-456".to_string()),
            new_client_order_id: Some("new-client-id-789".to_string()),
            recv_window: Some(5000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BNBUSDT");
        assert_eq!(json["orderListId"], 67890);
        assert_eq!(json["listClientOrderId"], "another-list-456");
        assert_eq!(json["newClientOrderId"], "new-client-id-789");
        assert_eq!(json["recvWindow"], 5000);
    }

    #[test]
    fn test_cancel_order_list_request_minimal() {
        // Test minimal request (should have at least symbol and one ID)
        let request = CancelOrderListRequest {
            symbol: "ADAUSDT".to_string(),
            order_list_id: None,
            list_client_order_id: None,
            new_client_order_id: None,
            recv_window: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "ADAUSDT");
        // Both IDs are None - this would fail on API, but serialization should work
        assert!(json.get("orderListId").is_none());
        assert!(json.get("listClientOrderId").is_none());
    }

    #[test]
    fn test_cancel_order_list_request_all_fields() {
        // Test all fields populated
        let request = CancelOrderListRequest {
            symbol: "DOTUSDT".to_string(),
            order_list_id: Some(999999999),
            list_client_order_id: Some(
                "complex-order-list-id-with-special-chars-123_456.789".to_string(),
            ),
            new_client_order_id: Some("new-order-id-xyz".to_string()),
            recv_window: Some(60000),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "DOTUSDT");
        assert_eq!(json["orderListId"], 999999999);
        assert_eq!(
            json["listClientOrderId"],
            "complex-order-list-id-with-special-chars-123_456.789"
        );
        assert_eq!(json["newClientOrderId"], "new-order-id-xyz");
        assert_eq!(json["recvWindow"], 60000);
    }

    #[test]
    fn test_cancel_order_list_request_different_symbols() {
        // Test various symbol formats
        let symbols = vec!["BTCUSDT", "ETHBTC", "BNBETH", "XRPUSDT", "SOLUSDT"];

        for symbol in symbols {
            let request = CancelOrderListRequest {
                symbol: symbol.to_string(),
                order_list_id: Some(12345),
                list_client_order_id: None,
                new_client_order_id: None,
                recv_window: None,
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["symbol"], symbol);
        }
    }

    #[test]
    fn test_cancel_order_list_response_deserialization() {
        // Test comprehensive response deserialization
        let json = r#"{
            "orderListId": 123456789,
            "contingencyType": "OCO",
            "listStatusType": "ALL_DONE",
            "listOrderStatus": "ALL_DONE",
            "listClientOrderId": "my-oco-order-123",
            "transactionTime": 1621910400000,
            "symbol": "BTCUSDT",
            "orders": [
                {
                    "symbol": "BTCUSDT",
                    "orderId": 111111,
                    "clientOrderId": "order-1"
                },
                {
                    "symbol": "BTCUSDT",
                    "orderId": 222222,
                    "clientOrderId": "order-2"
                }
            ],
            "orderReports": []
        }"#;

        let response: CancelOrderListResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_list_id, 123456789);
        assert!(matches!(response.contingency_type, ContingencyType::Oco));
        assert!(matches!(
            response.list_status_type,
            OrderListStatus::AllDone
        ));
        assert!(matches!(
            response.list_order_status,
            OrderListOrderStatus::AllDone
        ));
        assert_eq!(response.list_client_order_id, "my-oco-order-123");
        assert_eq!(response.transaction_time, 1621910400000);
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.orders.len(), 2);
        assert_eq!(response.order_reports.len(), 0);
    }

    #[test]
    fn test_cancel_order_list_order_deserialization() {
        // Test individual order deserialization
        let json = r#"{
            "symbol": "ETHUSDT",
            "orderId": 987654321,
            "clientOrderId": "my-client-order-456"
        }"#;

        let order: CancelOrderListOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "ETHUSDT");
        assert_eq!(order.order_id, 987654321);
        assert_eq!(order.client_order_id, "my-client-order-456");
    }

    #[test]
    fn test_cancel_order_list_response_oto_type() {
        // Test response with OTO contingency type
        let json = r#"{
            "orderListId": 555555,
            "contingencyType": "OTO",
            "listStatusType": "EXEC_STARTED",
            "listOrderStatus": "EXECUTING",
            "listClientOrderId": "oto-order-789",
            "transactionTime": 1621920000000,
            "symbol": "BNBUSDT",
            "orders": [
                {
                    "symbol": "BNBUSDT",
                    "orderId": 333333,
                    "clientOrderId": "primary-order"
                },
                {
                    "symbol": "BNBUSDT",
                    "orderId": 444444,
                    "clientOrderId": "contingent-order"
                }
            ],
            "orderReports": []
        }"#;

        let response: CancelOrderListResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_list_id, 555555);
        assert!(matches!(response.contingency_type, ContingencyType::Oto));
        assert!(matches!(
            response.list_status_type,
            OrderListStatus::ExecStarted
        ));
        assert!(matches!(
            response.list_order_status,
            OrderListOrderStatus::Executing
        ));
        assert_eq!(response.list_client_order_id, "oto-order-789");
        assert_eq!(response.symbol, "BNBUSDT");
        assert_eq!(response.orders.len(), 2);
    }

    #[test]
    fn test_cancel_order_list_response_otoco_type() {
        // Test response with OTOCO contingency type
        let json = r#"{
            "orderListId": 777777,
            "contingencyType": "OTOCO",
            "listStatusType": "RESPONSE",
            "listOrderStatus": "REJECT",
            "listClientOrderId": "otoco-order-complex",
            "transactionTime": 1621930000000,
            "symbol": "ADAUSDT",
            "orders": [
                {
                    "symbol": "ADAUSDT",
                    "orderId": 666666,
                    "clientOrderId": "primary"
                },
                {
                    "symbol": "ADAUSDT",
                    "orderId": 777777,
                    "clientOrderId": "oco-1"
                },
                {
                    "symbol": "ADAUSDT",
                    "orderId": 888888,
                    "clientOrderId": "oco-2"
                }
            ],
            "orderReports": [
                {"type": "TRADE", "price": "1.5"}
            ]
        }"#;

        let response: CancelOrderListResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_list_id, 777777);
        assert!(matches!(response.contingency_type, ContingencyType::Otoco));
        assert!(matches!(
            response.list_status_type,
            OrderListStatus::Response
        ));
        assert!(matches!(
            response.list_order_status,
            OrderListOrderStatus::Reject
        ));
        assert_eq!(response.list_client_order_id, "otoco-order-complex");
        assert_eq!(response.symbol, "ADAUSDT");
        assert_eq!(response.orders.len(), 3);
        assert_eq!(response.order_reports.len(), 1);
    }

    #[test]
    fn test_cancel_order_list_response_with_order_reports() {
        // Test response with order reports
        let json = r#"{
            "orderListId": 999999,
            "contingencyType": "OCO",
            "listStatusType": "REJECT",
            "listOrderStatus": "REJECT",
            "listClientOrderId": "failed-oco",
            "transactionTime": 1621940000000,
            "symbol": "XRPUSDT",
            "orders": [
                {
                    "symbol": "XRPUSDT",
                    "orderId": 111111,
                    "clientOrderId": "oco-buy"
                },
                {
                    "symbol": "XRPUSDT",
                    "orderId": 222222,
                    "clientOrderId": "oco-sell"
                }
            ],
            "orderReports": [
                {
                    "symbol": "XRPUSDT",
                    "orderId": 111111,
                    "orderListId": 999999,
                    "clientOrderId": "oco-buy",
                    "transactTime": 1621940000000,
                    "price": "0.5000",
                    "origQty": "100",
                    "executedQty": "0",
                    "cummulativeQuoteQty": "0",
                    "status": "CANCELED",
                    "timeInForce": "GTC",
                    "type": "LIMIT_MAKER",
                    "side": "BUY"
                },
                {
                    "symbol": "XRPUSDT",
                    "orderId": 222222,
                    "orderListId": 999999,
                    "clientOrderId": "oco-sell",
                    "transactTime": 1621940000000,
                    "price": "0.6000",
                    "origQty": "100",
                    "executedQty": "0",
                    "cummulativeQuoteQty": "0",
                    "status": "CANCELED",
                    "timeInForce": "GTC",
                    "type": "LIMIT_MAKER",
                    "side": "SELL"
                }
            ]
        }"#;

        let response: CancelOrderListResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_list_id, 999999);
        assert!(matches!(response.contingency_type, ContingencyType::Oco));
        assert!(matches!(response.list_status_type, OrderListStatus::Reject));
        assert!(matches!(
            response.list_order_status,
            OrderListOrderStatus::Reject
        ));
        assert_eq!(response.list_client_order_id, "failed-oco");
        assert_eq!(response.symbol, "XRPUSDT");
        assert_eq!(response.orders.len(), 2);
        assert_eq!(response.order_reports.len(), 2);

        // Verify the order reports are valid JSON values
        assert!(response.order_reports[0].is_object());
        assert!(response.order_reports[1].is_object());
    }

    #[test]
    fn test_cancel_order_list_response_empty_orders() {
        // Test response with empty orders array (edge case)
        let json = r#"{
            "orderListId": 0,
            "contingencyType": "OCO",
            "listStatusType": "ALL_DONE",
            "listOrderStatus": "ALL_DONE",
            "listClientOrderId": "",
            "transactionTime": 0,
            "symbol": "BTCUSDT",
            "orders": [],
            "orderReports": []
        }"#;

        let response: CancelOrderListResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.order_list_id, 0);
        assert!(matches!(response.contingency_type, ContingencyType::Oco));
        assert!(matches!(
            response.list_status_type,
            OrderListStatus::AllDone
        ));
        assert!(matches!(
            response.list_order_status,
            OrderListOrderStatus::AllDone
        ));
        assert_eq!(response.list_client_order_id, "");
        assert_eq!(response.transaction_time, 0);
        assert_eq!(response.symbol, "BTCUSDT");
        assert_eq!(response.orders.len(), 0);
        assert_eq!(response.order_reports.len(), 0);
    }

    #[test]
    fn test_cancel_order_list_response_all_contingency_types() {
        // Test all contingency type enum values
        let contingency_types = vec![
            ("OCO", ContingencyType::Oco),
            ("OTO", ContingencyType::Oto),
            ("OTOCO", ContingencyType::Otoco),
        ];

        for (json_value, expected_enum) in contingency_types {
            let json = format!(
                r#"{{
                    "orderListId": 123,
                    "contingencyType": "{}",
                    "listStatusType": "RESPONSE",
                    "listOrderStatus": "EXECUTING",
                    "listClientOrderId": "test",
                    "transactionTime": 1000000,
                    "symbol": "BTCUSDT",
                    "orders": [],
                    "orderReports": []
                }}"#,
                json_value
            );

            let response: CancelOrderListResponse = serde_json::from_str(&json).unwrap();
            match (json_value, expected_enum) {
                ("OCO", ContingencyType::Oco) => {
                    assert!(matches!(response.contingency_type, ContingencyType::Oco))
                }
                ("OTO", ContingencyType::Oto) => {
                    assert!(matches!(response.contingency_type, ContingencyType::Oto))
                }
                ("OTOCO", ContingencyType::Otoco) => {
                    assert!(matches!(response.contingency_type, ContingencyType::Otoco))
                }
                _ => panic!("Unexpected contingency type"),
            }
        }
    }

    #[test]
    fn test_cancel_order_list_response_all_status_types() {
        // Test all OrderListStatus enum values
        let status_types = vec![
            ("RESPONSE", OrderListStatus::Response),
            ("EXEC_STARTED", OrderListStatus::ExecStarted),
            ("ALL_DONE", OrderListStatus::AllDone),
            ("REJECT", OrderListStatus::Reject),
        ];

        for (json_value, expected_enum) in status_types {
            let json = format!(
                r#"{{
                    "orderListId": 123,
                    "contingencyType": "OCO",
                    "listStatusType": "{}",
                    "listOrderStatus": "EXECUTING",
                    "listClientOrderId": "test",
                    "transactionTime": 1000000,
                    "symbol": "BTCUSDT",
                    "orders": [],
                    "orderReports": []
                }}"#,
                json_value
            );

            let response: CancelOrderListResponse = serde_json::from_str(&json).unwrap();
            match (json_value, expected_enum) {
                ("RESPONSE", OrderListStatus::Response) => assert!(matches!(
                    response.list_status_type,
                    OrderListStatus::Response
                )),
                ("EXEC_STARTED", OrderListStatus::ExecStarted) => assert!(matches!(
                    response.list_status_type,
                    OrderListStatus::ExecStarted
                )),
                ("ALL_DONE", OrderListStatus::AllDone) => assert!(matches!(
                    response.list_status_type,
                    OrderListStatus::AllDone
                )),
                ("REJECT", OrderListStatus::Reject) => {
                    assert!(matches!(response.list_status_type, OrderListStatus::Reject))
                }
                _ => panic!("Unexpected list status type"),
            }
        }
    }

    #[test]
    fn test_cancel_order_list_response_all_order_status_types() {
        // Test all OrderListOrderStatus enum values
        let order_status_types = vec![
            ("EXECUTING", OrderListOrderStatus::Executing),
            ("ALL_DONE", OrderListOrderStatus::AllDone),
            ("REJECT", OrderListOrderStatus::Reject),
        ];

        for (json_value, expected_enum) in order_status_types {
            let json = format!(
                r#"{{
                    "orderListId": 123,
                    "contingencyType": "OCO",
                    "listStatusType": "RESPONSE",
                    "listOrderStatus": "{}",
                    "listClientOrderId": "test",
                    "transactionTime": 1000000,
                    "symbol": "BTCUSDT",
                    "orders": [],
                    "orderReports": []
                }}"#,
                json_value
            );

            let response: CancelOrderListResponse = serde_json::from_str(&json).unwrap();
            match (json_value, expected_enum) {
                ("EXECUTING", OrderListOrderStatus::Executing) => assert!(matches!(
                    response.list_order_status,
                    OrderListOrderStatus::Executing
                )),
                ("ALL_DONE", OrderListOrderStatus::AllDone) => assert!(matches!(
                    response.list_order_status,
                    OrderListOrderStatus::AllDone
                )),
                ("REJECT", OrderListOrderStatus::Reject) => assert!(matches!(
                    response.list_order_status,
                    OrderListOrderStatus::Reject
                )),
                _ => panic!("Unexpected order list order status"),
            }
        }
    }

    #[test]
    fn test_cancel_order_list_request_large_values() {
        // Test with maximum/large values
        let request = CancelOrderListRequest {
            symbol: "BTCUSDT".to_string(),
            order_list_id: Some(u64::MAX),
            list_client_order_id: Some("a".repeat(36)), // Binance typically allows up to 36 chars
            new_client_order_id: Some("b".repeat(36)),
            recv_window: Some(60000), // Maximum recv window
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["symbol"], "BTCUSDT");
        assert_eq!(json["orderListId"], u64::MAX);
        assert_eq!(json["listClientOrderId"], "a".repeat(36));
        assert_eq!(json["newClientOrderId"], "b".repeat(36));
        assert_eq!(json["recvWindow"], 60000);
    }

    #[test]
    fn test_cancel_order_list_order_multiple_orders() {
        // Test deserialization of multiple orders with different properties
        let json = r#"[
            {
                "symbol": "BTCUSDT",
                "orderId": 1,
                "clientOrderId": "first"
            },
            {
                "symbol": "ETHUSDT",
                "orderId": 999999999999999999,
                "clientOrderId": "very-long-client-order-id-that-tests-limits"
            },
            {
                "symbol": "BNBUSDT",
                "orderId": 12345,
                "clientOrderId": "special-chars_123.456"
            }
        ]"#;

        let orders: Vec<CancelOrderListOrder> = serde_json::from_str(json).unwrap();

        assert_eq!(orders.len(), 3);

        assert_eq!(orders[0].symbol, "BTCUSDT");
        assert_eq!(orders[0].order_id, 1);
        assert_eq!(orders[0].client_order_id, "first");

        assert_eq!(orders[1].symbol, "ETHUSDT");
        assert_eq!(orders[1].order_id, 999999999999999999);
        assert_eq!(
            orders[1].client_order_id,
            "very-long-client-order-id-that-tests-limits"
        );

        assert_eq!(orders[2].symbol, "BNBUSDT");
        assert_eq!(orders[2].order_id, 12345);
        assert_eq!(orders[2].client_order_id, "special-chars_123.456");
    }
}
