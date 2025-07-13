use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_PRICE_ORDERS_ENDPOINT: &str = "/delivery/{}/price_orders";
const DELIVERY_PRICE_ORDER_ENDPOINT: &str = "/delivery/{}/price_orders/{}";

/// Request to create delivery order (re-exported from orders module)
pub use super::orders::CreateDeliveryOrderRequest;

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
    /// See: Gate.io API documentation
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
    ) -> crate::gateio::delivery::Result<DeliveryPriceOrder> {
        let endpoint = DELIVERY_PRICE_ORDERS_ENDPOINT.replace("{}", &request.settle);
        self.post(&endpoint, &request).await
    }

    /// List all delivery price-triggered orders
    ///
    /// Retrieves all price-triggered orders with optional filtering.
    ///
    /// See: Gate.io API documentation
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
    ) -> crate::gateio::delivery::Result<Vec<DeliveryPriceOrder>> {
        let endpoint = DELIVERY_PRICE_ORDERS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a delivery price-triggered order
    ///
    /// Retrieves a specific price-triggered order by its ID.
    ///
    /// See: Gate.io API documentation
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
    ) -> crate::gateio::delivery::Result<DeliveryPriceOrder> {
        let endpoint = DELIVERY_PRICE_ORDER_ENDPOINT.replace("{}", settle).replace("{}", order_id);
        self.get(&endpoint).await
    }

    /// Cancel a delivery price-triggered order
    ///
    /// Cancels a specific price-triggered order.
    ///
    /// See: Gate.io API documentation
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
    ) -> crate::gateio::delivery::Result<DeliveryPriceOrder> {
        let endpoint = DELIVERY_PRICE_ORDER_ENDPOINT.replace("{}", settle).replace("{}", order_id);
        self.delete(&endpoint).await
    }

    /// Cancel all delivery price-triggered orders
    ///
    /// Cancels all price-triggered orders with optional contract filtering.
    ///
    /// See: Gate.io API documentation
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
    ) -> crate::gateio::delivery::Result<Vec<DeliveryPriceOrder>> {
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
