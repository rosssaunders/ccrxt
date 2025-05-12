use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::api_errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderAsynQuery {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderAsynResponse {
    pub download_id: String,
}

impl BinanceCoinMPrivateRest {
    /// Get order details asynchronously
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters for the order
    /// 
    /// # Returns
    /// 
    /// Download ID for retrieving the results
    pub async fn get_order_asyn(&self, query: OrderAsynQuery) -> BinanceCoinMResult<OrderAsynResponse> {
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
        let endpoint = "/dapi/v1/order/asyn";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            Some(&query_str),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("orderAsyn", 1)
        ).await?;
        Ok(response.data)
    }
} 