use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

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
        let url = format!("{}/dapi/v1/commissionRate?{}&signature={}", 
            self.base_url, query, signature);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let commission_rate: CommissionRate = response.json().await?;
        Ok(commission_rate)
    }
} 