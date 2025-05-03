use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub enum PositionMarginType {
    #[serde(rename = "1")]
    Add,
    #[serde(rename = "2")]
    Reduce,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionMarginResponse {
    pub amount: String,
    pub code: i32,
    pub msg: String,
    pub type_: i32,
}

impl BinanceCoinMPrivateRest {
    /// Modify isolated position margin
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Symbol to modify margin for
    /// * `amount` - Amount to add or reduce
    /// * `type_` - Type of margin modification (1: Add, 2: Reduce)
    /// * `position_side` - Optional position side (BOTH, LONG, SHORT)
    /// 
    /// # Returns
    /// 
    /// Position margin modification response
    pub async fn modify_position_margin(
        &self,
        symbol: &str,
        amount: &str,
        type_: PositionMarginType,
        position_side: Option<&str>,
    ) -> BinanceCoinMResult<PositionMarginResponse> {
        let mut query = format!(
            "symbol={}&amount={}&type={}",
            symbol,
            amount,
            serde_json::to_string(&type_)?,
        );

        if let Some(side) = position_side {
            query.push_str(&format!("&positionSide={}", side));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        query.push_str(&format!("&timestamp={}", timestamp));

        let signature = self.sign_request(&query);
        query.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/positionMargin?{}", self.base_url, query);

        let response = self.client
            .post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let margin_response: PositionMarginResponse = response.json().await?;
        Ok(margin_response)
    }
} 