use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bybit::{EndpointType, RestResult, enums::*};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_open_orders_request_builder() {
        let request = GetOpenOrdersRequest {
            category: Category::Linear,
            symbol: Some("BTCUSDT".to_string()),
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            open_only: Some(0),
            order_filter: None,
            limit: Some(10),
            cursor: None,
        };

        assert_eq!(request.category, Category::Linear);
        assert_eq!(request.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(request.limit, Some(10));
        assert_eq!(request.open_only, Some(0));
        assert!(request.base_coin.is_none());
        assert!(request.settle_coin.is_none());
    }

    #[test]
    fn test_get_open_orders_request_serialization() {
        let request = GetOpenOrdersRequest {
            category: Category::Spot,
            symbol: Some("ETHUSDT".to_string()),
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            open_only: None,
            order_filter: Some(OrderFilter::Order),
            limit: Some(20),
            cursor: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"category\":\"spot\""));
        assert!(json.contains("\"symbol\":\"ETHUSDT\""));
        assert!(json.contains("\"orderFilter\":\"Order\""));
        assert!(json.contains("\"limit\":20"));
        assert!(!json.contains("baseCoin")); // Should be skipped when None
    }
}
