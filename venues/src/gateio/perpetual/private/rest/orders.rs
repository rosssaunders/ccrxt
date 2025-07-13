use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request to create futures order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFuturesOrderRequest {
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

/// Futures order information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesOrder {
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

    /// Order type
    #[serde(rename = "type")]
    pub order_type: Option<String>,

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

    /// Reject post only
    pub reject_post_only: bool,

    /// STP action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_act: Option<String>,

    /// Amendment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,
}

/// Request parameters for listing futures orders
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListFuturesOrdersRequest {
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

    /// Count total records
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_total: Option<i32>,
}

/// Request to create batch orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchOrdersRequest {
    /// Settlement currency
    pub settle: String,
    /// List of orders to create
    pub orders: Vec<CreateFuturesOrderRequest>,
}

/// Request to cancel batch orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchCancelOrdersRequest {
    /// Settlement currency
    pub settle: String,
    /// List of order IDs to cancel
    pub order_ids: Vec<String>,
}

/// Request to amend batch orders
#[derive(Debug, Clone, Serialize)]
pub struct BatchAmendOrdersRequest {
    /// Settlement currency
    pub settle: String,
    /// List of order amendments
    pub orders: Vec<AmendFuturesOrderRequest>,
}

/// Request to amend a futures order
#[derive(Debug, Clone, Serialize)]
pub struct AmendFuturesOrderRequest {
    /// Settlement currency
    pub settle: String,
    /// Order ID to amend
    pub order_id: String,
    /// New order size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// New order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Amendment text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amend_text: Option<String>,
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

/// Request for countdown cancel
#[derive(Debug, Clone, Serialize)]
pub struct CountdownCancelRequest {
    /// Settlement currency
    pub settle: String,
    /// Timeout in seconds
    pub timeout: i32,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
}

/// Request parameters for listing orders by time range
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListOrdersByTimeRangeRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    /// Start time (Unix timestamp in seconds)
    pub from: i64,
    /// End time (Unix timestamp in seconds)
    pub to: i64,
    /// Maximum number of records to return (1-1000, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl RestClient {
    /// Create a futures order
    ///
    /// This endpoint creates a new futures order.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.com/docs/developers/apiv4/#create-a-futures-order>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The order creation request parameters
    ///
    /// # Returns
    /// Created order details
    pub async fn create_futures_order(
        &self,
        request: CreateFuturesOrderRequest,
    ) -> crate::gateio::perpetual::Result<FuturesOrder> {
        let endpoint = format!("/futures/{}/orders", request.settle);
        self.post(&endpoint, &request).await
    }

    /// List futures orders
    ///
    /// This endpoint returns futures orders for the authenticated user.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-futures-orders>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - The order listing request parameters
    ///
    /// # Returns
    /// List of orders
    pub async fn list_futures_orders(
        &self,
        params: ListFuturesOrdersRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesOrder>> {
        let endpoint = format!("/futures/{}/orders", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a specific futures order
    ///
    /// This endpoint returns details for a specific futures order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `order_id` - Order ID
    ///
    /// # Returns
    /// Order details
    pub async fn get_futures_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::perpetual::Result<FuturesOrder> {
        let endpoint = format!("/futures/{}/orders/{}", settle, order_id);
        self.get(&endpoint).await
    }

    /// Cancel all futures orders
    ///
    /// This endpoint cancels all futures orders for a specific contract or all contracts.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Optional contract filter
    /// * `side` - Optional side filter (buy/sell)
    ///
    /// # Returns
    /// List of cancelled orders
    pub async fn cancel_all_futures_orders(
        &self,
        settle: &str,
        contract: Option<&str>,
        side: Option<&str>,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesOrder>> {
        let mut endpoint = format!("/futures/{}/orders", settle);

        let mut query_params = Vec::new();
        if let Some(contract) = contract {
            query_params.push(format!("contract={}", contract));
        }
        if let Some(side) = side {
            query_params.push(format!("side={}", side));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        self.delete(&endpoint).await
    }

    /// Cancel a specific futures order
    ///
    /// This endpoint cancels a specific futures order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `order_id` - Order ID to cancel
    ///
    /// # Returns
    /// Cancelled order details
    pub async fn cancel_futures_order(
        &self,
        settle: &str,
        order_id: &str,
    ) -> crate::gateio::perpetual::Result<FuturesOrder> {
        let endpoint = format!("/futures/{}/orders/{}", settle, order_id);
        self.delete(&endpoint).await
    }

    /// Create a batch of futures orders
    ///
    /// Creates multiple orders in a single request for improved efficiency.
    ///
    /// See: Gate.io API documentation
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
    ) -> crate::gateio::perpetual::Result<Vec<BatchOrderResult>> {
        let endpoint = format!("/futures/{}/batch_orders", request.settle);
        self.post(&endpoint, &request).await
    }

    /// Cancel a batch of futures orders
    ///
    /// Cancels multiple orders in a single request.
    ///
    /// See: Gate.io API documentation
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
    ) -> crate::gateio::perpetual::Result<Vec<BatchOrderResult>> {
        let endpoint = format!("/futures/{}/batch_orders", request.settle);
        self.delete_with_query(&endpoint, &request).await
    }

    /// Amend a batch of futures orders
    ///
    /// Modifies multiple orders in a single request.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The batch order amendment request parameters
    ///
    /// # Returns
    /// List of batch order results
    pub async fn amend_batch_futures_orders(
        &self,
        request: BatchAmendOrdersRequest,
    ) -> crate::gateio::perpetual::Result<Vec<BatchOrderResult>> {
        let endpoint = format!("/futures/{}/batch_orders", request.settle);
        self.put(&endpoint, &request).await
    }

    /// Countdown cancel all futures orders
    ///
    /// Sets a countdown timer to cancel all orders after a specified timeout.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The countdown cancel request parameters
    ///
    /// # Returns
    /// Empty response indicating success
    pub async fn countdown_cancel_all_futures_orders(
        &self,
        request: CountdownCancelRequest,
    ) -> crate::gateio::perpetual::Result<()> {
        let endpoint = format!("/futures/{}/countdown_cancel_all", request.settle);
        self.post::<serde_json::Value>(&endpoint, &request).await?;
        Ok(())
    }

    /// List futures orders by time range
    ///
    /// Returns orders within a specific time range for better performance.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - The time range request parameters
    ///
    /// # Returns
    /// List of orders within the specified time range
    pub async fn list_futures_orders_by_time_range(
        &self,
        params: ListOrdersByTimeRangeRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesOrder>> {
        let endpoint = format!("/futures/{}/orders_timerange", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Amend a futures order
    ///
    /// Modifies the price and/or size of an existing order.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The order amendment request parameters
    ///
    /// # Returns
    /// Amended order details
    pub async fn amend_futures_order(
        &self,
        request: AmendFuturesOrderRequest,
    ) -> crate::gateio::perpetual::Result<FuturesOrder> {
        let endpoint = format!("/futures/{}/orders/{}", request.settle, request.order_id);
        self.put(&endpoint, &request).await
    }
}
