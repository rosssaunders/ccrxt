use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct LeverageResponse {
    pub leverage: i32,
    pub max_notional_value: String,
    pub symbol: String,
}

impl BinanceCoinMPrivateRest {
    /// Change leverage
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Symbol to change leverage for
    /// * `leverage` - New leverage value (1-125)
    /// 
    /// # Returns
    /// 
    /// Updated leverage information
    pub async fn change_leverage(&self, symbol: &str, leverage: i32) -> BinanceCoinMResult<LeverageResponse> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let query = format!(
            "symbol={}&leverage={}&timestamp={}",
            symbol,
            leverage,
            timestamp
        );
        let signature = self.sign_request(&query);
        let mut query_with_sig = query.clone();
        query_with_sig.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/leverage";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::POST,
            Some(&query_with_sig),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("leverage", 1)
        ).await?;
        Ok(response.data)
    }
} 