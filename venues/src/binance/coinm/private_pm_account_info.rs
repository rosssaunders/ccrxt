use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::api_errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

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
        let mut query_with_sig = query.clone();
        query_with_sig.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/pmAccountInfo";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            Some(&query_with_sig),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("pmAccountInfo", 1)
        ).await?;
        Ok(response.data)
    }
} 