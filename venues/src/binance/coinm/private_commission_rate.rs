use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::common::request::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommissionRate {
    pub symbol: String,
    pub maker_commission_rate: String,
    pub taker_commission_rate: String,
}

impl BinanceCoinMPrivateRest {
    /// Get commission rate information
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Symbol to get commission rate for
    /// 
    /// # Returns
    /// 
    /// Commission rate information
    pub async fn get_commission_rate(&self, symbol: &str) -> BinanceCoinMResult<CommissionRate> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let query = format!("symbol={}&timestamp={}", symbol, timestamp);
        let signature = self.sign_request(&query);
        let mut query_with_sig = query.clone();
        query_with_sig.push_str(&format!("&signature={}", signature));
        let response = send_request::<CommissionRate, _, _>(
            &self.client,
            &self.base_url,
            "/dapi/v1/commissionRate",
            reqwest::Method::GET,
            Some(&query_with_sig),
            Some(self.api_key.expose_secret()),
            || async { Ok(()) }, // TODO: Replace with actual rate limit check
        ).await?;
        Ok(response.data)
    }
} 