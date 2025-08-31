use serde::{Deserialize, Serialize};

use crate::binance::spot::{
    ContingencyType, OrderListOrderStatus, OrderListStatus, RestResult, private_client::RestClient,
};

const GET_ALL_ORDERLIST_ENDPOINT: &str = "/api/v3/allOrderList";

/// Request parameters for getting all order lists
#[derive(Debug, Clone, Serialize, Default)]
pub struct AllOrderListRequest {
    /// From ID
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,

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

/// All order list information
#[derive(Debug, Clone, Deserialize)]
pub struct AllOrderList {
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
    pub orders: Vec<AllOrderListOrder>,
}

/// Order information in the all order list
#[derive(Debug, Clone, Deserialize)]
pub struct AllOrderListOrder {
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
    /// Retrieve all order lists
    ///
    /// Retrieve all order lists.
    ///
    /// [docs](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/public-api-endpoints#query-all-order-lists--user_data)
    ///
    /// Method: GET /api/v3/allOrderList
    /// Weight: 20
    /// Security: USER_DATA
    pub async fn get_all_order_lists(
        &self,
        params: Option<AllOrderListRequest>,
    ) -> RestResult<Vec<AllOrderList>> {
        self.send_get_signed_request(
            GET_ALL_ORDERLIST_ENDPOINT,
            params.unwrap_or_default(),
            20,
            false,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_order_list_request_serialization_empty() {
        let request = AllOrderListRequest::default();

        // None of the fields are set, so serialization should produce empty query
        let query_result = serde_urlencoded::to_string(&request);
        assert!(query_result.is_ok());
        let query_string = query_result.unwrap();
        assert!(query_string.is_empty());
    }

    #[test]
    fn test_all_order_list_request_serialization_with_from_id() {
        let request = AllOrderListRequest {
            from_id: Some(12345),
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert!(query_string.contains("fromId=12345"));
        assert!(!query_string.contains("startTime"));
        assert!(!query_string.contains("endTime"));
        assert!(!query_string.contains("limit"));
        assert!(!query_string.contains("recvWindow"));
    }

    #[test]
    fn test_all_order_list_request_serialization_with_time_range() {
        let request = AllOrderListRequest {
            from_id: None,
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            limit: None,
            recv_window: None,
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert!(!query_string.contains("fromId"));
        assert!(query_string.contains("startTime=1625184000000"));
        assert!(query_string.contains("endTime=1625270400000"));
        assert!(!query_string.contains("limit"));
        assert!(!query_string.contains("recvWindow"));
    }

    #[test]
    fn test_all_order_list_request_serialization_full() {
        let request = AllOrderListRequest {
            from_id: Some(99999),
            start_time: Some(1625184000000),
            end_time: Some(1625270400000),
            limit: Some(500),
            recv_window: Some(5000),
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert!(query_string.contains("fromId=99999"));
        assert!(query_string.contains("startTime=1625184000000"));
        assert!(query_string.contains("endTime=1625270400000"));
        assert!(query_string.contains("limit=500"));
        assert!(query_string.contains("recvWindow=5000"));
    }

    #[test]
    fn test_all_order_list_request_serialization_limit_bounds() {
        // Test with maximum limit
        let request = AllOrderListRequest {
            from_id: None,
            start_time: None,
            end_time: None,
            limit: Some(1000),
            recv_window: None,
        };

        let query_string = serde_urlencoded::to_string(&request).unwrap();
        assert!(query_string.contains("limit=1000"));
    }

    #[test]
    fn test_all_order_list_order_deserialization() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "orderId": 28457,
            "clientOrderId": "ABC123"
        }"#;

        let order: AllOrderListOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.order_id, 28457);
        assert_eq!(order.client_order_id, "ABC123");
    }

    #[test]
    fn test_all_order_list_deserialization_oco() {
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
                    "clientOrderId": "order1"
                },
                {
                    "symbol": "BTCUSDT",
                    "orderId": 100002,
                    "clientOrderId": "order2"
                }
            ]
        }"#;

        let order_list: AllOrderList = serde_json::from_str(json).unwrap();
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

        assert_eq!(order_list.orders[0].symbol, "BTCUSDT");
        assert_eq!(order_list.orders[0].order_id, 100001);
        assert_eq!(order_list.orders[0].client_order_id, "order1");

        assert_eq!(order_list.orders[1].symbol, "BTCUSDT");
        assert_eq!(order_list.orders[1].order_id, 100002);
        assert_eq!(order_list.orders[1].client_order_id, "order2");
    }

    #[test]
    fn test_all_order_list_deserialization_oto() {
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
                    "clientOrderId": "primaryOrder"
                },
                {
                    "symbol": "ETHUSDT",
                    "orderId": 200002,
                    "clientOrderId": "secondaryOrder"
                }
            ]
        }"#;

        let order_list: AllOrderList = serde_json::from_str(json).unwrap();
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
    }

    #[test]
    fn test_all_order_list_deserialization_otoco() {
        let json = r#"{
            "orderListId": 345678,
            "contingencyType": "OTOCO",
            "listStatusType": "REJECT",
            "listOrderStatus": "REJECT",
            "listClientOrderId": "myOTOCOList1",
            "transactionTime": 1625356800000,
            "symbol": "BNBUSDT",
            "orders": [
                {
                    "symbol": "BNBUSDT",
                    "orderId": 300001,
                    "clientOrderId": "primaryOTOCO"
                },
                {
                    "symbol": "BNBUSDT",
                    "orderId": 300002,
                    "clientOrderId": "ocoLeg1"
                },
                {
                    "symbol": "BNBUSDT",
                    "orderId": 300003,
                    "clientOrderId": "ocoLeg2"
                }
            ]
        }"#;

        let order_list: AllOrderList = serde_json::from_str(json).unwrap();
        assert_eq!(order_list.order_list_id, 345678);
        assert!(matches!(
            order_list.contingency_type,
            ContingencyType::Otoco
        ));
        assert!(matches!(
            order_list.list_status_type,
            OrderListStatus::Reject
        ));
        assert!(matches!(
            order_list.list_order_status,
            OrderListOrderStatus::Reject
        ));
        assert_eq!(order_list.list_client_order_id, "myOTOCOList1");
        assert_eq!(order_list.transaction_time, 1625356800000);
        assert_eq!(order_list.symbol, "BNBUSDT");
        assert_eq!(order_list.orders.len(), 3);
    }

    #[test]
    fn test_all_order_list_array_deserialization_empty() {
        let json = "[]";
        let order_lists: Vec<AllOrderList> = serde_json::from_str(json).unwrap();
        assert_eq!(order_lists.len(), 0);
    }

    #[test]
    fn test_all_order_list_array_deserialization_multiple() {
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
                        "clientOrderId": "order1a"
                    },
                    {
                        "symbol": "BTCUSDT",
                        "orderId": 1002,
                        "clientOrderId": "order1b"
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
                        "clientOrderId": "order2a"
                    },
                    {
                        "symbol": "ETHUSDT",
                        "orderId": 2002,
                        "clientOrderId": "order2b"
                    }
                ]
            }
        ]"#;

        let order_lists: Vec<AllOrderList> = serde_json::from_str(json).unwrap();
        assert_eq!(order_lists.len(), 2);

        // First order list
        assert_eq!(order_lists[0].order_list_id, 111111);
        assert!(matches!(
            order_lists[0].contingency_type,
            ContingencyType::Oco
        ));
        assert!(matches!(
            order_lists[0].list_status_type,
            OrderListStatus::AllDone
        ));
        assert_eq!(order_lists[0].symbol, "BTCUSDT");
        assert_eq!(order_lists[0].orders.len(), 2);

        // Second order list
        assert_eq!(order_lists[1].order_list_id, 222222);
        assert!(matches!(
            order_lists[1].contingency_type,
            ContingencyType::Oto
        ));
        assert!(matches!(
            order_lists[1].list_status_type,
            OrderListStatus::Response
        ));
        assert_eq!(order_lists[1].symbol, "ETHUSDT");
        assert_eq!(order_lists[1].orders.len(), 2);
    }

    #[test]
    fn test_all_order_list_different_statuses() {
        let statuses = vec![
            ("RESPONSE", OrderListStatus::Response),
            ("EXEC_STARTED", OrderListStatus::ExecStarted),
            ("ALL_DONE", OrderListStatus::AllDone),
            ("REJECT", OrderListStatus::Reject),
        ];

        for (status_str, expected_status) in statuses {
            let json = format!(
                r#"{{
                    "orderListId": 999999,
                    "contingencyType": "OCO",
                    "listStatusType": "{}",
                    "listOrderStatus": "ALL_DONE",
                    "listClientOrderId": "testStatus",
                    "transactionTime": 1625184000000,
                    "symbol": "BTCUSDT",
                    "orders": []
                }}"#,
                status_str
            );

            let order_list: AllOrderList = serde_json::from_str(&json).unwrap();
            match expected_status {
                OrderListStatus::Response => assert!(matches!(
                    order_list.list_status_type,
                    OrderListStatus::Response
                )),
                OrderListStatus::ExecStarted => assert!(matches!(
                    order_list.list_status_type,
                    OrderListStatus::ExecStarted
                )),
                OrderListStatus::AllDone => assert!(matches!(
                    order_list.list_status_type,
                    OrderListStatus::AllDone
                )),
                OrderListStatus::Reject => assert!(matches!(
                    order_list.list_status_type,
                    OrderListStatus::Reject
                )),
            }
        }
    }

    #[test]
    fn test_all_order_list_different_order_statuses() {
        let order_statuses = vec![
            ("EXECUTING", OrderListOrderStatus::Executing),
            ("ALL_DONE", OrderListOrderStatus::AllDone),
            ("REJECT", OrderListOrderStatus::Reject),
        ];

        for (status_str, expected_status) in order_statuses {
            let json = format!(
                r#"{{
                    "orderListId": 888888,
                    "contingencyType": "OTO",
                    "listStatusType": "RESPONSE",
                    "listOrderStatus": "{}",
                    "listClientOrderId": "testOrderStatus",
                    "transactionTime": 1625184000000,
                    "symbol": "ETHUSDT",
                    "orders": []
                }}"#,
                status_str
            );

            let order_list: AllOrderList = serde_json::from_str(&json).unwrap();
            match expected_status {
                OrderListOrderStatus::Executing => assert!(matches!(
                    order_list.list_order_status,
                    OrderListOrderStatus::Executing
                )),
                OrderListOrderStatus::AllDone => assert!(matches!(
                    order_list.list_order_status,
                    OrderListOrderStatus::AllDone
                )),
                OrderListOrderStatus::Reject => assert!(matches!(
                    order_list.list_order_status,
                    OrderListOrderStatus::Reject
                )),
            }
        }
    }

    #[test]
    fn test_all_order_list_deserialization_with_empty_orders() {
        let json = r#"{
            "orderListId": 777777,
            "contingencyType": "OCO",
            "listStatusType": "REJECT",
            "listOrderStatus": "REJECT",
            "listClientOrderId": "rejectedList",
            "transactionTime": 1625443200000,
            "symbol": "ADAUSDT",
            "orders": []
        }"#;

        let order_list: AllOrderList = serde_json::from_str(json).unwrap();
        assert_eq!(order_list.order_list_id, 777777);
        assert_eq!(order_list.orders.len(), 0);
        assert!(order_list.orders.is_empty());
    }

    #[test]
    fn test_all_order_list_request_default() {
        let request = AllOrderListRequest::default();
        assert!(request.from_id.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
        assert!(request.limit.is_none());
        assert!(request.recv_window.is_none());
    }
}
