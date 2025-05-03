use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeAsynQuery {
    pub symbol: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i64>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeAsynResponse {
    pub download_id: String,
}

impl BinanceCoinMPrivateRest {
    /// Get trade history asynchronously
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters for the trade history
    /// 
    /// # Returns
    /// 
    /// Download ID for retrieving the results
    pub async fn get_trade_asyn(&self, query: TradeAsynQuery) -> BinanceCoinMResult<TradeAsynResponse> {
        let mut query_str = format!("symbol={}", query.symbol);

        if let Some(time) = query.start_time {
            query_str.push_str(&format!("&startTime={}", time));
        }
        if let Some(time) = query.end_time {
            query_str.push_str(&format!("&endTime={}", time));
        }
        if let Some(id) = query.from_id {
            query_str.push_str(&format!("&fromId={}", id));
        }
        if let Some(limit) = query.limit {
            query_str.push_str(&format!("&limit={}", limit));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        query_str.push_str(&format!("&timestamp={}", timestamp));

        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/trade/asyn?{}", self.base_url, query_str);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let result: TradeAsynResponse = response.json().await?;
        Ok(result)
    }
} 