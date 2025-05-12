use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::api_errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::common::request::send_request;

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
        let mut query_with_sig = query.clone();
        query_with_sig.push_str(&format!("&signature={}", signature));
        let response = send_request::<Vec<Balance>, _, _>(
            &self.client,
            &self.base_url,
            "/dapi/v1/balance",
            reqwest::Method::GET,
            Some(&query_with_sig),
            Some(self.api_key.expose_secret()),
            || async { Ok(()) }, // TODO: Replace with actual rate limit check
        ).await?;
        Ok(response.data)
    }
} 