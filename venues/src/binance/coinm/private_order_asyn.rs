use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

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
        let mut query_str = format!("symbol={}", query.symbol);

        if let Some(id) = query.order_id {
            query_str.push_str(&format!("&orderId={}", id));
        }
        if let Some(id) = query.client_order_id {
            query_str.push_str(&format!("&origClientOrderId={}", id));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        query_str.push_str(&format!("&timestamp={}", timestamp));

        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/order/asyn?{}", self.base_url, query_str);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let result: OrderAsynResponse = response.json().await?;
        Ok(result)
    }
} 