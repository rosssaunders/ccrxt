use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

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
        let url = format!("{}/dapi/v1/leverage?{}&signature={}", 
            self.base_url, query, signature);

        let response = self.client
            .post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let leverage_info: LeverageResponse = response.json().await?;
        Ok(leverage_info)
    }
} 