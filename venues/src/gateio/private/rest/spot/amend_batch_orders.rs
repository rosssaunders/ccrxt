use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const AMEND_BATCH_ORDERS_ENDPOINT: &str = "/spot/amend_batch_orders";

/// Request parameters for amending an order in batch.
#[derive(Debug, Clone, Serialize)]
pub struct AmendOrderRequest {
    /// Order ID to amend.
    pub id: String,

    /// New price for the order (optional). If provided, must be a valid decimal string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// New amount for the order (optional). If provided, must be a valid decimal string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// Currency pair for the order (e.g., "BTC_USDT").
    pub currency_pair: String,

    /// Account mode (optional). Can be "spot", "margin", "cross_margin", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

/// Request parameters for batch order amendment.
#[derive(Debug, Clone, Serialize)]
pub struct AmendBatchOrdersRequest {
    /// List of orders to amend in the batch operation.
    pub orders: Vec<AmendOrderRequest>,
}

/// Response information for an amended order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendedOrder {
    /// Order ID.
    pub id: String,

    /// Currency pair for the order.
    pub currency_pair: String,

    /// Order status (e.g., "open", "closed", "cancelled").
    pub status: String,

    /// Account mode for the order.
    pub account: String,

    /// Order side ("buy" or "sell").
    pub side: String,

    /// Order amount as a decimal string.
    pub amount: String,

    /// Order price as a decimal string.
    pub price: String,

    /// Order type (e.g., "limit", "market").
    #[serde(rename = "type")]
    pub order_type: String,

    /// Time in force setting for the order.
    pub time_in_force: String,

    /// Amount that has been filled as a decimal string.
    pub filled_amount: String,

    /// Remaining amount to be filled as a decimal string.
    pub left: String,

    /// Average deal price as a decimal string.
    pub avg_deal_price: String,

    /// Fee paid for this order as a decimal string.
    pub fee: String,

    /// Currency in which the fee is paid.
    pub fee_currency: String,

    /// Points fee as a decimal string.
    pub points_fee: String,

    /// GT fee as a decimal string.
    pub gt_fee: String,

    /// Order creation timestamp as a string.
    pub create_time: String,

    /// Order update timestamp as a string.
    pub update_time: String,

    /// Optional text identifier for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Whether the amendment was successful.
    pub succeeded: bool,

    /// Error message if amendment failed (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Error code if amendment failed (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl RestClient {
    /// Batch modification of orders
    ///
    /// Amend multiple orders in batch. Batch amendment can change the order price,
    /// amount, and account mode. Returns information about each order amendment attempt.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#batch-modification-of-orders)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The batch amendment request containing orders to modify
    ///
    /// # Returns
    /// List of amended order results indicating success or failure for each order
    pub async fn spot_amend_batch_orders(
        &self,
        request: AmendBatchOrdersRequest,
    ) -> RestResult<Vec<AmendedOrder>> {
        self.post(AMEND_BATCH_ORDERS_ENDPOINT, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_amend_request(
        id: &str,
        currency_pair: &str,
        price: Option<&str>,
        amount: Option<&str>,
    ) -> AmendOrderRequest {
        AmendOrderRequest {
            id: id.to_string(),
            price: price.map(|p| p.to_string()),
            amount: amount.map(|a| a.to_string()),
            currency_pair: currency_pair.to_string(),
            account: None,
        }
    }

    #[test]
    fn test_amend_order_request_serialization() {
        let request = AmendOrderRequest {
            id: "12345678".to_string(),
            price: Some("31000".to_string()),
            amount: Some("0.002".to_string()),
            currency_pair: "BTC_USDT".to_string(),
            account: Some("spot".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["id"], "12345678");
        assert_eq!(json["price"], "31000");
        assert_eq!(json["amount"], "0.002");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["account"], "spot");
    }

    #[test]
    fn test_amend_order_request_price_only() {
        let request = AmendOrderRequest {
            id: "87654321".to_string(),
            price: Some("32000".to_string()),
            amount: None,
            currency_pair: "BTC_USDT".to_string(),
            account: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["id"], "87654321");
        assert_eq!(json["price"], "32000");
        assert_eq!(json["currency_pair"], "BTC_USDT");

        // Amount and account should not be present when None
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("amount"));
        assert!(!obj.contains_key("account"));
    }

    #[test]
    fn test_amend_order_request_amount_only() {
        let request = AmendOrderRequest {
            id: "11111111".to_string(),
            price: None,
            amount: Some("0.005".to_string()),
            currency_pair: "ETH_USDT".to_string(),
            account: Some("margin".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["id"], "11111111");
        assert_eq!(json["amount"], "0.005");
        assert_eq!(json["currency_pair"], "ETH_USDT");
        assert_eq!(json["account"], "margin");

        // Price should not be present when None
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("price"));
    }

    #[test]
    fn test_amend_batch_orders_request_single_order() {
        let order =
            create_sample_amend_request("12345678", "BTC_USDT", Some("31000"), Some("0.002"));
        let request = AmendBatchOrdersRequest {
            orders: vec![order],
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["orders"].is_array());
        assert_eq!(json["orders"].as_array().unwrap().len(), 1);

        let first_order = &json["orders"][0];
        assert_eq!(first_order["id"], "12345678");
        assert_eq!(first_order["currency_pair"], "BTC_USDT");
        assert_eq!(first_order["price"], "31000");
        assert_eq!(first_order["amount"], "0.002");
    }

    #[test]
    fn test_amend_batch_orders_request_multiple_orders() {
        let orders = vec![
            create_sample_amend_request("12345678", "BTC_USDT", Some("31000"), None),
            create_sample_amend_request("87654321", "ETH_USDT", None, Some("0.5")),
            create_sample_amend_request("11111111", "BNB_USDT", Some("310"), Some("5.0")),
        ];

        let request = AmendBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["orders"].is_array());
        assert_eq!(json["orders"].as_array().unwrap().len(), 3);

        let orders_array = json["orders"].as_array().unwrap();

        // First order - price amendment only
        assert_eq!(orders_array[0]["id"], "12345678");
        assert_eq!(orders_array[0]["price"], "31000");
        let obj0 = orders_array[0].as_object().unwrap();
        assert!(!obj0.contains_key("amount"));

        // Second order - amount amendment only
        assert_eq!(orders_array[1]["id"], "87654321");
        assert_eq!(orders_array[1]["amount"], "0.5");
        let obj1 = orders_array[1].as_object().unwrap();
        assert!(!obj1.contains_key("price"));

        // Third order - both price and amount
        assert_eq!(orders_array[2]["id"], "11111111");
        assert_eq!(orders_array[2]["price"], "310");
        assert_eq!(orders_array[2]["amount"], "5.0");
    }

    #[test]
    fn test_amend_batch_orders_request_empty_orders() {
        let request = AmendBatchOrdersRequest { orders: vec![] };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json["orders"].is_array());
        assert_eq!(json["orders"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_amended_order_successful_response() {
        let json = r#"{
            "id": "12345678",
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "spot",
            "side": "buy",
            "amount": "0.002",
            "price": "31000",
            "type": "limit",
            "time_in_force": "gtc",
            "filled_amount": "0",
            "left": "0.002",
            "avg_deal_price": "0",
            "fee": "0",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0",
            "create_time": "1640995200",
            "update_time": "1640995300",
            "text": "amended_order",
            "succeeded": true
        }"#;

        let amended_order: AmendedOrder = serde_json::from_str(json).unwrap();
        assert_eq!(amended_order.id, "12345678");
        assert_eq!(amended_order.currency_pair, "BTC_USDT");
        assert_eq!(amended_order.status, "open");
        assert_eq!(amended_order.account, "spot");
        assert_eq!(amended_order.side, "buy");
        assert_eq!(amended_order.amount, "0.002");
        assert_eq!(amended_order.price, "31000");
        assert_eq!(amended_order.order_type, "limit");
        assert_eq!(amended_order.time_in_force, "gtc");
        assert!(amended_order.succeeded);
        assert!(amended_order.message.is_none());
        assert!(amended_order.code.is_none());
    }

    #[test]
    fn test_amended_order_failed_response() {
        let json = r#"{
            "id": "87654321",
            "currency_pair": "ETH_USDT",
            "status": "open",
            "account": "spot",
            "side": "sell",
            "amount": "1.0",
            "price": "2500",
            "type": "limit",
            "time_in_force": "gtc",
            "filled_amount": "0",
            "left": "1.0",
            "avg_deal_price": "0",
            "fee": "0",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0",
            "create_time": "1640995200",
            "update_time": "1640995200",
            "succeeded": false,
            "message": "Invalid price",
            "code": "INVALID_PRICE"
        }"#;

        let amended_order: AmendedOrder = serde_json::from_str(json).unwrap();
        assert_eq!(amended_order.id, "87654321");
        assert!(!amended_order.succeeded);
        assert_eq!(amended_order.message.as_ref().unwrap(), "Invalid price");
        assert_eq!(amended_order.code.as_ref().unwrap(), "INVALID_PRICE");
    }

    #[test]
    fn test_amended_order_array_response() {
        let json = r#"[
            {
                "id": "12345678",
                "currency_pair": "BTC_USDT",
                "status": "open",
                "account": "spot",
                "side": "buy",
                "amount": "0.002",
                "price": "31000",
                "type": "limit",
                "time_in_force": "gtc",
                "filled_amount": "0",
                "left": "0.002",
                "avg_deal_price": "0",
                "fee": "0",
                "fee_currency": "USDT",
                "points_fee": "0",
                "gt_fee": "0",
                "create_time": "1640995200",
                "update_time": "1640995300",
                "text": "amended_order_1",
                "succeeded": true
            },
            {
                "id": "87654321",
                "currency_pair": "ETH_USDT",
                "status": "open",
                "account": "spot",
                "side": "sell",
                "amount": "1.0",
                "price": "2500",
                "type": "limit",
                "time_in_force": "gtc",
                "filled_amount": "0",
                "left": "1.0",
                "avg_deal_price": "0",
                "fee": "0",
                "fee_currency": "USDT",
                "points_fee": "0",
                "gt_fee": "0",
                "create_time": "1640995200",
                "update_time": "1640995200",
                "succeeded": false,
                "message": "Insufficient balance",
                "code": "INSUFFICIENT_BALANCE"
            }
        ]"#;

        let amended_orders: Vec<AmendedOrder> = serde_json::from_str(json).unwrap();
        assert_eq!(amended_orders.len(), 2);

        // First amendment - successful
        assert_eq!(amended_orders[0].id, "12345678");
        assert!(amended_orders[0].succeeded);
        assert!(amended_orders[0].message.is_none());
        assert!(amended_orders[0].code.is_none());

        // Second amendment - failed
        assert_eq!(amended_orders[1].id, "87654321");
        assert!(!amended_orders[1].succeeded);
        assert_eq!(
            amended_orders[1].message.as_ref().unwrap(),
            "Insufficient balance"
        );
        assert_eq!(
            amended_orders[1].code.as_ref().unwrap(),
            "INSUFFICIENT_BALANCE"
        );
    }

    #[test]
    fn test_amend_batch_orders_request_different_currency_pairs() {
        let orders = vec![
            create_sample_amend_request("12345678", "BTC_USDT", Some("31000"), None),
            create_sample_amend_request("87654321", "ETH_BTC", None, Some("0.5")),
            create_sample_amend_request("11111111", "SOL_USDC", Some("160"), Some("10.0")),
            create_sample_amend_request("22222222", "USDC_USDT", Some("1.0002"), None),
        ];

        let request = AmendBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array[0]["currency_pair"], "BTC_USDT");
        assert_eq!(orders_array[1]["currency_pair"], "ETH_BTC");
        assert_eq!(orders_array[2]["currency_pair"], "SOL_USDC");
        assert_eq!(orders_array[3]["currency_pair"], "USDC_USDT");
    }

    #[test]
    fn test_amend_batch_orders_request_different_account_types() {
        let orders = vec![
            AmendOrderRequest {
                id: "12345678".to_string(),
                price: Some("31000".to_string()),
                amount: None,
                currency_pair: "BTC_USDT".to_string(),
                account: Some("spot".to_string()),
            },
            AmendOrderRequest {
                id: "87654321".to_string(),
                price: None,
                amount: Some("0.5".to_string()),
                currency_pair: "ETH_USDT".to_string(),
                account: Some("margin".to_string()),
            },
            AmendOrderRequest {
                id: "11111111".to_string(),
                price: Some("310".to_string()),
                amount: Some("5.0".to_string()),
                currency_pair: "BNB_USDT".to_string(),
                account: None,
            },
        ];

        let request = AmendBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array[0]["account"], "spot");
        assert_eq!(orders_array[1]["account"], "margin");

        // Third order should not have account field
        let obj2 = orders_array[2].as_object().unwrap();
        assert!(!obj2.contains_key("account"));
    }

    #[test]
    fn test_amend_batch_orders_request_realistic_price_adjustment_scenario() {
        // Scenario: Market maker adjusting quotes based on market movement
        let orders = vec![
            create_sample_amend_request("order1", "BTC_USDT", Some("31500"), None), // Increase bid
            create_sample_amend_request("order2", "BTC_USDT", Some("31600"), None), // Increase ask
            create_sample_amend_request("order3", "ETH_USDT", Some("2550"), None), // Adjust ETH bid
            create_sample_amend_request("order4", "ETH_USDT", Some("2560"), None), // Adjust ETH ask
        ];

        let request = AmendBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array.len(), 4);

        // Verify all are price-only amendments
        for order in orders_array {
            assert!(order["price"].is_string());
            let obj = order.as_object().unwrap();
            assert!(!obj.contains_key("amount"));
        }
    }

    #[test]
    fn test_amend_batch_orders_request_realistic_amount_adjustment_scenario() {
        // Scenario: Adjusting order sizes based on inventory management
        let orders = vec![
            create_sample_amend_request("order1", "BTC_USDT", None, Some("0.001")), // Reduce BTC size
            create_sample_amend_request("order2", "ETH_USDT", None, Some("2.0")), // Increase ETH size
            create_sample_amend_request("order3", "BNB_USDT", None, Some("50.0")), // Adjust BNB size
        ];

        let request = AmendBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array.len(), 3);

        // Verify all are amount-only amendments
        for order in orders_array {
            assert!(order["amount"].is_string());
            let obj = order.as_object().unwrap();
            assert!(!obj.contains_key("price"));
        }
    }

    #[test]
    fn test_amend_batch_orders_request_realistic_mixed_scenario() {
        // Scenario: Complex adjustment with both price and amount changes
        let orders = vec![
            create_sample_amend_request("order1", "BTC_USDT", Some("31200"), Some("0.005")), // Both
            create_sample_amend_request("order2", "ETH_USDT", Some("2520"), None), // Price only
            create_sample_amend_request("order3", "BNB_USDT", None, Some("25.0")), // Amount only
            create_sample_amend_request("order4", "SOL_USDC", Some("155"), Some("100.0")), // Both
        ];

        let request = AmendBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array.len(), 4);

        // Verify mixed amendments
        let obj0 = orders_array[0].as_object().unwrap();
        assert!(obj0.contains_key("price"));
        assert!(obj0.contains_key("amount"));

        let obj1 = orders_array[1].as_object().unwrap();
        assert!(obj1.contains_key("price"));
        assert!(!obj1.contains_key("amount"));

        let obj2 = orders_array[2].as_object().unwrap();
        assert!(!obj2.contains_key("price"));
        assert!(obj2.contains_key("amount"));

        let obj3 = orders_array[3].as_object().unwrap();
        assert!(obj3.contains_key("price"));
        assert!(obj3.contains_key("amount"));
    }

    #[test]
    fn test_amend_batch_orders_request_precision_handling() {
        let orders = vec![
            create_sample_amend_request(
                "order1",
                "BTC_USDT",
                Some("31000.12345678"),
                Some("0.00000001"),
            ),
            create_sample_amend_request(
                "order2",
                "ETH_USDT",
                Some("2500.123456"),
                Some("0.000001"),
            ),
            create_sample_amend_request(
                "order3",
                "USDC_USDT",
                Some("1.00001234"),
                Some("1000.123456"),
            ),
        ];

        let request = AmendBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array[0]["price"], "31000.12345678");
        assert_eq!(orders_array[0]["amount"], "0.00000001");
        assert_eq!(orders_array[1]["price"], "2500.123456");
        assert_eq!(orders_array[1]["amount"], "0.000001");
        assert_eq!(orders_array[2]["price"], "1.00001234");
        assert_eq!(orders_array[2]["amount"], "1000.123456");
    }

    #[test]
    fn test_amended_order_with_text_field() {
        let json = r#"{
            "id": "12345678",
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "spot",
            "side": "buy",
            "amount": "0.002",
            "price": "31000",
            "type": "limit",
            "time_in_force": "gtc",
            "filled_amount": "0",
            "left": "0.002",
            "avg_deal_price": "0",
            "fee": "0",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0",
            "create_time": "1640995200",
            "update_time": "1640995300",
            "text": "client_order_123",
            "succeeded": true
        }"#;

        let amended_order: AmendedOrder = serde_json::from_str(json).unwrap();
        assert_eq!(amended_order.text.as_ref().unwrap(), "client_order_123");
        assert!(amended_order.succeeded);
    }

    #[test]
    fn test_amended_order_without_optional_fields() {
        let json = r#"{
            "id": "12345678",
            "currency_pair": "BTC_USDT",
            "status": "open",
            "account": "spot",
            "side": "buy",
            "amount": "0.002",
            "price": "31000",
            "type": "limit",
            "time_in_force": "gtc",
            "filled_amount": "0",
            "left": "0.002",
            "avg_deal_price": "0",
            "fee": "0",
            "fee_currency": "USDT",
            "points_fee": "0",
            "gt_fee": "0",
            "create_time": "1640995200",
            "update_time": "1640995300",
            "succeeded": true
        }"#;

        let amended_order: AmendedOrder = serde_json::from_str(json).unwrap();
        assert!(amended_order.text.is_none());
        assert!(amended_order.message.is_none());
        assert!(amended_order.code.is_none());
        assert!(amended_order.succeeded);
    }

    #[test]
    fn test_amend_order_request_clone() {
        let original =
            create_sample_amend_request("12345678", "BTC_USDT", Some("31000"), Some("0.002"));
        let cloned = original.clone();

        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.price, original.price);
        assert_eq!(cloned.amount, original.amount);
        assert_eq!(cloned.account, original.account);
    }

    #[test]
    fn test_amend_batch_orders_request_clone() {
        let orders = vec![
            create_sample_amend_request("12345678", "BTC_USDT", Some("31000"), None),
            create_sample_amend_request("87654321", "ETH_USDT", None, Some("0.5")),
        ];

        let original = AmendBatchOrdersRequest { orders };
        let cloned = original.clone();

        assert_eq!(cloned.orders.len(), original.orders.len());
        assert_eq!(cloned.orders[0].id, original.orders[0].id);
        assert_eq!(cloned.orders[1].id, original.orders[1].id);
    }

    #[test]
    fn test_amended_order_clone() {
        let original = AmendedOrder {
            id: "12345678".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.002".to_string(),
            price: "31000".to_string(),
            order_type: "limit".to_string(),
            time_in_force: "gtc".to_string(),
            filled_amount: "0".to_string(),
            left: "0.002".to_string(),
            avg_deal_price: "0".to_string(),
            fee: "0".to_string(),
            fee_currency: "USDT".to_string(),
            points_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            create_time: "1640995200".to_string(),
            update_time: "1640995300".to_string(),
            text: Some("test_order".to_string()),
            succeeded: true,
            message: None,
            code: None,
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.succeeded, original.succeeded);
        assert_eq!(cloned.text, original.text);
    }

    #[test]
    fn test_amend_order_request_debug() {
        let request =
            create_sample_amend_request("12345678", "BTC_USDT", Some("31000"), Some("0.002"));
        let debug_str = format!("{:?}", request);

        assert!(debug_str.contains("AmendOrderRequest"));
        assert!(debug_str.contains("12345678"));
        assert!(debug_str.contains("BTC_USDT"));
    }

    #[test]
    fn test_amend_batch_orders_request_debug() {
        let orders = vec![create_sample_amend_request(
            "12345678",
            "BTC_USDT",
            Some("31000"),
            None,
        )];

        let request = AmendBatchOrdersRequest { orders };
        let debug_str = format!("{:?}", request);

        assert!(debug_str.contains("AmendBatchOrdersRequest"));
        assert!(debug_str.contains("12345678"));
    }

    #[test]
    fn test_amended_order_debug() {
        let amended_order = AmendedOrder {
            id: "12345678".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.002".to_string(),
            price: "31000".to_string(),
            order_type: "limit".to_string(),
            time_in_force: "gtc".to_string(),
            filled_amount: "0".to_string(),
            left: "0.002".to_string(),
            avg_deal_price: "0".to_string(),
            fee: "0".to_string(),
            fee_currency: "USDT".to_string(),
            points_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            create_time: "1640995200".to_string(),
            update_time: "1640995300".to_string(),
            text: None,
            succeeded: true,
            message: None,
            code: None,
        };

        let debug_str = format!("{:?}", amended_order);
        assert!(debug_str.contains("AmendedOrder"));
        assert!(debug_str.contains("12345678"));
        assert!(debug_str.contains("true"));
    }

    #[test]
    fn test_amend_batch_orders_request_endpoint_validation() {
        let orders = vec![create_sample_amend_request(
            "12345678",
            "BTC_USDT",
            Some("31000"),
            Some("0.002"),
        )];

        let request = AmendBatchOrdersRequest { orders };

        // Verify request can be serialized for the endpoint
        let json = serde_json::to_value(&request).unwrap();
        assert!(json["orders"].is_array());

        // Verify the structure matches API expectations
        assert!(json.as_object().unwrap().contains_key("orders"));
        assert_eq!(json.as_object().unwrap().len(), 1); // Only "orders" field
    }

    #[test]
    fn test_amended_order_serialization() {
        let amended_order = AmendedOrder {
            id: "12345678".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            status: "open".to_string(),
            account: "spot".to_string(),
            side: "buy".to_string(),
            amount: "0.002".to_string(),
            price: "31000".to_string(),
            order_type: "limit".to_string(),
            time_in_force: "gtc".to_string(),
            filled_amount: "0".to_string(),
            left: "0.002".to_string(),
            avg_deal_price: "0".to_string(),
            fee: "0".to_string(),
            fee_currency: "USDT".to_string(),
            points_fee: "0".to_string(),
            gt_fee: "0".to_string(),
            create_time: "1640995200".to_string(),
            update_time: "1640995300".to_string(),
            text: Some("test_order".to_string()),
            succeeded: true,
            message: None,
            code: None,
        };

        let json = serde_json::to_value(&amended_order).unwrap();
        assert_eq!(json["id"], "12345678");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["type"], "limit");
        assert_eq!(json["succeeded"], true);
        assert_eq!(json["text"], "test_order");

        // Optional fields that are None should be omitted
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("message"));
        assert!(!obj.contains_key("code"));
    }

    #[test]
    fn test_amend_batch_orders_request_large_batch() {
        // Test with multiple orders simulating a large batch amendment
        let mut orders = Vec::new();
        for i in 1..=10 {
            orders.push(create_sample_amend_request(
                &format!("order_{}", i),
                "BTC_USDT",
                Some(&format!("{}", 30000 + i * 100)),
                None,
            ));
        }

        let request = AmendBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array.len(), 10);

        // Verify each order has incremental values
        for (i, order) in orders_array.iter().enumerate() {
            let expected_id = format!("order_{}", i + 1);
            let expected_price = format!("{}", 30000 + (i + 1) * 100);

            assert_eq!(order["id"], expected_id);
            assert_eq!(order["price"], expected_price);
        }
    }

    #[test]
    fn test_amend_batch_orders_request_edge_cases() {
        let orders = vec![
            // Very small amounts and prices
            create_sample_amend_request("order1", "BTC_USDT", Some("0.01"), Some("0.00000001")),
            // Very large amounts and prices
            create_sample_amend_request("order2", "BTC_USDT", Some("999999.99"), Some("1000000.0")),
            // Stablecoin pair with precise pricing
            create_sample_amend_request(
                "order3",
                "USDC_USDT",
                Some("1.00000001"),
                Some("10000.000001"),
            ),
        ];

        let request = AmendBatchOrdersRequest { orders };

        let json = serde_json::to_value(&request).unwrap();
        let orders_array = json["orders"].as_array().unwrap();

        assert_eq!(orders_array[0]["price"], "0.01");
        assert_eq!(orders_array[0]["amount"], "0.00000001");
        assert_eq!(orders_array[1]["price"], "999999.99");
        assert_eq!(orders_array[1]["amount"], "1000000.0");
        assert_eq!(orders_array[2]["price"], "1.00000001");
        assert_eq!(orders_array[2]["amount"], "10000.000001");
    }
}
