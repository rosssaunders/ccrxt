use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForceOrdersQuery {
    pub symbol: Option<String>,
    pub auto_close_type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForceOrder {
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
    /// Get all force orders
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters for filtering force orders
    /// 
    /// # Returns
    /// 
    /// List of force orders matching the query criteria
    pub async fn get_force_orders(&self, query: ForceOrdersQuery) -> BinanceCoinMResult<Vec<ForceOrder>> {
        let mut params = Vec::with_capacity(5);
        if let Some(sym) = query.symbol {
            params.push(format!("symbol={}", sym));
        }
        if let Some(close_type) = query.auto_close_type {
            params.push(format!("autoCloseType={}", close_type));
        }
        if let Some(time) = query.start_time {
            params.push(format!("startTime={}", time));
        }
        if let Some(time) = query.end_time {
            params.push(format!("endTime={}", time));
        }
        if let Some(limit) = query.limit {
            params.push(format!("limit={}", limit));
        }
        let timestamp = chrono::Utc::now().timestamp_millis();
        params.push(format!("timestamp={}", timestamp));
        let mut query_str = params.join("&");
        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/forceOrders";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            Some(&query_str),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("forceOrders", 1)
        ).await?;
        Ok(response.data)
    }
} 