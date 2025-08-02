use serde::{Deserialize, Serialize};

use super::RestClient;

const CANCEL_BATCH_ORDERS_ENDPOINT: &str = "/spot/cancel_batch_orders";

/// Request to cancel batch orders
#[derive(Debug, Clone, Serialize)]
pub struct CancelBatchOrdersRequest {
    /// List of order IDs to cancel
    pub order_ids: Vec<String>,
}

/// Cancel batch orders response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelBatchOrdersResponse {
    /// Successfully cancelled order IDs
    pub succeeded: Vec<String>,

    /// Failed order cancellations with error details
    pub failed: Vec<CancelBatchOrderError>,
}

/// Failed batch order cancellation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelBatchOrderError {
    /// Order ID that failed to cancel
    pub id: String,

    /// Error message
    pub message: String,

    /// Error code
    pub code: String,
}

impl RestClient {
    /// Cancel multiple orders in batch
    ///
    /// This endpoint allows cancelling multiple orders at once. It returns
    /// information about which orders were successfully cancelled and which failed.
    ///
    /// # API Documentation
    /// <https://www.gate.com/docs/developers/apiv4/#cancel-a-batch-of-orders-with-an-id-list>
    pub async fn cancel_batch_orders(
        &self,
        request: CancelBatchOrdersRequest,
    ) -> crate::gateio::spot::Result<CancelBatchOrdersResponse> {
        self.post(CANCEL_BATCH_ORDERS_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_batch_orders_request_single_order() {
        let request = CancelBatchOrdersRequest {
            order_ids: vec!["12345678".to_string()],
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["order_ids"].is_array());
        assert_eq!(json["order_ids"].as_array().unwrap().len(), 1);
        assert_eq!(json["order_ids"][0], "12345678");
    }

    #[test]
    fn test_cancel_batch_orders_request_multiple_orders() {
        let request = CancelBatchOrdersRequest {
            order_ids: vec![
                "12345678".to_string(),
                "87654321".to_string(),
                "11111111".to_string(),
                "22222222".to_string(),
            ],
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["order_ids"].is_array());
        let order_ids = json["order_ids"].as_array().unwrap();
        assert_eq!(order_ids.len(), 4);
        assert_eq!(order_ids[0], "12345678");
        assert_eq!(order_ids[1], "87654321");
        assert_eq!(order_ids[2], "11111111");
        assert_eq!(order_ids[3], "22222222");
    }

    #[test]
    fn test_cancel_batch_orders_request_empty_list() {
        let request = CancelBatchOrdersRequest { order_ids: vec![] };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["order_ids"].is_array());
        assert_eq!(json["order_ids"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_cancel_batch_orders_request_large_batch() {
        let mut order_ids = Vec::new();
        for i in 1..=50 {
            order_ids.push(format!("order_{:03}", i));
        }

        let request = CancelBatchOrdersRequest { order_ids };

        let json = serde_json::to_value(&request).unwrap();
        let order_ids_array = json["order_ids"].as_array().unwrap();
        assert_eq!(order_ids_array.len(), 50);
        assert_eq!(order_ids_array[0], "order_001");
        assert_eq!(order_ids_array[49], "order_050");
    }

    #[test]
    fn test_cancel_batch_orders_request_mixed_order_id_formats() {
        let request = CancelBatchOrdersRequest {
            order_ids: vec![
                "12345678".to_string(),       // Numeric ID
                "abc123def".to_string(),      // Alphanumeric ID
                "order_uuid_456".to_string(), // UUID-style ID
                "9876543210".to_string(),     // Long numeric ID
            ],
        };

        let json = serde_json::to_value(&request).unwrap();
        let order_ids = json["order_ids"].as_array().unwrap();
        assert_eq!(order_ids[0], "12345678");
        assert_eq!(order_ids[1], "abc123def");
        assert_eq!(order_ids[2], "order_uuid_456");
        assert_eq!(order_ids[3], "9876543210");
    }

    #[test]
    fn test_cancel_batch_orders_response_all_succeeded() {
        let json = r#"{
            "succeeded": ["12345678", "87654321", "11111111"],
            "failed": []
        }"#;

        let response: CancelBatchOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.succeeded.len(), 3);
        assert_eq!(response.failed.len(), 0);
        assert!(response.succeeded.contains(&"12345678".to_string()));
        assert!(response.succeeded.contains(&"87654321".to_string()));
        assert!(response.succeeded.contains(&"11111111".to_string()));
    }

    #[test]
    fn test_cancel_batch_orders_response_all_failed() {
        let json = r#"{
            "succeeded": [],
            "failed": [
                {
                    "id": "12345678",
                    "message": "Order not found",
                    "code": "ORDER_NOT_FOUND"
                },
                {
                    "id": "87654321",
                    "message": "Order already cancelled",
                    "code": "ORDER_ALREADY_CANCELLED"
                }
            ]
        }"#;

        let response: CancelBatchOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.succeeded.len(), 0);
        assert_eq!(response.failed.len(), 2);

        assert_eq!(response.failed[0].id, "12345678");
        assert_eq!(response.failed[0].message, "Order not found");
        assert_eq!(response.failed[0].code, "ORDER_NOT_FOUND");

        assert_eq!(response.failed[1].id, "87654321");
        assert_eq!(response.failed[1].message, "Order already cancelled");
        assert_eq!(response.failed[1].code, "ORDER_ALREADY_CANCELLED");
    }

    #[test]
    fn test_cancel_batch_orders_response_mixed_results() {
        let json = r#"{
            "succeeded": ["12345678", "11111111"],
            "failed": [
                {
                    "id": "87654321",
                    "message": "Order not found",
                    "code": "ORDER_NOT_FOUND"
                },
                {
                    "id": "22222222",
                    "message": "Insufficient permissions",
                    "code": "PERMISSION_DENIED"
                }
            ]
        }"#;

        let response: CancelBatchOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.succeeded.len(), 2);
        assert_eq!(response.failed.len(), 2);

        // Check succeeded orders
        assert!(response.succeeded.contains(&"12345678".to_string()));
        assert!(response.succeeded.contains(&"11111111".to_string()));

        // Check failed orders
        assert_eq!(response.failed[0].id, "87654321");
        assert_eq!(response.failed[0].code, "ORDER_NOT_FOUND");
        assert_eq!(response.failed[1].id, "22222222");
        assert_eq!(response.failed[1].code, "PERMISSION_DENIED");
    }

    #[test]
    fn test_cancel_batch_order_error_structure() {
        let json = r#"{
            "id": "12345678",
            "message": "Order cannot be cancelled in current state",
            "code": "INVALID_ORDER_STATE"
        }"#;

        let error: CancelBatchOrderError = serde_json::from_str(json).unwrap();
        assert_eq!(error.id, "12345678");
        assert_eq!(error.message, "Order cannot be cancelled in current state");
        assert_eq!(error.code, "INVALID_ORDER_STATE");
    }

    #[test]
    fn test_cancel_batch_orders_response_empty_results() {
        let json = r#"{
            "succeeded": [],
            "failed": []
        }"#;

        let response: CancelBatchOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.succeeded.len(), 0);
        assert_eq!(response.failed.len(), 0);
    }

    #[test]
    fn test_cancel_batch_orders_request_realistic_stop_loss_scenario() {
        // Scenario: Cancelling multiple stop-loss orders for risk management
        let request = CancelBatchOrdersRequest {
            order_ids: vec![
                "sl_btc_001".to_string(), // BTC stop-loss
                "sl_eth_001".to_string(), // ETH stop-loss
                "sl_bnb_001".to_string(), // BNB stop-loss
                "sl_sol_001".to_string(), // SOL stop-loss
            ],
        };

        let json = serde_json::to_value(&request).unwrap();
        let order_ids = json["order_ids"].as_array().unwrap();
        assert_eq!(order_ids.len(), 4);
        assert!(
            order_ids
                .iter()
                .all(|id| id.as_str().unwrap().starts_with("sl_"))
        );
    }

    #[test]
    fn test_cancel_batch_orders_request_realistic_grid_cancel_scenario() {
        // Scenario: Cancelling grid trading orders
        let mut order_ids = Vec::new();
        for i in 1..=10 {
            order_ids.push(format!("grid_btc_level_{}", i));
        }

        let request = CancelBatchOrdersRequest { order_ids };

        let json = serde_json::to_value(&request).unwrap();
        let order_ids_array = json["order_ids"].as_array().unwrap();
        assert_eq!(order_ids_array.len(), 10);

        for (i, order_id) in order_ids_array.iter().enumerate() {
            let expected = format!("grid_btc_level_{}", i + 1);
            assert_eq!(order_id.as_str().unwrap(), expected);
        }
    }

    #[test]
    fn test_cancel_batch_orders_request_realistic_portfolio_rebalance_scenario() {
        // Scenario: Cancelling all open orders before portfolio rebalancing
        let request = CancelBatchOrdersRequest {
            order_ids: vec![
                "buy_btc_30000".to_string(),
                "sell_btc_32000".to_string(),
                "buy_eth_2400".to_string(),
                "sell_eth_2600".to_string(),
                "buy_bnb_300".to_string(),
                "sell_bnb_320".to_string(),
                "dca_order_1".to_string(),
                "dca_order_2".to_string(),
            ],
        };

        let json = serde_json::to_value(&request).unwrap();
        let order_ids = json["order_ids"].as_array().unwrap();
        assert_eq!(order_ids.len(), 8);

        // Verify mix of buy/sell orders and DCA orders
        let order_strings: Vec<String> = order_ids
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert!(order_strings.iter().any(|id| id.contains("buy_")));
        assert!(order_strings.iter().any(|id| id.contains("sell_")));
        assert!(order_strings.iter().any(|id| id.contains("dca_")));
    }

    #[test]
    fn test_cancel_batch_orders_response_realistic_partial_failure_scenario() {
        let json = r#"{
            "succeeded": [
                "buy_btc_30000",
                "sell_eth_2600",
                "dca_order_1"
            ],
            "failed": [
                {
                    "id": "sell_btc_32000",
                    "message": "Order already filled",
                    "code": "ORDER_FILLED"
                },
                {
                    "id": "buy_eth_2400",
                    "message": "Order not found",
                    "code": "ORDER_NOT_FOUND"
                },
                {
                    "id": "expired_order_123",
                    "message": "Order has expired",
                    "code": "ORDER_EXPIRED"
                }
            ]
        }"#;

        let response: CancelBatchOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.succeeded.len(), 3);
        assert_eq!(response.failed.len(), 3);

        // Verify successful cancellations
        assert!(response.succeeded.contains(&"buy_btc_30000".to_string()));
        assert!(response.succeeded.contains(&"sell_eth_2600".to_string()));
        assert!(response.succeeded.contains(&"dca_order_1".to_string()));

        // Verify different failure reasons
        let failure_codes: Vec<&str> = response.failed.iter().map(|f| f.code.as_str()).collect();
        assert!(failure_codes.contains(&"ORDER_FILLED"));
        assert!(failure_codes.contains(&"ORDER_NOT_FOUND"));
        assert!(failure_codes.contains(&"ORDER_EXPIRED"));
    }

    #[test]
    fn test_cancel_batch_orders_response_common_error_scenarios() {
        let json = r#"{
            "succeeded": ["valid_order_1"],
            "failed": [
                {
                    "id": "filled_order",
                    "message": "Order already completely filled",
                    "code": "ORDER_FILLED"
                },
                {
                    "id": "cancelled_order",
                    "message": "Order already cancelled",
                    "code": "ORDER_CANCELLED"
                },
                {
                    "id": "expired_order",
                    "message": "Order expired",
                    "code": "ORDER_EXPIRED"
                },
                {
                    "id": "nonexistent_order",
                    "message": "Order does not exist",
                    "code": "ORDER_NOT_FOUND"
                },
                {
                    "id": "wrong_user_order",
                    "message": "Access denied",
                    "code": "ACCESS_DENIED"
                }
            ]
        }"#;

        let response: CancelBatchOrdersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.succeeded.len(), 1);
        assert_eq!(response.failed.len(), 5);

        // Verify all common error codes are handled
        let error_codes: Vec<&str> = response.failed.iter().map(|f| f.code.as_str()).collect();
        assert!(error_codes.contains(&"ORDER_FILLED"));
        assert!(error_codes.contains(&"ORDER_CANCELLED"));
        assert!(error_codes.contains(&"ORDER_EXPIRED"));
        assert!(error_codes.contains(&"ORDER_NOT_FOUND"));
        assert!(error_codes.contains(&"ACCESS_DENIED"));
    }

    #[test]
    fn test_cancel_batch_orders_request_duplicate_order_ids() {
        let request = CancelBatchOrdersRequest {
            order_ids: vec![
                "12345678".to_string(),
                "87654321".to_string(),
                "12345678".to_string(), // Duplicate
                "11111111".to_string(),
                "87654321".to_string(), // Another duplicate
            ],
        };

        let json = serde_json::to_value(&request).unwrap();
        let order_ids = json["order_ids"].as_array().unwrap();
        assert_eq!(order_ids.len(), 5); // Should include duplicates in request

        // Count occurrences
        let id_count = order_ids.iter().filter(|&id| id == "12345678").count();
        assert_eq!(id_count, 2);
    }

    #[test]
    fn test_cancel_batch_orders_request_edge_case_long_order_ids() {
        let request = CancelBatchOrdersRequest {
            order_ids: vec![
                "a".repeat(100),                                        // Very long order ID
                "1234567890123456789012345678901234567890".to_string(), // Long numeric ID
                "".to_string(),                                         // Empty order ID
                "abc".to_string(),                                      // Short order ID
            ],
        };

        let json = serde_json::to_value(&request).unwrap();
        let order_ids = json["order_ids"].as_array().unwrap();
        assert_eq!(order_ids.len(), 4);
        assert_eq!(order_ids[0].as_str().unwrap().len(), 100);
        assert_eq!(order_ids[2], "");
        assert_eq!(order_ids[3], "abc");
    }

    #[test]
    fn test_cancel_batch_orders_request_clone() {
        let original = CancelBatchOrdersRequest {
            order_ids: vec!["12345678".to_string(), "87654321".to_string()],
        };

        let cloned = original.clone();
        assert_eq!(cloned.order_ids.len(), original.order_ids.len());
        assert_eq!(cloned.order_ids[0], original.order_ids[0]);
        assert_eq!(cloned.order_ids[1], original.order_ids[1]);
    }

    #[test]
    fn test_cancel_batch_orders_response_clone() {
        let original = CancelBatchOrdersResponse {
            succeeded: vec!["12345678".to_string()],
            failed: vec![CancelBatchOrderError {
                id: "87654321".to_string(),
                message: "Test error".to_string(),
                code: "TEST_ERROR".to_string(),
            }],
        };

        let cloned = original.clone();
        assert_eq!(cloned.succeeded.len(), original.succeeded.len());
        assert_eq!(cloned.failed.len(), original.failed.len());
        assert_eq!(cloned.succeeded[0], original.succeeded[0]);
        assert_eq!(cloned.failed[0].id, original.failed[0].id);
    }

    #[test]
    fn test_cancel_batch_order_error_clone() {
        let original = CancelBatchOrderError {
            id: "12345678".to_string(),
            message: "Order not found".to_string(),
            code: "ORDER_NOT_FOUND".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.message, original.message);
        assert_eq!(cloned.code, original.code);
    }

    #[test]
    fn test_cancel_batch_orders_request_debug() {
        let request = CancelBatchOrdersRequest {
            order_ids: vec!["12345678".to_string(), "87654321".to_string()],
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("CancelBatchOrdersRequest"));
        assert!(debug_str.contains("12345678"));
        assert!(debug_str.contains("87654321"));
    }

    #[test]
    fn test_cancel_batch_orders_response_debug() {
        let response = CancelBatchOrdersResponse {
            succeeded: vec!["12345678".to_string()],
            failed: vec![CancelBatchOrderError {
                id: "87654321".to_string(),
                message: "Test error".to_string(),
                code: "TEST_ERROR".to_string(),
            }],
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("CancelBatchOrdersResponse"));
        assert!(debug_str.contains("12345678"));
        assert!(debug_str.contains("87654321"));
        assert!(debug_str.contains("Test error"));
    }

    #[test]
    fn test_cancel_batch_order_error_debug() {
        let error = CancelBatchOrderError {
            id: "12345678".to_string(),
            message: "Order not found".to_string(),
            code: "ORDER_NOT_FOUND".to_string(),
        };

        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("CancelBatchOrderError"));
        assert!(debug_str.contains("12345678"));
        assert!(debug_str.contains("Order not found"));
        assert!(debug_str.contains("ORDER_NOT_FOUND"));
    }

    #[test]
    fn test_cancel_batch_orders_request_serialization() {
        let request = CancelBatchOrdersRequest {
            order_ids: vec!["12345678".to_string(), "87654321".to_string()],
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("order_ids"));
        assert_eq!(json.as_object().unwrap().len(), 1); // Only "order_ids" field
    }

    #[test]
    fn test_cancel_batch_orders_response_serialization() {
        let response = CancelBatchOrdersResponse {
            succeeded: vec!["12345678".to_string()],
            failed: vec![CancelBatchOrderError {
                id: "87654321".to_string(),
                message: "Test error".to_string(),
                code: "TEST_ERROR".to_string(),
            }],
        };

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["succeeded"][0], "12345678");
        assert_eq!(json["failed"][0]["id"], "87654321");
        assert_eq!(json["failed"][0]["message"], "Test error");
        assert_eq!(json["failed"][0]["code"], "TEST_ERROR");
    }

    #[test]
    fn test_cancel_batch_order_error_serialization() {
        let error = CancelBatchOrderError {
            id: "12345678".to_string(),
            message: "Order not found".to_string(),
            code: "ORDER_NOT_FOUND".to_string(),
        };

        let json = serde_json::to_value(&error).unwrap();
        assert_eq!(json["id"], "12345678");
        assert_eq!(json["message"], "Order not found");
        assert_eq!(json["code"], "ORDER_NOT_FOUND");
    }

    #[test]
    fn test_cancel_batch_orders_request_endpoint_validation() {
        let request = CancelBatchOrdersRequest {
            order_ids: vec!["12345678".to_string()],
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["order_ids"].is_array());
        assert!(json.as_object().unwrap().contains_key("order_ids"));
    }

    #[test]
    fn test_cancel_batch_orders_response_round_trip() {
        let original = CancelBatchOrdersResponse {
            succeeded: vec!["12345678".to_string(), "87654321".to_string()],
            failed: vec![
                CancelBatchOrderError {
                    id: "11111111".to_string(),
                    message: "Order not found".to_string(),
                    code: "ORDER_NOT_FOUND".to_string(),
                },
                CancelBatchOrderError {
                    id: "22222222".to_string(),
                    message: "Order already cancelled".to_string(),
                    code: "ORDER_CANCELLED".to_string(),
                },
            ],
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: CancelBatchOrdersResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.succeeded.len(), original.succeeded.len());
        assert_eq!(deserialized.failed.len(), original.failed.len());
        assert_eq!(deserialized.succeeded[0], original.succeeded[0]);
        assert_eq!(deserialized.failed[0].id, original.failed[0].id);
        assert_eq!(deserialized.failed[1].message, original.failed[1].message);
    }

    #[test]
    fn test_cancel_batch_orders_request_maximum_batch_size() {
        // Test with a large number of orders (simulating API limits)
        let mut order_ids = Vec::new();
        for i in 1..=100 {
            order_ids.push(format!("order_{:03}", i));
        }

        let request = CancelBatchOrdersRequest { order_ids };

        let json = serde_json::to_value(&request).unwrap();
        let order_ids_array = json["order_ids"].as_array().unwrap();
        assert_eq!(order_ids_array.len(), 100);
        assert_eq!(order_ids_array[0], "order_001");
        assert_eq!(order_ids_array[99], "order_100");
    }
}
