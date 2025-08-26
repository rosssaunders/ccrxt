use serde::{Deserialize, Serialize};

use super::create_order::{CreateOrderRequest, Order};
use super::{OrderSide, OrderType, StpMode, TimeInForce};
use super::{RestClient, RestResult};

const CREATE_BATCH_ORDERS_ENDPOINT: &str = "/spot/batch_orders";

/// Request parameters for creating multiple orders in a single batch operation.
///
/// This request allows submitting multiple order creation requests simultaneously,
/// improving efficiency and reducing latency compared to individual order submissions.
#[derive(Debug, Clone, Serialize)]
pub struct CreateBatchOrdersRequest {
    /// The list of individual order creation requests to process in this batch.
    /// Each order in the list will be processed according to its specific parameters.
    /// The API typically limits the maximum number of orders per batch (commonly 10).
    pub orders: Vec<CreateOrderRequest>,
}

/// Response from a single order creation within a batch operation.
///
/// Each order in the batch will have its own response indicating success or failure,
/// allowing partial batch processing where some orders succeed while others may fail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOrderResponse {
    /// Indicates whether this specific order was successfully created.
    /// True means the order was accepted and is now active in the order book.
    pub succeeded: bool,

    /// The complete order details if the order was successfully created (optional).
    /// This field contains all order information including assigned order ID, timestamps, and current status.
    /// Will be None if the order creation failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,

    /// Error message explaining why the order creation failed (optional).
    /// This field provides detailed information about validation errors, insufficient balance,
    /// or other issues that prevented the order from being created. Will be None for successful orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl RestClient {
    /// Create a batch of orders
    ///
    /// Creates multiple orders in a single API request for improved efficiency and reduced latency.
    /// This endpoint is ideal for implementing trading strategies that require multiple simultaneous
    /// orders, such as grid trading, arbitrage, or portfolio rebalancing.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#create-a-batch-of-orders)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `orders` - A vector of order creation requests to process in the batch (maximum 10 orders)
    ///
    /// # Returns
    /// A vector of batch order responses, one for each order in the request, indicating success or failure
    pub async fn spot_create_batch_orders(
        &self,
        orders: Vec<CreateOrderRequest>,
    ) -> RestResult<Vec<BatchOrderResponse>> {
        let request = CreateBatchOrdersRequest { orders };
        self.post(CREATE_BATCH_ORDERS_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_order(
        currency_pair: &str,
        side: OrderSide,
        amount: &str,
        price: &str,
    ) -> CreateOrderRequest {
        CreateOrderRequest {
            currency_pair: currency_pair.to_string(),
            side,
            amount: amount.to_string(),
            price: Some(price.to_string()),
            order_type: OrderType::Limit,
            account: None,
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            iceberg: None,
            stp_mode: None,
            text: None,
        }
    }

    #[test]
    fn test_create_batch_orders_request_single_order() {
        let order = create_sample_order("BTC_USDT", OrderSide::Buy, "0.001", "30000");
        let request = CreateBatchOrdersRequest {
            orders: vec![order],
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["orders"].is_array());
        assert_eq!(json["orders"].as_array().unwrap().len(), 1);

        let first_order = &json["orders"][0];
        assert_eq!(first_order["currency_pair"], "BTC_USDT");
        assert_eq!(first_order["side"], "buy");
        assert_eq!(first_order["amount"], "0.001");
        assert_eq!(first_order["price"], "30000");
    }

    #[test]
    fn test_create_batch_orders_request_multiple_orders() {
        let orders = vec![
            create_sample_order("BTC_USDT", OrderSide::Buy, "0.001", "30000"),
            create_sample_order("ETH_USDT", OrderSide::Sell, "0.1", "2500"),
            create_sample_order("BNB_USDT", OrderSide::Buy, "1.0", "300"),
        ];

        let request = CreateBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["orders"].is_array());
        assert_eq!(json["orders"].as_array().unwrap().len(), 3);

        // Verify each order
        let orders_array = json["orders"].as_array().unwrap();
        assert_eq!(orders_array[0]["currency_pair"], "BTC_USDT");
        assert_eq!(orders_array[1]["currency_pair"], "ETH_USDT");
        assert_eq!(orders_array[2]["currency_pair"], "BNB_USDT");
    }

    #[test]
    fn test_create_batch_orders_request_maximum_orders() {
        // Test with maximum 10 orders
        let mut orders = Vec::new();
        for i in 0..10 {
            orders.push(create_sample_order(
                "BTC_USDT",
                if i % 2 == 0 {
                    OrderSide::Buy
                } else {
                    OrderSide::Sell
                },
                "0.001",
                &format!("{}", 30000 + i * 100),
            ));
        }

        let request = CreateBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["orders"].as_array().unwrap().len(), 10);

        // Verify prices increment correctly
        let orders_array = json["orders"].as_array().unwrap();
        for (i, order) in orders_array.iter().enumerate() {
            let expected_price = format!("{}", 30000 + i * 100);
            assert_eq!(order["price"], expected_price);
        }
    }

    #[test]
    fn test_create_batch_orders_request_empty_orders() {
        let request = CreateBatchOrdersRequest { orders: vec![] };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["orders"].is_array());
        assert_eq!(json["orders"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_create_batch_orders_request_mixed_currency_pairs() {
        let orders = vec![
            create_sample_order("BTC_USDT", OrderSide::Buy, "0.001", "30000"),
            create_sample_order("ETH_BTC", OrderSide::Sell, "0.5", "0.075"),
            create_sample_order("SOL_USDC", OrderSide::Buy, "10.0", "150"),
            create_sample_order("USDC_USDT", OrderSide::Sell, "1000", "1.0001"),
        ];

        let request = CreateBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array[0]["currency_pair"], "BTC_USDT");
        assert_eq!(orders_array[1]["currency_pair"], "ETH_BTC");
        assert_eq!(orders_array[2]["currency_pair"], "SOL_USDC");
        assert_eq!(orders_array[3]["currency_pair"], "USDC_USDT");
    }

    #[test]
    fn test_create_batch_orders_request_mixed_sides() {
        let orders = vec![
            create_sample_order("BTC_USDT", OrderSide::Buy, "0.001", "30000"),
            create_sample_order("BTC_USDT", OrderSide::Sell, "0.002", "31000"),
            create_sample_order("ETH_USDT", OrderSide::Buy, "0.1", "2500"),
            create_sample_order("ETH_USDT", OrderSide::Sell, "0.15", "2600"),
        ];

        let request = CreateBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array[0]["side"], "buy");
        assert_eq!(orders_array[1]["side"], "sell");
        assert_eq!(orders_array[2]["side"], "buy");
        assert_eq!(orders_array[3]["side"], "sell");
    }

    #[test]
    fn test_batch_order_response_successful() {
        let json = r#"{
            "succeeded": true,
            "order": {
                "id": "12345678",
                "text": "batch_order_1",
                "amend_text": null,
                "create_time": "1640995200",
                "update_time": "1640995200",
                "status": "open",
                "currency_pair": "BTC_USDT",
                "type": "limit",
                "account": "spot",
                "side": "buy",
                "amount": "0.001",
                "price": "30000",
                "time_in_force": "gtc",
                "iceberg": "0",
                "left": "0.001",
                "filled_amount": "0",
                "fill_price": "0",
                "fee": "0",
                "fee_currency": "USDT",
                "point_fee": "0",
                "gt_fee": "0",
                "gt_discount": false,
                "rebated_fee": "0",
                "rebated_fee_currency": "USDT"
            }
        }"#;

        let response: BatchOrderResponse = serde_json::from_str(json).unwrap();
        assert!(response.succeeded);
        assert!(response.order.is_some());
        assert!(response.message.is_none());

        let order = response.order.unwrap();
        assert_eq!(order.id, "12345678");
        assert_eq!(order.currency_pair, "BTC_USDT");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.amount, "0.001");
        assert_eq!(order.price, "30000");
    }

    #[test]
    fn test_batch_order_response_failed() {
        let json = r#"{
            "succeeded": false,
            "message": "Insufficient balance"
        }"#;

        let response: BatchOrderResponse = serde_json::from_str(json).unwrap();
        assert!(!response.succeeded);
        assert!(response.order.is_none());
        assert!(response.message.is_some());
        assert_eq!(response.message.unwrap(), "Insufficient balance");
    }

    #[test]
    fn test_batch_order_response_array() {
        let json = r#"[
            {
                "succeeded": true,
                "order": {
                    "id": "12345678",
                    "text": "batch_order_1",
                    "amend_text": null,
                    "create_time": "1640995200",
                    "update_time": "1640995200",
                    "status": "open",
                    "currency_pair": "BTC_USDT",
                    "type": "limit",
                    "account": "spot",
                    "side": "buy",
                    "amount": "0.001",
                    "price": "30000",
                    "time_in_force": "gtc",
                    "iceberg": "0",
                    "left": "0.001",
                    "filled_amount": "0",
                    "fill_price": "0",
                    "fee": "0",
                    "fee_currency": "USDT",
                    "point_fee": "0",
                    "gt_fee": "0",
                    "gt_discount": false,
                    "rebated_fee": "0",
                    "rebated_fee_currency": "USDT"
                }
            },
            {
                "succeeded": false,
                "message": "Invalid price"
            }
        ]"#;

        let responses: Vec<BatchOrderResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 2);

        // First response - successful
        assert!(responses[0].succeeded);
        assert!(responses[0].order.is_some());
        assert!(responses[0].message.is_none());

        // Second response - failed
        assert!(!responses[1].succeeded);
        assert!(responses[1].order.is_none());
        assert_eq!(responses[1].message.as_ref().unwrap(), "Invalid price");
    }

    #[test]
    fn test_create_batch_orders_request_different_order_types() {
        let mut orders = vec![create_sample_order(
            "BTC_USDT",
            OrderSide::Buy,
            "0.001",
            "30000",
        )];

        // Market order (no price)
        orders.push(CreateOrderRequest {
            currency_pair: "ETH_USDT".to_string(),
            side: OrderSide::Sell,
            amount: "0.1".to_string(),
            price: None,
            order_type: OrderType::Market,
            account: None,
            time_in_force: Some(TimeInForce::ImmediateOrCancel),
            iceberg: None,
            stp_mode: None,
            text: None,
        });

        let request = CreateBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array[0]["type"], "limit");
        assert_eq!(orders_array[1]["type"], "market");

        // Limit order should have price, market order should not
        assert!(orders_array[0]["price"].is_string());
        let obj = orders_array[1].as_object().unwrap();
        assert!(!obj.contains_key("price"));
    }

    #[test]
    fn test_create_batch_orders_request_with_optional_fields() {
        let order = CreateOrderRequest {
            currency_pair: "BTC_USDT".to_string(),
            side: OrderSide::Buy,
            amount: "0.001".to_string(),
            price: Some("30000".to_string()),
            order_type: OrderType::Limit,
            account: Some("spot".to_string()),
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            iceberg: Some("0.0001".to_string()),
            stp_mode: Some(StpMode::CancelNewest),
            text: Some("batch_test_order".to_string()),
        };

        let request = CreateBatchOrdersRequest {
            orders: vec![order],
        };

        let json = serde_json::to_value(&request).unwrap();
        let first_order = &json["orders"][0];

        assert_eq!(first_order["account"], "spot");
        assert_eq!(first_order["iceberg"], "0.0001");
        assert_eq!(first_order["stp_mode"], "cn");
        assert_eq!(first_order["text"], "batch_test_order");
    }

    #[test]
    fn test_create_batch_orders_request_realistic_arbitrage_scenario() {
        // Simulate an arbitrage scenario across multiple pairs
        let orders = vec![
            // Buy BTC with USDT
            create_sample_order("BTC_USDT", OrderSide::Buy, "0.1", "29950"),
            // Sell BTC for ETH
            create_sample_order("BTC_ETH", OrderSide::Sell, "0.1", "12.5"),
            // Sell ETH for USDT
            create_sample_order("ETH_USDT", OrderSide::Sell, "1.25", "2400"),
        ];

        let request = CreateBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        // Verify the arbitrage chain
        assert_eq!(orders_array[0]["currency_pair"], "BTC_USDT");
        assert_eq!(orders_array[0]["side"], "buy");

        assert_eq!(orders_array[1]["currency_pair"], "BTC_ETH");
        assert_eq!(orders_array[1]["side"], "sell");

        assert_eq!(orders_array[2]["currency_pair"], "ETH_USDT");
        assert_eq!(orders_array[2]["side"], "sell");
    }

    #[test]
    fn test_create_batch_orders_request_dca_scenario() {
        // Dollar Cost Averaging scenario - multiple small buys at different prices
        let mut orders = Vec::new();
        let base_price = 30000;
        let increment = 500;

        for i in 0..5 {
            let price = base_price - (i * increment);
            orders.push(create_sample_order(
                "BTC_USDT",
                OrderSide::Buy,
                "0.002",
                &price.to_string(),
            ));
        }

        let request = CreateBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array.len(), 5);

        // Verify descending price levels
        for (i, order) in orders_array.iter().enumerate() {
            let expected_price = base_price - (i * increment);
            assert_eq!(order["price"], expected_price.to_string());
            assert_eq!(order["side"], "buy");
            assert_eq!(order["amount"], "0.002");
        }
    }

    #[test]
    fn test_create_batch_orders_request_grid_trading_scenario() {
        // Grid trading scenario - alternating buy/sell orders
        let mut orders = Vec::new();
        let base_price = 30000;
        let grid_size = 200;

        for i in 0..6 {
            let price = base_price + ((i - 3) * grid_size);
            let side = if i < 3 {
                OrderSide::Buy
            } else {
                OrderSide::Sell
            };

            orders.push(create_sample_order(
                "BTC_USDT",
                side,
                "0.001",
                &price.to_string(),
            ));
        }

        let request = CreateBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array.len(), 6);

        // Verify grid structure
        for (i, order) in orders_array.iter().enumerate() {
            let expected_side = if i < 3 { "buy" } else { "sell" };
            assert_eq!(order["side"], expected_side);
        }
    }

    #[test]
    fn test_create_batch_orders_request_different_precision_amounts() {
        let orders = vec![
            create_sample_order("BTC_USDT", OrderSide::Buy, "0.00000001", "30000"), // Min BTC precision
            create_sample_order("ETH_USDT", OrderSide::Buy, "0.000001", "2500"), // Min ETH precision
            create_sample_order("BNB_USDT", OrderSide::Buy, "0.001", "300"),     // Normal precision
            create_sample_order("USDC_USDT", OrderSide::Buy, "1000.123456", "1.0001"), // Stablecoin
        ];

        let request = CreateBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array[0]["amount"], "0.00000001");
        assert_eq!(orders_array[1]["amount"], "0.000001");
        assert_eq!(orders_array[2]["amount"], "0.001");
        assert_eq!(orders_array[3]["amount"], "1000.123456");
    }

    #[test]
    fn test_create_batch_orders_request_endpoint_validation() {
        let orders = vec![create_sample_order(
            "BTC_USDT",
            OrderSide::Buy,
            "0.001",
            "30000",
        )];

        let request = CreateBatchOrdersRequest { orders };

        // Verify request can be serialized for the endpoint
        let json = serde_json::to_value(&request).unwrap();
        assert!(json["orders"].is_array());

        // Verify the structure matches API expectations
        assert!(json.as_object().unwrap().contains_key("orders"));
        assert_eq!(json.as_object().unwrap().len(), 1); // Only "orders" field
    }

    #[test]
    fn test_batch_order_response_serialization() {
        let response = BatchOrderResponse {
            succeeded: true,
            order: None,
            message: Some("Order queued".to_string()),
        };

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["succeeded"], true);
        assert_eq!(json["message"], "Order queued");

        // Order should be omitted when None
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("order"));
    }

    #[test]
    fn test_create_batch_orders_request_clone() {
        let orders = vec![create_sample_order(
            "BTC_USDT",
            OrderSide::Buy,
            "0.001",
            "30000",
        )];

        let original = CreateBatchOrdersRequest { orders };
        let cloned = original.clone();

        assert_eq!(cloned.orders.len(), original.orders.len());
        assert_eq!(
            cloned.orders[0].currency_pair,
            original.orders[0].currency_pair
        );
    }

    #[test]
    fn test_create_batch_orders_request_debug() {
        let orders = vec![create_sample_order(
            "BTC_USDT",
            OrderSide::Buy,
            "0.001",
            "30000",
        )];

        let request = CreateBatchOrdersRequest { orders };
        let debug_str = format!("{:?}", request);

        assert!(debug_str.contains("CreateBatchOrdersRequest"));
        assert!(debug_str.contains("BTC_USDT"));
    }

    #[test]
    fn test_batch_order_response_clone() {
        let response = BatchOrderResponse {
            succeeded: true,
            order: None,
            message: Some("Test message".to_string()),
        };

        let cloned = response.clone();
        assert!(cloned.succeeded);
        assert_eq!(cloned.message, Some("Test message".to_string()));
        assert!(cloned.order.is_none());
    }

    #[test]
    fn test_batch_order_response_debug() {
        let response = BatchOrderResponse {
            succeeded: false,
            order: None,
            message: Some("Error message".to_string()),
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("BatchOrderResponse"));
        assert!(debug_str.contains("false"));
        assert!(debug_str.contains("Error message"));
    }

    #[test]
    fn test_create_batch_orders_request_edge_case_large_batch() {
        // Test with exactly 10 orders (maximum allowed)
        let mut orders = Vec::new();
        for i in 1..=10 {
            orders.push(create_sample_order(
                "BTC_USDT",
                OrderSide::Buy,
                &format!("0.00{}", i),
                &format!("{}", 30000 + i * 10),
            ));
        }

        let request = CreateBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array.len(), 10);

        // Verify each order has unique values
        for (i, order) in orders_array.iter().enumerate() {
            let expected_amount = format!("0.00{}", i + 1);
            let expected_price = format!("{}", 30000 + (i + 1) * 10);

            assert_eq!(order["amount"], expected_amount);
            assert_eq!(order["price"], expected_price);
        }
    }

    #[test]
    fn test_create_batch_orders_request_mixed_accounts() {
        let orders = vec![
            CreateOrderRequest {
                currency_pair: "BTC_USDT".to_string(),
                side: OrderSide::Buy,
                amount: "0.001".to_string(),
                price: Some("30000".to_string()),
                order_type: OrderType::Limit,
                account: Some("spot".to_string()),
                time_in_force: Some(TimeInForce::GoodTillCanceled),
                iceberg: None,
                stp_mode: None,
                text: None,
            },
            CreateOrderRequest {
                currency_pair: "ETH_USDT".to_string(),
                side: OrderSide::Sell,
                amount: "0.1".to_string(),
                price: Some("2500".to_string()),
                order_type: OrderType::Limit,
                account: Some("margin".to_string()),
                time_in_force: Some(TimeInForce::GoodTillCanceled),
                iceberg: None,
                stp_mode: None,
                text: None,
            },
        ];

        let request = CreateBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array[0]["account"], "spot");
        assert_eq!(orders_array[1]["account"], "margin");
    }
}
