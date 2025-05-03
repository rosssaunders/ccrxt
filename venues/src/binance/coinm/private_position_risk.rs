use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionRisk {
    pub symbol: String,
    pub position_amt: String,
    pub entry_price: String,
    pub mark_price: String,
    pub un_realized_profit: String,
    pub liquidation_price: String,
    pub leverage: String,
    pub max_notional_value: String,
    pub margin_type: String,
    pub isolated_margin: String,
    pub is_auto_add_margin: String,
    pub position_side: String,
    pub notional: String,
    pub isolated_wallet: String,
    pub update_time: i64,
}

impl BinanceCoinMPrivateRest {
    /// Get current position risk information
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Optional symbol to filter positions. If not provided, returns all positions
    /// 
    /// # Returns
    /// 
    /// Vector of position risk information
    pub async fn get_position_risk(&self, symbol: Option<&str>) -> BinanceCoinMResult<Vec<PositionRisk>> {
        let mut query = String::new();
        if let Some(s) = symbol {
            query.push_str(&format!("symbol={}", s));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        query.push_str(&format!("&timestamp={}", timestamp));

        let signature = self.sign_request(&query);
        query.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/positionRisk?{}", self.base_url, query);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let positions: Vec<PositionRisk> = response.json().await?;
        Ok(positions)
    }
} 