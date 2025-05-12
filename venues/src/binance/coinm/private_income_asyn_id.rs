use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeAsynIdQuery {
    pub download_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeAsynIdResponse {
    pub status: String,
    pub data: Option<Vec<Income>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Income {
    pub symbol: String,
    pub income_type: String,
    pub income: String,
    pub asset: String,
    pub info: String,
    pub time: i64,
    pub tran_id: i64,
    pub trade_id: Option<String>,
}

impl BinanceCoinMPrivateRest {
    /// Get income history by download ID
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters containing the download ID
    /// 
    /// # Returns
    /// 
    /// Status and data of the asynchronous income history request
    pub async fn get_income_history_asyn_id(&self, query: IncomeAsynIdQuery) -> BinanceCoinMResult<IncomeAsynIdResponse> {
        let mut params = Vec::with_capacity(2);
        params.push(format!("downloadId={}", query.download_id));
        let timestamp = chrono::Utc::now().timestamp_millis();
        params.push(format!("timestamp={}", timestamp));
        let mut query_str = params.join("&");
        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/income/asyn/id";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            Some(&query_str),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("incomeAsynId", 1)
        ).await?;
        Ok(response.data)
    }
} 