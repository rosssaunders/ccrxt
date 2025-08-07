use serde::{Deserialize, Serialize};

use super::{RestClient, create_order::CreateFuturesOrderRequest};

const ENDPOINT_FUTURES_PREFIX: &str = "/futures";

/// Request to create batch orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOrdersRequest {
    /// Settlement currency
    pub settle: String,
    /// List of orders to create
    pub orders: Vec<CreateFuturesOrderRequest>,
}

/// Request to cancel batch orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCancelOrdersRequest {
    /// Settlement currency
    pub settle: String,
    /// List of order IDs to cancel
    pub order_ids: Vec<String>,
}

/// Result of batch order operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOrderResult {
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Success status
    pub succeeded: bool,
    /// Error label if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Error message if failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl RestClient {
    /// Create a batch of futures orders
    ///
    /// Creates multiple orders in a single request for improved efficiency.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#create-a-batch-of-futures-orders>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The batch order creation request parameters
    ///
    /// # Returns
    /// List of batch order results
    pub async fn create_batch_futures_orders(
        &self,
        request: BatchOrdersRequest,
    ) -> crate::gateio::perpetual::RestResult<Vec<BatchOrderResult>> {
        let endpoint = format!("{}/{}/batch_orders", ENDPOINT_FUTURES_PREFIX, request.settle);
        self.post(&endpoint, &request).await
    }

    /// Cancel a batch of futures orders
    ///
    /// Cancels multiple orders in a single request.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.io/docs/developers/apiv4/#cancel-a-batch-of-open-orders>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The batch order cancellation request parameters
    ///
    /// # Returns
    /// List of batch order results
    pub async fn cancel_batch_futures_orders(
        &self,
        request: BatchCancelOrdersRequest,
    ) -> crate::gateio::perpetual::RestResult<Vec<BatchOrderResult>> {
        let endpoint = format!("{}/{}/batch_orders", ENDPOINT_FUTURES_PREFIX, request.settle);
        self.delete_with_query(&endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_orders_request_serialization() {
        let order1 = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "BTC_USDT".to_string(),
            size: 1000,
            price: Some("43000".to_string()),
            tif: Some("gtc".to_string()),
            text: None,
            reduce_only: None,
            close: None,
            iceberg: None,
            auto_size: None,
        };

        let order2 = CreateFuturesOrderRequest {
            settle: "USDT".to_string(),
            contract: "ETH_USDT".to_string(),
            size: -2000,
            price: Some("2650".to_string()),
            tif: Some("gtc".to_string()),
            text: None,
            reduce_only: None,
            close: None,
            iceberg: None,
            auto_size: None,
        };

        let request = BatchOrdersRequest {
            settle: "USDT".to_string(),
            orders: vec![order1, order2],
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["orders"].as_array().unwrap().len(), 2);
        assert_eq!(json["orders"][0]["contract"], "BTC_USDT");
        assert_eq!(json["orders"][1]["contract"], "ETH_USDT");
    }

    #[test]
    fn test_batch_cancel_request_serialization() {
        let request = BatchCancelOrdersRequest {
            settle: "USDT".to_string(),
            order_ids: vec![
                "12345".to_string(),
                "67890".to_string(),
                "11111".to_string(),
            ],
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["order_ids"].as_array().unwrap().len(), 3);
        assert_eq!(json["order_ids"][0], "12345");
        assert_eq!(json["order_ids"][1], "67890");
        assert_eq!(json["order_ids"][2], "11111");
    }

    #[test]
    fn test_batch_order_result_deserialization_success() {
        let json = r#"{
            "id": "12345",
            "succeeded": true
        }"#;

        let result: BatchOrderResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.id, Some("12345".to_string()));
        assert!(result.succeeded);
        assert!(result.label.is_none());
        assert!(result.message.is_none());
    }

    #[test]
    fn test_batch_order_result_deserialization_failure() {
        let json = r#"{
            "succeeded": false,
            "label": "INVALID_PRICE",
            "message": "Price must be positive"
        }"#;

        let result: BatchOrderResult = serde_json::from_str(json).unwrap();
        assert!(result.id.is_none());
        assert!(!result.succeeded);
        assert_eq!(result.label, Some("INVALID_PRICE".to_string()));
        assert_eq!(result.message, Some("Price must be positive".to_string()));
    }

    #[test]
    fn test_batch_create_multiple_contracts() {
        let contracts = vec!["BTC_USDT", "ETH_USDT", "SOL_USDT", "MATIC_USDT"];
        let mut orders = Vec::new();

        for (i, contract) in contracts.iter().enumerate() {
            let order = CreateFuturesOrderRequest {
                settle: "USDT".to_string(),
                contract: contract.to_string(),
                size: (i as i64 + 1) * 1000,
                price: Some(format!("{}.0", 40000 + i * 1000)),
                tif: Some("gtc".to_string()),
                text: None,
                reduce_only: None,
                close: None,
                iceberg: None,
                auto_size: None,
            };
            orders.push(order);
        }

        let request = BatchOrdersRequest {
            settle: "USDT".to_string(),
            orders,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["orders"].as_array().unwrap().len(), 4);
    }

    #[test]
    fn test_batch_cancel_large_order_list() {
        let mut order_ids = Vec::new();
        for i in 1..=50 {
            order_ids.push(format!("order_{}", i));
        }

        let request = BatchCancelOrdersRequest {
            settle: "USDT".to_string(),
            order_ids,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["order_ids"].as_array().unwrap().len(), 50);
    }

    #[test]
    fn test_batch_result_various_error_scenarios() {
        let error_scenarios = vec![
            ("INSUFFICIENT_BALANCE", "Insufficient balance"),
            ("INVALID_PRICE", "Price must be positive"),
            ("INVALID_SIZE", "Size cannot be zero"),
            ("CONTRACT_NOT_FOUND", "Contract does not exist"),
            ("RATE_LIMIT", "Rate limit exceeded"),
        ];

        for (label, message) in error_scenarios {
            let json = format!(
                r#"{{
                    "succeeded": false,
                    "label": "{}",
                    "message": "{}"
                }}"#,
                label, message
            );

            let result: BatchOrderResult = serde_json::from_str(&json).unwrap();
            assert!(!result.succeeded);
            assert_eq!(result.label.unwrap(), label);
            assert_eq!(result.message.unwrap(), message);
        }
    }

    #[test]
    fn test_mixed_batch_results() {
        let results_json = r#"[
            {
                "id": "12345",
                "succeeded": true
            },
            {
                "succeeded": false,
                "label": "INVALID_PRICE",
                "message": "Price must be positive"
            },
            {
                "id": "67890",
                "succeeded": true
            }
        ]"#;

        let results: Vec<BatchOrderResult> = serde_json::from_str(results_json).unwrap();
        assert_eq!(results.len(), 3);

        // First result: success
        assert!(results[0].succeeded);
        assert!(results[0].id.is_some());

        // Second result: failure
        assert!(!results[1].succeeded);
        assert!(results[1].label.is_some());

        // Third result: success
        assert!(results[2].succeeded);
        assert!(results[2].id.is_some());
    }
}
