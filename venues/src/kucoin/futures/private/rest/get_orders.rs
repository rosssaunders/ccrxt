use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::kucoin::spot::{OrderSide, OrderStatus, OrderType, ResponseHeaders, RestResponse, Result};

/// Endpoint URL for get orders
pub const GET_ORDERS_ENDPOINT: &str = "/api/v1/orders";

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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedOrdersResponse {
    pub current_page: i32,
    pub page_size: i32,
    pub total_num: i32,
    pub total_page: i32,
    pub items: Vec<super::OrderDetails>,
}

impl super::RestClient {
    /// Get orders with pagination
    pub async fn get_orders(
        &self,
        request: GetOrdersRequest,
    ) -> Result<(RestResponse<PaginatedOrdersResponse>, ResponseHeaders)> {
        let endpoint = GET_ORDERS_ENDPOINT;
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
            params.insert(
                "type".to_string(),
                serde_json::to_string(&order_type)
                    .unwrap()
                    .trim_matches('"')
                    .to_string(),
            );
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
        let params = if params.is_empty() {
            None
        } else {
            Some(params)
        };
        self.get(endpoint, params).await
    }
}
