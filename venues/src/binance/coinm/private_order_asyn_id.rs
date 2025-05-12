use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderAsynIdQuery {
    pub download_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderAsynIdResponse {
    pub status: String,
    pub data: Option<Order>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
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
    /// Get order details by download ID
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters containing the download ID
    /// 
    /// # Returns
    /// 
    /// Status and data of the asynchronous order request
    pub async fn get_order_asyn_id(&self, query: OrderAsynIdQuery) -> BinanceCoinMResult<OrderAsynIdResponse> {
        let mut query_str = format!("downloadId={}", query.download_id);
        let timestamp = chrono::Utc::now().timestamp_millis();
        query_str.push_str(&format!("&timestamp={}", timestamp));
        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/order/asyn/id";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            Some(&query_str),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("orderAsynId", 1)
        ).await?;
        Ok(response.data)
    }
} 