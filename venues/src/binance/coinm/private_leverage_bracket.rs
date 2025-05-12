use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bracket {
    pub bracket: i32,
    pub initial_leverage: i32,
    pub notional_cap: i64,
    pub notional_floor: i64,
    pub maint_margin_ratio: String,
    pub cum: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeverageBracket {
    pub symbol: String,
    pub brackets: Vec<Bracket>,
}

impl BinanceCoinMPrivateRest {
    /// Get leverage bracket information
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Optional symbol to filter brackets. If not provided, returns all brackets
    /// 
    /// # Returns
    /// 
    /// Vector of leverage bracket information
    pub async fn get_leverage_bracket(&self, symbol: Option<&str>) -> BinanceCoinMResult<Vec<LeverageBracket>> {
        let mut params = Vec::with_capacity(2);
        if let Some(s) = symbol {
            params.push(format!("symbol={}", s));
        }
        let timestamp = chrono::Utc::now().timestamp_millis();
        params.push(format!("timestamp={}", timestamp));
        let mut query = params.join("&");
        let signature = self.sign_request(&query);
        query.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/leverageBracket";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            Some(&query),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("leverageBracket", 1)
        ).await?;
        Ok(response.data)
    }
} 