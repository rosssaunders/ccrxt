use serde::{Deserialize, Serialize};
use super::private_rest::BinanceCoinMPrivateRest;
use super::api_errors::BinanceCoinMError;
use super::types::BinanceCoinMResult;
use super::utils::send_request;

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeAsynQuery {
    pub symbol: Option<String>,
    pub income_type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeAsynResponse {
    pub download_id: String,
}

impl BinanceCoinMPrivateRest {
    /// Get income history asynchronously
    /// 
    /// # Arguments
    /// 
    /// * `query` - Query parameters for filtering income history
    /// 
    /// # Returns
    /// 
    /// Download ID for retrieving the results
    pub async fn get_income_history_asyn(&self, query: IncomeAsynQuery) -> BinanceCoinMResult<IncomeAsynResponse> {
        let mut params = Vec::with_capacity(5);
        if let Some(sym) = query.symbol {
            params.push(format!("symbol={}", sym));
        }
        if let Some(income_type) = query.income_type {
            params.push(format!("incomeType={}", income_type));
        }
        if let Some(time) = query.start_time {
            params.push(format!("startTime={}", time));
        }
        if let Some(time) = query.end_time {
            params.push(format!("endTime={}", time));
        }
        if let Some(limit) = query.limit {
            params.push(format!("limit={}", limit));
        }
        let timestamp = chrono::Utc::now().timestamp_millis();
        params.push(format!("timestamp={}", timestamp));
        let mut query_str = params.join("&");
        let signature = self.sign_request(&query_str);
        query_str.push_str(&format!("&signature={}", signature));
        let endpoint = "/dapi/v1/income/asyn";
        let response = send_request(
            &self.client,
            &self.base_url,
            endpoint,
            reqwest::Method::GET,
            Some(&query_str),
            Some(self.api_key.expose_secret()),
            || self.rate_limiter.check_weight_limit("incomeAsyn", 1)
        ).await?;
        Ok(response.data)
    }
} 