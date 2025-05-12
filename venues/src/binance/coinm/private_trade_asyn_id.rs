use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

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
        let mut params = Vec::with_capacity(2);
        params.push(format!("downloadId={}", query.download_id));
        let timestamp = chrono::Utc::now().timestamp_millis();
        params.push(format!("timestamp={}", timestamp));
        let mut query_str = params.join("&");
        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/trade/asyn/id";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            Some(&query_str),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("tradeAsynId", 1)
        ).await?;
        Ok(response.data)
    }
} 