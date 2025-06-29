use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;
use super::get_open_orders::OrderInfo;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryRequest {
    pub category: Category,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_status: Option<OrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryData {
    pub category: Category,
    pub list: Vec<OrderInfo>,
    pub next_page_cursor: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetOrderHistoryResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetOrderHistoryData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get order history
    ///
    /// Query order history with various time range limitations based on order status.
    ///
    /// # Arguments
    /// * `request` - The get order history request parameters
    ///
    /// # Returns
    /// A result containing the order history response or an error
    pub async fn get_order_history(
        &self,
        request: GetOrderHistoryRequest,
    ) -> RestResult<GetOrderHistoryResponse> {
        self.send_signed_request(
            "/v5/order/history",
            reqwest::Method::GET,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

impl GetOrderHistoryRequest {
    /// Create a new get order history request
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            order_filter: None,
            order_status: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Filter by symbol
    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Filter by base coin
    pub fn base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Filter by settle coin
    pub fn settle_coin(mut self, settle_coin: String) -> Self {
        self.settle_coin = Some(settle_coin);
        self
    }

    /// Filter by order ID
    pub fn order_id(mut self, order_id: String) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /// Filter by order link ID
    pub fn order_link_id(mut self, order_link_id: String) -> Self {
        self.order_link_id = Some(order_link_id);
        self
    }

    /// Set order filter
    pub fn order_filter(mut self, order_filter: OrderFilter) -> Self {
        self.order_filter = Some(order_filter);
        self
    }

    /// Filter by order status
    pub fn order_status(mut self, order_status: OrderStatus) -> Self {
        self.order_status = Some(order_status);
        self
    }

    /// Set start time (timestamp in milliseconds)
    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Set end time (timestamp in milliseconds)
    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Set page limit (1-50, default 20)
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set pagination cursor
    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_order_history_request_builder() {
        let request = GetOrderHistoryRequest::new(Category::Linear)
            .symbol("BTCUSDT".to_string())
            .order_status(OrderStatus::Filled)
            .limit(10)
            .start_time(1640995200000)
            .end_time(1641081600000);

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.order_status, Some(OrderStatus::Filled));
        assert_eq!(request.limit, Some(10));
        assert_eq!(request.start_time, Some(1640995200000));
        assert_eq!(request.end_time, Some(1641081600000));
    }

    #[test]
    fn test_get_order_history_request_serialization() {
        let request = GetOrderHistoryRequest::new(Category::Spot)
            .base_coin("BTC".to_string())
            .order_filter(OrderFilter::Order)
            .limit(20);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"spot\""));
        assert!(json.contains("\"baseCoin\":\"BTC\""));
        assert!(json.contains("\"orderFilter\":\"Order\""));
        assert!(json.contains("\"limit\":20"));
        assert!(!json.contains("symbol")); // Should be skipped when None
    }
}