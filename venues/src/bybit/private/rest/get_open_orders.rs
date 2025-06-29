use serde::{Deserialize, Serialize};

use crate::bybit::{enums::*, EndpointType, RestResult};

use super::client::RestClient;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOrdersRequest {
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
    pub open_only: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<OrderFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub order_id: String,
    pub order_link_id: String,
    pub block_trade_id: String,
    pub symbol: String,
    pub price: String,
    pub qty: String,
    pub side: Side,
    pub is_leverage: String,
    pub position_idx: i32,
    pub order_status: OrderStatus,
    pub cancel_type: String,
    pub reject_reason: String,
    pub avg_price: String,
    pub leaves_qty: String,
    pub leaves_value: String,
    pub cum_exec_qty: String,
    pub cum_exec_value: String,
    pub cum_exec_fee: String,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub stop_order_type: String,
    pub order_iv: String,
    pub trigger_price: String,
    pub take_profit: String,
    pub stop_loss: String,
    pub tp_trigger_by: String,
    pub sl_trigger_by: String,
    pub trigger_direction: i32,
    pub trigger_by: String,
    pub last_price_on_created: String,
    pub reduce_only: bool,
    pub close_on_trigger: bool,
    pub smp_type: String,
    pub smp_group: i32,
    pub smp_order_id: String,
    pub tpsl_mode: String,
    pub tp_limit_price: String,
    pub sl_limit_price: String,
    pub place_type: String,
    pub created_time: String,
    pub updated_time: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOrdersData {
    pub category: Category,
    pub next_page_cursor: String,
    pub list: Vec<OrderInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetOpenOrdersResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: GetOpenOrdersData,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    pub time: u64,
}

impl RestClient {
    /// Get open orders
    ///
    /// Query unfilled or partially filled orders in real-time, plus recent 500 closed orders.
    ///
    /// # Arguments
    /// * `request` - The get open orders request parameters
    ///
    /// # Returns
    /// A result containing the open orders response or an error
    pub async fn get_open_orders(
        &self,
        request: GetOpenOrdersRequest,
    ) -> RestResult<GetOpenOrdersResponse> {
        self.send_signed_request(
            "/v5/order/realtime",
            reqwest::Method::GET,
            request,
            EndpointType::Trade,
        )
        .await
    }
}

impl GetOpenOrdersRequest {
    /// Create a new get open orders request
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            open_only: None,
            order_filter: None,
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

    /// Set open only filter (0=open only, 1/2=include closed orders)
    pub fn open_only(mut self, open_only: i32) -> Self {
        self.open_only = Some(open_only);
        self
    }

    /// Set order filter
    pub fn order_filter(mut self, order_filter: OrderFilter) -> Self {
        self.order_filter = Some(order_filter);
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
    fn test_get_open_orders_request_builder() {
        let request = GetOpenOrdersRequest::new(Category::Linear)
            .symbol("BTCUSDT".to_string())
            .limit(10)
            .open_only(0);

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.limit, Some(10));
        assert_eq!(request.open_only, Some(0));
        assert!(request.base_coin.is_none());
        assert!(request.settle_coin.is_none());
    }

    #[test]
    fn test_get_open_orders_request_serialization() {
        let request = GetOpenOrdersRequest::new(Category::Spot)
            .symbol("ETHUSDT".to_string())
            .order_filter(OrderFilter::Order)
            .limit(20);

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"spot\""));
        assert!(json.contains("\"symbol\":\"ETHUSDT\""));
        assert!(json.contains("\"orderFilter\":\"Order\""));
        assert!(json.contains("\"limit\":20"));
        assert!(!json.contains("baseCoin")); // Should be skipped when None
    }
}