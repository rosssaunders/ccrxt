use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelAllOrdersResponse {
    pub code: i32,
    pub msg: String,
}

impl BinanceCoinMPrivateRest {
    /// Cancel all open orders
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Optional symbol to cancel orders for. If not provided, cancels all open orders.
    /// 
    /// # Returns
    /// 
    /// Response indicating success or failure of the cancellation
    pub async fn cancel_all_orders(&self, symbol: Option<String>) -> BinanceCoinMResult<CancelAllOrdersResponse> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let mut query_str = format!("timestamp={}", timestamp);

        if let Some(sym) = symbol {
            query_str = format!("symbol={}&{}", sym, query_str);
        }

        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/allOpenOrders?{}", self.base_url, query_str);

        let response = self.client
            .delete(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let result: CancelAllOrdersResponse = response.json().await?;
        Ok(result)
    }
} 