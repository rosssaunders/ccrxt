use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{OrderSide, OrderStatus, ResponseHeaders, RestResponse, Result, TradeSide};

use super::RestClient;

/// Request for getting order list
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrdersRequest {
    /// Order status filter (optional): active, done
    pub status: Option<OrderStatus>,
    /// Trading symbol filter (optional)
    pub symbol: Option<String>,
    /// Order side filter (optional)
    pub side: Option<OrderSide>,
    /// Order type filter (optional)
    #[serde(rename = "type")]
    pub order_type: Option<String>,
    /// Start time filter (optional, milliseconds)
    #[serde(rename = "startAt")]
    pub start_time: Option<i64>,
    /// End time filter (optional, milliseconds)
    #[serde(rename = "endAt")]
    pub end_time: Option<i64>,
}

/// Order information
#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    /// Order ID
    pub id: String,
    /// Trading symbol
    pub symbol: String,
    /// Operation type (DEAL)
    #[serde(rename = "opType")]
    pub operation_type: String,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: String,
    /// Order side
    pub side: OrderSide,
    /// Order amount
    pub amount: String,
    /// Order funds
    pub funds: String,
    /// Dealt amount
    #[serde(rename = "dealAmount")]
    pub deal_amount: String,
    /// Dealt funds
    #[serde(rename = "dealFunds")]
    pub deal_funds: String,
    /// Fee
    pub fee: String,
    /// Fee currency
    #[serde(rename = "feeCurrency")]
    pub fee_currency: String,
    /// Self-trade prevention
    pub stp: String,
    /// Stop type
    pub stop: String,
    /// Stop triggered flag
    #[serde(rename = "stopTriggered")]
    pub stop_triggered: bool,
    /// Stop price
    #[serde(rename = "stopPrice")]
    pub stop_price: String,
    /// Time in force
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    /// Post only flag
    #[serde(rename = "postOnly")]
    pub post_only: bool,
    /// Hidden order flag
    pub hidden: bool,
    /// Iceberg order flag
    pub iceberg: bool,
    /// Visible size for iceberg orders
    #[serde(rename = "visibleSize")]
    pub visible_size: String,
    /// Cancel after time
    #[serde(rename = "cancelAfter")]
    pub cancel_after: i64,
    /// Channel
    pub channel: String,
    /// Client order ID
    #[serde(rename = "clientOid")]
    pub client_order_id: String,
    /// Remark
    pub remark: String,
    /// Tags
    pub tags: String,
    /// Is active flag
    #[serde(rename = "isActive")]
    pub is_active: bool,
    /// Cancel exist flag
    #[serde(rename = "cancelExist")]
    pub cancel_exist: bool,
    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    /// Trade type
    #[serde(rename = "tradeType")]
    pub trade_type: String,
    /// Price
    pub price: String,
    /// Size
    pub size: String,
}

/// Response wrapper for order list
#[derive(Debug, Clone, Deserialize)]
pub struct OrdersResponse {
    /// Current page
    #[serde(rename = "currentPage")]
    pub current_page: i32,
    /// Page size
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    /// Total number of records
    #[serde(rename = "totalNum")]
    pub total_num: i32,
    /// Total pages
    #[serde(rename = "totalPage")]
    pub total_page: i32,
    /// Order items
    pub items: Vec<Order>,
}

/// Request for getting order details
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderRequest {
    /// Order ID
    pub order_id: String,
}

/// Request for getting fills (trade history)
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFillsRequest {
    /// Trading symbol filter (optional)
    pub symbol: Option<String>,
    /// Order ID filter (optional)
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,
    /// Trade side filter (optional)
    pub side: Option<TradeSide>,
    /// Trade type filter (optional)
    #[serde(rename = "type")]
    pub trade_type: Option<String>,
    /// Start time filter (optional, milliseconds)
    #[serde(rename = "startAt")]
    pub start_time: Option<i64>,
    /// End time filter (optional, milliseconds)
    #[serde(rename = "endAt")]
    pub end_time: Option<i64>,
}

/// Fill (trade) information
#[derive(Debug, Clone, Deserialize)]
pub struct Fill {
    /// Trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    /// Symbol
    pub symbol: String,
    /// Counter order ID
    #[serde(rename = "counterOrderId")]
    pub counter_order_id: String,
    /// Order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Side
    pub side: TradeSide,
    /// Liquidity (taker/maker)
    pub liquidity: String,
    /// Force taker flag
    #[serde(rename = "forceTaker")]
    pub force_taker: bool,
    /// Price
    pub price: String,
    /// Size
    pub size: String,
    /// Funds
    pub funds: String,
    /// Fee
    pub fee: String,
    /// Fee rate
    #[serde(rename = "feeRate")]
    pub fee_rate: String,
    /// Fee currency
    #[serde(rename = "feeCurrency")]
    pub fee_currency: String,
    /// Stop type
    pub stop: String,
    /// Trade type
    #[serde(rename = "type")]
    pub trade_type: String,
    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    /// Trade time
    #[serde(rename = "tradeTime")]
    pub trade_time: i64,
}

/// Response wrapper for fills
#[derive(Debug, Clone, Deserialize)]
pub struct FillsResponse {
    /// Current page
    #[serde(rename = "currentPage")]
    pub current_page: i32,
    /// Page size
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    /// Total number of records
    #[serde(rename = "totalNum")]
    pub total_num: i32,
    /// Total pages
    #[serde(rename = "totalPage")]
    pub total_page: i32,
    /// Fill items
    pub items: Vec<Fill>,
}

/// Request for getting recent fills
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRecentFillsRequest {
    /// Symbol filter (optional)
    pub symbol: Option<String>,
}

/// Request for getting stop orders
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetStopOrdersRequest {
    /// Trading symbol filter (optional)
    pub symbol: Option<String>,
    /// Order IDs filter (optional, comma-separated)
    #[serde(rename = "orderIds")]
    pub order_ids: Option<String>,
    /// Page number (optional, default 1)
    #[serde(rename = "currentPage")]
    pub current_page: Option<i32>,
    /// Page size (optional, default 50, max 500)
    #[serde(rename = "pageSize")]
    pub page_size: Option<i32>,
    /// Start time filter (optional, milliseconds)
    #[serde(rename = "startAt")]
    pub start_time: Option<i64>,
    /// End time filter (optional, milliseconds)
    #[serde(rename = "endAt")]
    pub end_time: Option<i64>,
}

impl RestClient {
    /// Get list of orders
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetOrdersRequest};
    /// use kucoin::OrderStatus;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetOrdersRequest {
    ///         status: Some(OrderStatus::Active),
    ///         symbol: Some("BTC-USDT".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let (orders, _headers) = client.get_orders(request).await?;
    ///     println!("Found {} orders", orders.items.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_orders(
        &self,
        request: GetOrdersRequest,
    ) -> Result<(OrdersResponse, ResponseHeaders)> {
        let mut params = HashMap::new();

        if let Some(status) = request.status {
            params.insert(
                "status".to_string(),
                serde_json::to_string(&status)
                    .unwrap()
                    .trim_matches('"')
                    .to_string(),
            );
        }
        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        if let Some(side) = request.side {
            params.insert(
                "side".to_string(),
                serde_json::to_string(&side)
                    .unwrap()
                    .trim_matches('"')
                    .to_string(),
            );
        }
        if let Some(order_type) = request.order_type {
            params.insert("type".to_string(), order_type);
        }
        if let Some(start_time) = request.start_time {
            params.insert("startAt".to_string(), start_time.to_string());
        }
        if let Some(end_time) = request.end_time {
            params.insert("endAt".to_string(), end_time.to_string());
        }

        let (response, headers): (RestResponse<OrdersResponse>, ResponseHeaders) =
            self.get("/api/v1/orders", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Get order details by order ID
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetOrderRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetOrderRequest {
    ///         order_id: "order_id_here".to_string(),
    ///     };
    ///     let (order, _headers) = client.get_order(request).await?;
    ///     println!("Order: {} - Status: {}", order.id, order.is_active);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_order(&self, request: GetOrderRequest) -> Result<(Order, ResponseHeaders)> {
        let endpoint = format!("/api/v1/orders/{}", request.order_id);

        let (response, headers): (RestResponse<Order>, ResponseHeaders) =
            self.get(&endpoint, None).await?;

        Ok((response.data, headers))
    }

    /// Get fills (trade history)
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetFillsRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetFillsRequest {
    ///         symbol: Some("BTC-USDT".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let (fills, _headers) = client.get_fills(request).await?;
    ///     println!("Found {} fills", fills.items.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_fills(
        &self,
        request: GetFillsRequest,
    ) -> Result<(FillsResponse, ResponseHeaders)> {
        let mut params = HashMap::new();

        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        if let Some(order_id) = request.order_id {
            params.insert("orderId".to_string(), order_id);
        }
        if let Some(side) = request.side {
            params.insert(
                "side".to_string(),
                serde_json::to_string(&side)
                    .unwrap()
                    .trim_matches('"')
                    .to_string(),
            );
        }
        if let Some(trade_type) = request.trade_type {
            params.insert("type".to_string(), trade_type);
        }
        if let Some(start_time) = request.start_time {
            params.insert("startAt".to_string(), start_time.to_string());
        }
        if let Some(end_time) = request.end_time {
            params.insert("endAt".to_string(), end_time.to_string());
        }

        let (response, headers): (RestResponse<FillsResponse>, ResponseHeaders) =
            self.get("/api/v1/fills", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Get recent fills
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetRecentFillsRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetRecentFillsRequest::default();
    ///     let (fills, _headers) = client.get_recent_fills(request).await?;
    ///     println!("Found {} recent fills", fills.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_recent_fills(
        &self,
        request: GetRecentFillsRequest,
    ) -> Result<(Vec<Fill>, ResponseHeaders)> {
        let mut params = HashMap::new();

        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }

        let (response, headers): (RestResponse<Vec<Fill>>, ResponseHeaders) =
            self.get("/api/v1/limit/fills", Some(params)).await?;

        Ok((response.data, headers))
    }

    /// Get stop orders
    ///
    /// # Example
    /// ```rust,no_run
    /// use kucoin::private::{RestClient, GetStopOrdersRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = RestClient::new_with_credentials("api_key", "api_secret", "api_passphrase");
    ///     let request = GetStopOrdersRequest {
    ///         symbol: Some("BTC-USDT".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let (orders, _headers) = client.get_stop_orders(request).await?;
    ///     println!("Found {} stop orders", orders.items.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_stop_orders(
        &self,
        request: GetStopOrdersRequest,
    ) -> Result<(OrdersResponse, ResponseHeaders)> {
        let mut params = HashMap::new();

        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        if let Some(order_ids) = request.order_ids {
            params.insert("orderIds".to_string(), order_ids);
        }
        if let Some(current_page) = request.current_page {
            params.insert("currentPage".to_string(), current_page.to_string());
        }
        if let Some(page_size) = request.page_size {
            params.insert("pageSize".to_string(), page_size.to_string());
        }
        if let Some(start_time) = request.start_time {
            params.insert("startAt".to_string(), start_time.to_string());
        }
        if let Some(end_time) = request.end_time {
            params.insert("endAt".to_string(), end_time.to_string());
        }

        let (response, headers): (RestResponse<OrdersResponse>, ResponseHeaders) =
            self.get("/api/v1/stop-order", Some(params)).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kucoin::OrderStatus;

    #[test]
    fn test_orders_request_creation() {
        let request = GetOrdersRequest {
            status: Some(OrderStatus::Active),
            symbol: Some("BTC-USDT".to_string()),
            ..Default::default()
        };
        assert_eq!(request.status, Some(OrderStatus::Active));
        assert_eq!(request.symbol, Some("BTC-USDT".to_string()));
    }

    #[test]
    fn test_fills_request_default() {
        let request = GetFillsRequest::default();
        assert!(request.symbol.is_none());
        assert!(request.order_id.is_none());
    }

    #[test]
    fn test_order_request_creation() {
        let request = GetOrderRequest {
            order_id: "test_order_id".to_string(),
        };
        assert_eq!(request.order_id, "test_order_id");
    }
}
