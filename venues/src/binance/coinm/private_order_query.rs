use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::api_errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderQuery {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderQueryResponse {
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
    /// Query an order's status
    /// 
    /// # Arguments
    /// 
    /// * `query` - Order query parameters
    /// 
    /// # Returns
    /// 
    /// Order information
    pub async fn query_order(&self, query: OrderQuery) -> BinanceCoinMResult<OrderQueryResponse> {
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

        let url = format!("{}/dapi/v1/order?{}", self.base_url, query_str);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let order: OrderQueryResponse = response.json().await?;
        Ok(order)
    }
} 