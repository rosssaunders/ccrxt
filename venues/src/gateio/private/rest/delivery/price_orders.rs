use serde::{Deserialize, Serialize};

use super::models::CreateDeliveryOrderRequest;
use super::{RestClient, RestResult};

const DELIVERY_PRICE_ORDERS_ENDPOINT: &str = "/delivery/{}/price_orders";
const DELIVERY_PRICE_ORDER_ENDPOINT: &str = "/delivery/{}/price_orders/{}";

/// Request to create a delivery price-triggered order
#[derive(Debug, Clone, Serialize)]
pub struct CreateDeliveryPriceOrderRequest {
    /// Settlement currency
    pub settle: String,

    /// Initial order (will be created when triggered)
    pub initial: CreateDeliveryOrderRequest,

    /// Trigger condition
    pub trigger: DeliveryTriggerCondition,
}

/// Trigger condition for delivery price orders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryTriggerCondition {
    /// Price comparison rule (>=, <=)
    pub rule: i32,

    /// Trigger price
    pub price: String,

    /// Expiration time (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
}

/// Request parameters for listing delivery price orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListDeliveryPriceOrdersRequest {
    /// Settlement currency
    pub settle: String,

    /// Order status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Maximum number of records (1-1000, default 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Delivery price-triggered order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPriceOrder {
    /// Price order ID
    pub id: i64,

    /// User ID
    pub user: i64,

    /// Creation time
    pub create_time: f64,

    /// Finish time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<f64>,

    /// Trade ID (if triggered)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<i64>,

    /// Price order status
    pub status: String,

    /// Initial order details
    pub initial: CreateDeliveryOrderRequest,

    /// Trigger condition
    pub trigger: DeliveryTriggerCondition,

    /// Reason for order completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl RestClient {
    /// Create a delivery price-triggered order
    ///
    /// Creates a conditional order that triggers when the market price reaches a specified level.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#create-price-triggered-order-3)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The price-triggered order creation request parameters
    ///
    /// # Returns
    /// Created price-triggered order information
    pub async fn create_delivery_price_triggered_order(
        &self,
        request: CreateDeliveryPriceOrderRequest,
    ) -> RestResult<DeliveryPriceOrder> {
        let endpoint = DELIVERY_PRICE_ORDERS_ENDPOINT.replace("{}", &request.settle);
        self.post(&endpoint, &request).await
    }

    /// List all delivery price-triggered orders
    ///
    /// Retrieves all price-triggered orders with optional filtering.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#create-price-triggered-order-3)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The price-triggered orders list request parameters
    ///
    /// # Returns
    /// List of price-triggered orders
    pub async fn list_delivery_price_triggered_orders(
        &self,
        params: ListDeliveryPriceOrdersRequest,
    ) -> RestResult<Vec<DeliveryPriceOrder>> {
        let endpoint = DELIVERY_PRICE_ORDERS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a delivery price-triggered order
    ///
    /// Retrieves a specific price-triggered order by its ID.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#create-price-triggered-order-3)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `order_id` - Price order ID to retrieve
    ///
    /// # Returns
    /// Specific price-triggered order details
    pub async fn get_delivery_price_triggered_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> RestResult<DeliveryPriceOrder> {
        let endpoint = DELIVERY_PRICE_ORDER_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", order_id, 1);
        self.get(&endpoint).await
    }

    /// Cancel a delivery price-triggered order
    ///
    /// Cancels a specific price-triggered order.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#create-price-triggered-order-3)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `order_id` - Price order ID to cancel
    ///
    /// # Returns
    /// Cancelled price-triggered order details
    pub async fn cancel_delivery_price_triggered_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> RestResult<DeliveryPriceOrder> {
        let endpoint = DELIVERY_PRICE_ORDER_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", order_id, 1);
        self.delete(&endpoint).await
    }

    /// Cancel all delivery price-triggered orders
    ///
    /// Cancels all price-triggered orders with optional contract filtering.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#create-price-triggered-order-3)
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Optional contract filter
    ///
    /// # Returns
    /// List of cancelled price-triggered orders
    pub async fn cancel_all_delivery_price_triggered_orders(
        &self,
        settle: &str,
        contract: Option<&str>,
    ) -> RestResult<Vec<DeliveryPriceOrder>> {
        let endpoint = DELIVERY_PRICE_ORDERS_ENDPOINT.replace("{}", settle);

        #[derive(Serialize)]
        struct CancelAllParams<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            contract: Option<&'a str>,
        }

        let params = CancelAllParams { contract };
        self.delete_with_query(&endpoint, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_price_orders_endpoint() {
        assert_eq!(DELIVERY_PRICE_ORDERS_ENDPOINT, "/delivery/{}/price_orders");
    }

    #[test]
    fn test_delivery_price_order_endpoint() {
        assert_eq!(
            DELIVERY_PRICE_ORDER_ENDPOINT,
            "/delivery/{}/price_orders/{}"
        );
    }

    #[test]
    fn test_price_orders_endpoint_construction() {
        let settle = "BTC";
        let endpoint = DELIVERY_PRICE_ORDERS_ENDPOINT.replace("{}", settle);
        assert_eq!(endpoint, "/delivery/BTC/price_orders");
    }

    #[test]
    fn test_price_order_endpoint_construction() {
        let settle = "USDT";
        let order_id = "12345678";
        let endpoint = DELIVERY_PRICE_ORDER_ENDPOINT
            .replacen("{}", settle, 1)
            .replacen("{}", order_id, 1);
        assert_eq!(endpoint, "/delivery/USDT/price_orders/12345678");
    }

    #[test]
    fn test_delivery_trigger_condition_deserialization() {
        let json = r#"{
            "rule": 1,
            "price": "45000.0",
            "expiration": 1640995200
        }"#;

        let trigger: DeliveryTriggerCondition = serde_json::from_str(json).unwrap();
        assert_eq!(trigger.rule, 1);
        assert_eq!(trigger.price, "45000.0");
        assert_eq!(trigger.expiration, Some(1640995200));
    }

    #[test]
    fn test_list_delivery_price_orders_request_minimal() {
        let request = ListDeliveryPriceOrdersRequest {
            settle: "BTC".to_string(),
            ..Default::default()
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "BTC");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_list_delivery_price_orders_request_full() {
        let request = ListDeliveryPriceOrdersRequest {
            settle: "USDT".to_string(),
            status: Some("open".to_string()),
            contract: Some("BTC_USDT_20240315".to_string()),
            offset: Some(10),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["settle"], "USDT");
        assert_eq!(json["status"], "open");
        assert_eq!(json["contract"], "BTC_USDT_20240315");
        assert_eq!(json["offset"], 10);
        assert_eq!(json["limit"], 50);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 5);
    }

    #[test]
    fn test_endpoints_have_correct_placeholders() {
        let orders_placeholder_count = DELIVERY_PRICE_ORDERS_ENDPOINT.matches("{}").count();
        assert_eq!(orders_placeholder_count, 1);

        let order_placeholder_count = DELIVERY_PRICE_ORDER_ENDPOINT.matches("{}").count();
        assert_eq!(order_placeholder_count, 2);
    }

    #[test]
    fn test_price_order_endpoint_different_params() {
        let test_cases = vec![
            ("BTC", "order123", "/delivery/BTC/price_orders/order123"),
            ("USDT", "987654321", "/delivery/USDT/price_orders/987654321"),
            (
                "ETH",
                "abc-def-123",
                "/delivery/ETH/price_orders/abc-def-123",
            ),
        ];

        for (settle, order_id, expected) in test_cases {
            let endpoint = DELIVERY_PRICE_ORDER_ENDPOINT
                .replacen("{}", settle, 1)
                .replacen("{}", order_id, 1);
            assert_eq!(
                endpoint, expected,
                "Failed for settle: {}, order_id: {}",
                settle, order_id
            );
        }
    }
}
