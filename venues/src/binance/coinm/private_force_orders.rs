use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForceOrdersQuery {
    pub symbol: Option<String>,
    pub auto_close_type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForceOrder {
    pub order_id: i64,
    pub symbol: String,
    pub status: String,
    pub client_order_id: String,
    pub price: String,
    pub avg_price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub cum_qty: String,
    pub cum_quote: String,
    pub time_in_force: String,
    pub type_: String,
    pub reduce_only: bool,
    pub close_position: bool,
    pub side: String,
    pub position_side: String,
    pub stop_price: String,
    pub working_type: String,
    pub price_protect: bool,
    pub orig_type: String,
    pub time: i64,
    pub update_time: i64,
}

impl BinanceCoinMPrivateRest {
    /// Get all force orders
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters for filtering force orders
    /// 
    /// # Returns
    /// 
    /// List of force orders matching the query criteria
    pub async fn get_force_orders(&self, query: ForceOrdersQuery) -> BinanceCoinMResult<Vec<ForceOrder>> {
        let mut query_str = String::new();

        if let Some(sym) = query.symbol {
            query_str.push_str(&format!("symbol={}", sym));
        }
        if let Some(close_type) = query.auto_close_type {
            if !query_str.is_empty() {
                query_str.push('&');
            }
            query_str.push_str(&format!("autoCloseType={}", close_type));
        }
        if let Some(time) = query.start_time {
            if !query_str.is_empty() {
                query_str.push('&');
            }
            query_str.push_str(&format!("startTime={}", time));
        }
        if let Some(time) = query.end_time {
            if !query_str.is_empty() {
                query_str.push('&');
            }
            query_str.push_str(&format!("endTime={}", time));
        }
        if let Some(limit) = query.limit {
            if !query_str.is_empty() {
                query_str.push('&');
            }
            query_str.push_str(&format!("limit={}", limit));
        }

        let timestamp = chrono::Utc::now().timestamp_millis();
        if !query_str.is_empty() {
            query_str.push('&');
        }
        query_str.push_str(&format!("timestamp={}", timestamp));

        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));

        let url = format!("{}/dapi/v1/forceOrders?{}", self.base_url, query_str);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let orders: Vec<ForceOrder> = response.json().await?;
        Ok(orders)
    }
} 