use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request to create options order
#[derive(Debug, Clone, Serialize)]
pub struct CreateOptionsOrderRequest {
    /// Contract name
    pub contract: String,

    /// Order size
    pub size: String,

    /// Order price
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
}

/// Options order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsOrder {
    /// Order ID
    pub id: i64,

    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Underlying asset
    pub underlying: String,

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
    pub size: String,

    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Time in force
    pub tif: String,

    /// Left amount
    pub left: String,

    /// Filled total
    pub filled_total: String,

    /// Average fill price
    pub avg_deal_price: String,

    /// Order text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,

    /// Is reduce only
    pub is_reduce_only: bool,

    /// Is close order
    pub is_close: bool,

    /// Order fee
    pub fee: String,

    /// Rebate
    pub rebate: String,
}

/// Request parameters for listing options orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListOptionsOrdersRequest {
    /// Order status (open, finished)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Underlying filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,

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
    /// Create an options order
    ///
    /// This endpoint creates a new options order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The options order creation request parameters
    ///
    /// # Returns
    /// Created options order information
    pub async fn create_options_order(
        &self,
        request: CreateOptionsOrderRequest,
    ) -> crate::gateio::options::Result<OptionsOrder> {
        self.post("/options/orders", &request).await
    }

    /// List options orders
    ///
    /// This endpoint returns options orders for the authenticated user.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The options orders list request parameters
    ///
    /// # Returns
    /// List of options orders
    pub async fn list_options_orders(
        &self,
        params: ListOptionsOrdersRequest,
    ) -> crate::gateio::options::Result<Vec<OptionsOrder>> {
        self.get_with_query("/options/orders", &params).await
    }

    /// Get a specific options order
    ///
    /// This endpoint returns details for a specific options order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `order_id` - Order ID to retrieve
    ///
    /// # Returns
    /// Specific options order details
    pub async fn get_options_order(
        &self,
        order_id: &str,
    ) -> crate::gateio::options::Result<OptionsOrder> {
        let endpoint = format!("/options/orders/{}", order_id);
        self.get(&endpoint).await
    }

    /// Cancel all options orders
    ///
    /// This endpoint cancels all options orders for a specific underlying or contract.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `underlying` - Optional underlying asset filter
    /// * `contract` - Optional contract filter
    ///
    /// # Returns
    /// List of cancelled options orders
    pub async fn cancel_all_options_orders(
        &self,
        underlying: Option<&str>,
        contract: Option<&str>,
    ) -> crate::gateio::options::Result<Vec<OptionsOrder>> {
        let mut endpoint = "/options/orders".to_string();
        let mut query_params = Vec::new();

        if let Some(underlying) = underlying {
            query_params.push(format!("underlying={}", underlying));
        }
        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        self.delete(&endpoint).await
    }

    /// Cancel a specific options order
    ///
    /// This endpoint cancels a specific options order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `order_id` - Order ID to cancel
    ///
    /// # Returns
    /// Cancelled options order details
    pub async fn cancel_options_order(
        &self,
        order_id: &str,
    ) -> crate::gateio::options::Result<OptionsOrder> {
        let endpoint = format!("/options/orders/{}", order_id);
        self.delete(&endpoint).await
    }

    /// Countdown cancel orders
    ///
    /// Sets a countdown timer to cancel all open options orders after specified time.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `timeout` - Countdown time in seconds
    /// * `underlying` - Optional underlying asset filter
    ///
    /// # Returns
    /// Countdown cancel response
    pub async fn countdown_cancel_options_orders(
        &self,
        timeout: i32,
        underlying: Option<&str>,
    ) -> crate::gateio::options::Result<serde_json::Value> {
        let mut request = serde_json::json!({
            "timeout": timeout
        });

        if let Some(underlying) = underlying {
            request["underlying"] = serde_json::json!(underlying);
        }

        self.post("/options/countdown_cancel_all", &request).await
    }
}
