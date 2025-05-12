use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::api_errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::common::request::send_request;

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

        let response = send_request::<CancelAllOrdersResponse, _, _>(
            &self.client,
            &self.base_url,
            "/dapi/v1/allOpenOrders",
            reqwest::Method::DELETE,
            Some(&query_str),
            Some(self.api_key.expose_secret()),
            || async { Ok(()) }, // TODO: Replace with actual rate limit check
        ).await?;
        Ok(response.data)
    }
} 