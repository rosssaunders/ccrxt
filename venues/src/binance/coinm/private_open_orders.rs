use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::api_errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrder {
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
    /// Get all open orders
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Optional symbol to filter orders
    /// 
    /// # Returns
    /// 
    /// List of open orders
    pub async fn get_open_orders(&self, symbol: Option<String>) -> BinanceCoinMResult<Vec<OpenOrder>> {
        let mut query = String::new();
        if let Some(sym) = symbol {
            query.push_str(&format!("symbol={}", sym));
        }
        let timestamp = chrono::Utc::now().timestamp_millis();
        if !query.is_empty() {
            query.push('&');
        }
        query.push_str(&format!("timestamp={}", timestamp));
        let signature = self.sign_request(&query);
        query.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/openOrders";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            Some(&query),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("openOrders", 1)
        ).await?;
        Ok(response.data)
    }
} 