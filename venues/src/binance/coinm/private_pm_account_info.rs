use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct PMAccountInfo {
    pub asset: String,
    pub wallet_balance: String,
    pub unrealized_profit: String,
    pub margin_balance: String,
    pub maint_margin: String,
    pub initial_margin: String,
    pub position_initial_margin: String,
    pub open_order_initial_margin: String,
    pub max_withdraw_amount: String,
    pub cross_wallet_balance: String,
    pub cross_un_pnl: String,
    pub available_balance: String,
    pub update_time: i64,
}

impl BinanceCoinMPrivateRest {
    /// Get portfolio margin account information
    /// 
    /// # Returns
    /// 
    /// Vector of portfolio margin account information
    pub async fn get_pm_account_info(&self) -> BinanceCoinMResult<Vec<PMAccountInfo>> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let query = format!("timestamp={}", timestamp);
        let signature = self.sign_request(&query);
        let url = format!("{}/dapi/v1/pmAccountInfo?{}&signature={}", 
            self.base_url, query, signature);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let account_info: Vec<PMAccountInfo> = response.json().await?;
        Ok(account_info)
    }
} 