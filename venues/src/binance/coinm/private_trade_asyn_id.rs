use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeAsynIdQuery {
    pub download_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeAsynIdResponse {
    pub status: String,
    pub data: Option<Vec<Trade>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub symbol: String,
    pub id: i64,
    pub order_id: i64,
    pub side: String,
    pub price: String,
    pub qty: String,
    pub realized_pnl: String,
    pub margin_asset: String,
    pub base_qty: String,
    pub commission: String,
    pub commission_asset: String,
    pub time: i64,
    pub position_side: String,
    pub buyer: bool,
    pub maker: bool,
}

impl BinanceCoinMPrivateRest {
    /// Get trade history by download ID
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters containing the download ID
    /// 
    /// # Returns
    /// 
    /// Status and data of the asynchronous trade request
    pub async fn get_trade_asyn_id(&self, query: TradeAsynIdQuery) -> BinanceCoinMResult<TradeAsynIdResponse> {
        let mut query_str = format!("downloadId={}", query.download_id);

        let timestamp = chrono::Utc::now().timestamp_millis();
        query_str.push_str(&format!("&timestamp={}", timestamp));

        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/trade/asyn/id?{}", self.base_url, query_str);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let result: TradeAsynIdResponse = response.json().await?;
        Ok(result)
    }
} 