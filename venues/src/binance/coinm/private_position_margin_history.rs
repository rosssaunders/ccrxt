use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionMarginHistory {
    pub amount: String,
    pub asset: String,
    pub symbol: String,
    pub time: i64,
    pub type_: i32,
    pub position_side: String,
}

impl BinanceCoinMPrivateRest {
    /// Get position margin history
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Symbol to get margin history for
    /// * `type_` - Optional type of margin modification (1: Add, 2: Reduce)
    /// * `start_time` - Optional start time in milliseconds
    /// * `end_time` - Optional end time in milliseconds
    /// * `limit` - Optional limit of records to return (default: 500, max: 1000)
    /// 
    /// # Returns
    /// 
    /// Vector of position margin history records
    pub async fn get_position_margin_history(
        &self,
        symbol: &str,
        type_: Option<i32>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<i32>,
    ) -> BinanceCoinMResult<Vec<PositionMarginHistory>> {
        let mut query = format!("symbol={}", symbol);

        if let Some(t) = type_ {
            query.push_str(&format!("&type={}", t));
        }
        if let Some(time) = start_time {
            query.push_str(&format!("&startTime={}", time));
        }
        if let Some(time) = end_time {
            query.push_str(&format!("&endTime={}", time));
        }
        if let Some(l) = limit {
            query.push_str(&format!("&limit={}", l));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        query.push_str(&format!("&timestamp={}", timestamp));

        let signature = self.sign_request(&query);
        query.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/positionMargin/history?{}", self.base_url, query);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let history: Vec<PositionMarginHistory> = response.json().await?;
        Ok(history)
    }
} 