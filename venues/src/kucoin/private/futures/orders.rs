use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::kucoin::{
    OrderSide, OrderStatus, OrderType, ResponseHeaders, RestResponse, Result,
    StopType, TimeInForce,
};

/// Place order request for futures
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    /// Client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    /// Order side
    pub side: OrderSide,
    /// Symbol
    pub symbol: String,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Leverage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    /// Reduce only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Close order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_order: Option<bool>,
    /// Force hold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_hold: Option<bool>,
    /// Order size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Time in force
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Post only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    /// Hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// Iceberg
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<bool>,
    /// Visible size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_size: Option<i64>,
    /// Remark
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// Stop type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopType>,
    /// Stop price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price_type: Option<String>,
    /// Stop price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
}

/// Place order response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    /// Order ID
    pub order_id: String,
}

/// Cancel order request
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    pub order_id: String,
}

/// Cancel order response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    /// Cancelled order IDs
    pub cancelled_order_ids: Vec<String>,
}

/// Cancel all orders request
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelAllOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

/// Cancel all orders response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersResponse {
    /// Cancelled order IDs
    pub cancelled_order_ids: Vec<String>,
}

/// Get order request
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderRequest {
    pub order_id: String,
}

/// Order details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetails {
    /// Order ID
    pub id: String,
    /// Symbol
    pub symbol: String,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order side
    pub side: OrderSide,
    /// Order size
    pub size: String,
    /// Order price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Stop price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    /// Filled size
    pub deal_size: String,
    /// Deal funds
    pub deal_funds: String,
    /// Fee
    pub fee: String,
    /// Fee currency
    pub fee_currency: String,
    /// Stop type
    pub stop: Option<String>,
    /// Time in force
    pub time_in_force: Option<TimeInForce>,
    /// Post only
    pub post_only: bool,
    /// Hidden
    pub hidden: bool,
    /// Iceberg
    pub iceberg: bool,
    /// Leverage
    pub leverage: String,
    /// Force hold
    pub force_hold: bool,
    /// Close order
    pub close_order: bool,
    /// Visible size
    pub visible_size: Option<String>,
    /// Client order ID
    pub client_oid: Option<String>,
    /// Remark
    pub remark: Option<String>,
    /// Tags
    pub tags: Option<String>,
    /// Is active
    pub is_active: bool,
    /// Cancel exist stop
    pub cancel_exist_stop: bool,
    /// Created at
    pub created_at: i64,
    /// Updated at
    pub updated_at: i64,
    /// End at
    pub end_at: Option<i64>,
    /// Order time
    pub order_time: i64,
    /// Settlement currency
    pub settle_currency: String,
    /// Status
    pub status: OrderStatus,
    /// Filled value
    pub filled_value: String,
    /// Reduce only
    pub reduce_only: bool,
}

/// Get orders request
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// Paginated response for orders
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedOrdersResponse {
    /// Current page
    pub current_page: i32,
    /// Page size
    pub page_size: i32,
    /// Total number of items
    pub total_num: i32,
    /// Total number of pages
    pub total_page: i32,
    /// Items
    pub items: Vec<OrderDetails>,
}

impl super::RestClient {
    /// Place a new order
    pub async fn place_order(
        &self,
        request: PlaceOrderRequest,
    ) -> Result<(RestResponse<PlaceOrderResponse>, ResponseHeaders)> {
        let endpoint = "/api/v1/orders";
        self.post(endpoint, &request).await
    }

    /// Cancel an order
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<(RestResponse<CancelOrderResponse>, ResponseHeaders)> {
        let endpoint = format!("/api/v1/orders/{}", request.order_id);
        self.delete(&endpoint, None::<&()>).await
    }

    /// Cancel all orders
    pub async fn cancel_all_orders(
        &self,
        request: CancelAllOrdersRequest,
    ) -> Result<(RestResponse<CancelAllOrdersResponse>, ResponseHeaders)> {
        let endpoint = "/api/v1/orders";
        
        let mut params = HashMap::new();
        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        
        let params = if params.is_empty() { None } else { Some(params) };
        
        self.delete(endpoint, params).await
    }

    /// Get order details
    pub async fn get_order(
        &self,
        request: GetOrderRequest,
    ) -> Result<(RestResponse<OrderDetails>, ResponseHeaders)> {
        let endpoint = format!("/api/v1/orders/{}", request.order_id);
        self.get(&endpoint, None).await
    }

    /// Get orders with pagination
    pub async fn get_orders(
        &self,
        request: GetOrdersRequest,
    ) -> Result<(RestResponse<PaginatedOrdersResponse>, ResponseHeaders)> {
        let endpoint = "/api/v1/orders";
        
        let mut params = HashMap::new();
        
        if let Some(status) = request.status {
            params.insert("status".to_string(), serde_json::to_string(&status).unwrap().trim_matches('"').to_string());
        }
        if let Some(symbol) = request.symbol {
            params.insert("symbol".to_string(), symbol);
        }
        if let Some(side) = request.side {
            params.insert("side".to_string(), serde_json::to_string(&side).unwrap().trim_matches('"').to_string());
        }
        if let Some(order_type) = request.order_type {
            params.insert("type".to_string(), serde_json::to_string(&order_type).unwrap().trim_matches('"').to_string());
        }
        if let Some(start_at) = request.start_at {
            params.insert("startAt".to_string(), start_at.to_string());
        }
        if let Some(end_at) = request.end_at {
            params.insert("endAt".to_string(), end_at.to_string());
        }
        if let Some(current_page) = request.current_page {
            params.insert("currentPage".to_string(), current_page.to_string());
        }
        if let Some(page_size) = request.page_size {
            params.insert("pageSize".to_string(), page_size.to_string());
        }

        let params = if params.is_empty() { None } else { Some(params) };
        
        self.get(endpoint, params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_request_serialization() {
        let request = PlaceOrderRequest {
            client_oid: Some("test123".to_string()),
            side: OrderSide::Buy,
            symbol: "XBTUSDTM".to_string(),
            order_type: OrderType::Limit,
            leverage: Some("10".to_string()),
            reduce_only: Some(false),
            close_order: Some(false),
            force_hold: Some(false),
            size: Some(1),
            price: Some("50000".to_string()),
            time_in_force: Some(TimeInForce::GoodTillCanceled),
            post_only: Some(false),
            hidden: Some(false),
            iceberg: Some(false),
            visible_size: None,
            remark: Some("test order".to_string()),
            stop: None,
            stop_price_type: None,
            stop_price: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("XBTUSDTM"));
        assert!(json.contains("buy"));
        assert!(json.contains("limit"));
    }

    #[test]
    fn test_order_details_deserialization() {
        let json = r#"{
            "id": "5cdfc138b21023a909e5ad55",
            "symbol": "XBTUSDTM",
            "type": "limit",
            "side": "buy",
            "size": "1",
            "price": "50000",
            "dealSize": "0",
            "dealFunds": "0",
            "fee": "0",
            "feeCurrency": "USDT",
            "stop": null,
            "timeInForce": "GTC",
            "postOnly": false,
            "hidden": false,
            "iceberg": false,
            "leverage": "10",
            "forceHold": false,
            "closeOrder": false,
            "visibleSize": null,
            "clientOid": "test123",
            "remark": "test order",
            "tags": null,
            "isActive": true,
            "cancelExistStop": false,
            "createdAt": 1558167872000,
            "updatedAt": 1558167872000,
            "endAt": null,
            "orderTime": 1558167872000,
            "settleCurrency": "USDT",
            "status": "active",
            "filledValue": "0",
            "reduceOnly": false
        }"#;

        let order: OrderDetails = serde_json::from_str(json).unwrap();
        assert_eq!(order.symbol, "XBTUSDTM");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.leverage, "10");
    }
}
