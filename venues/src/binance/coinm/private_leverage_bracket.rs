use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

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
        let mut query = String::new();
        if let Some(s) = symbol {
            query.push_str(&format!("symbol={}", s));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        query.push_str(&format!("&timestamp={}", timestamp));

        let signature = self.sign_request(&query);
        query.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/leverageBracket?{}", self.base_url, query);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let brackets: Vec<LeverageBracket> = response.json().await?;
        Ok(brackets)
    }
} 