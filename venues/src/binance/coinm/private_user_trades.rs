use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTrade {
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
    /// Get user trades
    /// 
    /// # Arguments
    /// 
    /// * `symbol` - Symbol to get trades for
    /// * `order_id` - Optional order ID to filter trades
    /// * `start_time` - Optional start time in milliseconds
    /// * `end_time` - Optional end time in milliseconds
    /// * `from_id` - Optional trade ID to fetch from
    /// * `limit` - Optional limit of trades to return (default: 500, max: 1000)
    /// 
    /// # Returns
    /// 
    /// Vector of user trades
    pub async fn get_user_trades(
        &self,
        symbol: &str,
        order_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        from_id: Option<i64>,
        limit: Option<i32>,
    ) -> BinanceCoinMResult<Vec<UserTrade>> {
        let mut query = format!("symbol={}", symbol);

        if let Some(id) = order_id {
            query.push_str(&format!("&orderId={}", id));
        }
        if let Some(time) = start_time {
            query.push_str(&format!("&startTime={}", time));
        }
        if let Some(time) = end_time {
            query.push_str(&format!("&endTime={}", time));
        }
        if let Some(id) = from_id {
            query.push_str(&format!("&fromId={}", id));
        }
        if let Some(l) = limit {
            query.push_str(&format!("&limit={}", l));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        query.push_str(&format!("&timestamp={}", timestamp));

        let signature = self.sign_request(&query);
        query.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/userTrades?{}", self.base_url, query);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let trades: Vec<UserTrade> = response.json().await?;
        Ok(trades)
    }
} 