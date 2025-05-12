use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrderQuery {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOrder {
    pub symbol: String,
    pub order_id: i64,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub status: String,
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
    /// Get open order details
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters for the open order
    /// 
    /// # Returns
    /// 
    /// Open order details
    pub async fn get_open_order(&self, query: OpenOrderQuery) -> BinanceCoinMResult<OpenOrder> {
        let mut params = Vec::with_capacity(3);
        params.push(format!("symbol={}", query.symbol));
        if let Some(id) = query.order_id {
            params.push(format!("orderId={}", id));
        }
        if let Some(id) = query.client_order_id {
            params.push(format!("origClientOrderId={}", id));
        }
        let timestamp = chrono::Utc::now().timestamp_millis();
        params.push(format!("timestamp={}", timestamp));
        let mut query_str = params.join("&");
        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/openOrder";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            Some(&query_str),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("openOrder", 1)
        ).await?;
        Ok(response.data)
    }
} 