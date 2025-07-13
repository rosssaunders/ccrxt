use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_ORDERS_ENDPOINT: &str = "/delivery/{}/orders";
const DELIVERY_ORDER_ENDPOINT: &str = "/delivery/{}/orders/{}";

/// Request to create delivery order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeliveryOrderRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Order size (positive for long, negative for short)
    pub size: i64,

    /// Order price (omit for market orders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force (gtc, ioc, poc, fok)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,

    /// Text label for order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Reduce only order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,

    /// Close position order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<bool>,

    /// Iceberg order amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<i64>,

    /// Auto size for closing position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_size: Option<String>,
}

/// Delivery order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryOrder {
    /// Order ID
    pub id: i64,

    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Creation timestamp
    pub create_time: f64,

    /// Finish timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<f64>,

    /// Finish reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_as: Option<String>,

    /// Order status
    pub status: String,

    /// Order size
    pub size: i64,

    /// Iceberg amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<i64>,

    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force
    pub tif: String,

    /// Left amount
    pub left: i64,

    /// Filled total
    pub fill_price: String,

    /// Order text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Reduce only
    pub reduce_only: bool,

    /// Close position
    pub close: bool,

    /// STP action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,
}

/// Request parameters for listing delivery orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListDeliveryOrdersRequest {
    /// Settlement currency
    pub settle: String,

    /// Order status (open, finished)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-1000, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl RestClient {
    /// Create a delivery order
    ///
    /// This endpoint creates a new delivery order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The delivery order creation request parameters
    ///
    /// # Returns
    /// Created delivery order information
    pub async fn create_delivery_order(
        &self,
        request: CreateDeliveryOrderRequest,
    ) -> crate::gateio::delivery::Result<DeliveryOrder> {
        let endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", &request.settle);
        self.post(&endpoint, &request).await
    }

    /// List delivery orders
    ///
    /// This endpoint returns delivery orders for the authenticated user.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery orders list request parameters
    ///
    /// # Returns
    /// List of delivery orders
    pub async fn list_delivery_orders(
        &self,
        params: ListDeliveryOrdersRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryOrder>> {
        let endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a specific delivery order
    ///
    /// This endpoint returns details for a specific delivery order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `order_id` - Order ID to retrieve
    ///
    /// # Returns
    /// Specific delivery order details
    pub async fn get_delivery_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::delivery::Result<DeliveryOrder> {
        let endpoint = DELIVERY_ORDER_ENDPOINT
            .replace("{}", settle)
            .replace("{}", order_id);
        self.get(&endpoint).await
    }

    /// Cancel all delivery orders
    ///
    /// This endpoint cancels all delivery orders for a specific contract or all contracts.
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
    /// List of cancelled delivery orders
    pub async fn cancel_all_delivery_orders(
        &self,
        settle: &str,
        contract: Option<&str>,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryOrder>> {
        let mut endpoint = DELIVERY_ORDERS_ENDPOINT.replace("{}", settle);

        if let Some(contract) = contract {
            endpoint.push_str(&format!("?contract={}", contract));
        }

        self.delete(&endpoint).await
    }

    /// Cancel a specific delivery order
    ///
    /// This endpoint cancels a specific delivery order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `order_id` - Order ID to cancel
    ///
    /// # Returns
    /// Cancelled delivery order details
    pub async fn cancel_delivery_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::delivery::Result<DeliveryOrder> {
        let endpoint = DELIVERY_ORDER_ENDPOINT
            .replace("{}", settle)
            .replace("{}", order_id);
        self.delete(&endpoint).await
    }
}
