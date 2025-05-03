use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub account_alias: String,
    pub asset: String,
    pub wallet_balance: String,
    pub cross_wallet_balance: String,
    pub cross_un_pnl: String,
    pub available_balance: String,
    pub max_withdraw_amount: String,
    pub margin_available: bool,
    pub update_time: i64,
}

impl BinanceCoinMPrivateRest {
    /// Get account balance
    /// 
    /// # Returns
    /// 
    /// Vector of account balances
    pub async fn get_balance(&self) -> BinanceCoinMResult<Vec<Balance>> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let query = format!("timestamp={}", timestamp);
        let signature = self.sign_request(&query);
        let url = format!("{}/dapi/v1/balance?{}&signature={}", 
            self.base_url, query, signature);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let balances: Vec<Balance> = response.json().await?;
        Ok(balances)
    }
} 