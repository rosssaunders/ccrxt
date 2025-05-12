use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::api_errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use std::collections::BTreeMap;
use serde_urlencoded;
use super::request::append_timestamp_and_signature;

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeAsynQuery {
    pub symbol: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i64>,
    pub limit: Option<i32>,
}

impl TradeAsynQuery {
    /// Serializes the struct to a URL query string using serde_urlencoded.
    ///
    /// The order of parameters in the output string matches the order of fields in the struct.
    /// Fields with None values are omitted.
    pub fn to_query_string(&self) -> Result<String, serde_urlencoded::ser::Error> {
        serde_urlencoded::to_string(self)
    }
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
    pub async fn get_trade_asyn(&self, mut query: TradeAsynQuery) -> BinanceCoinMResult<TradeAsynResponse> {
        let timestamp = chrono::Utc::now().timestamp_millis();
        // Serialize the struct to a query string
        let query_str = query.to_query_string()
            .expect("Failed to serialize query params");
        let query_str = append_timestamp_and_signature(query_str, |qs| self.sign_request(qs))?;

        let api_key = self.get_api_key()?;
        let url = format!("{}/dapi/v1/trade/asyn?{}", self.base_url, query_str);

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", api_key.expose_secret())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinanceCoinMError::from_response(response).await);
        }

        let result: TradeAsynResponse = response.json().await?;
        Ok(result)
    }
} 