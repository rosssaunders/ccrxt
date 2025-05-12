use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::api_errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::common::request::send_request;
use std::collections::BTreeMap;
use serde_urlencoded;

#[derive(Debug, Serialize, Deserialize)]
pub struct AllOrdersQuery {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub order_id: i64,
    pub symbol: String,
    pub status: String,
    pub client_order_id: String,
    pub price: String,
    pub avg_price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub time_in_force: String,
    pub type_: String,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: String,
    pub position_side: String,
    pub stop_price: String,
    pub working_type: String,
    pub price_protect: bool,
    pub orig_type: String,
    pub time: i64,
    pub update_time: i64,
}

impl BinanceCoinMPrivateRest {
    /// Get all orders
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters for filtering orders
    /// 
    /// # Returns
    /// 
    /// List of orders matching the query criteria
    pub async fn get_all_orders(&self, query: AllOrdersQuery) -> BinanceCoinMResult<Vec<Order>> {
        let mut params = BTreeMap::new();
        params.insert("symbol", query.symbol.clone());
        if let Some(id) = query.order_id {
            params.insert("orderId", id.to_string());
        }
        if let Some(time) = query.start_time {
            params.insert("startTime", time.to_string());
        }
        if let Some(time) = query.end_time {
            params.insert("endTime", time.to_string());
        }
        if let Some(limit) = query.limit {
            params.insert("limit", limit.to_string());
        }
        let timestamp = chrono::Utc::now().timestamp_millis();
        params.insert("timestamp", timestamp.to_string());

        let mut query_str = serde_urlencoded::to_string(&params)
            .expect("Failed to serialize query params");
        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));

        let response = send_request::<Vec<Order>, _, _>(
            &self.client,
            &self.base_url,
            "/dapi/v1/allOrders",
            reqwest::Method::GET,
            Some(&query_str),
            Some(self.api_key.expose_secret()),
            || async { Ok(()) }, // TODO: Replace with actual rate limit check
        ).await?;

        Ok(response.data)
    }
} 